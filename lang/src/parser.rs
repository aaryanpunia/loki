use std::{fmt::Display, iter::Peekable, ops::Range};

use logos::{Lexer, Logos, Span, SpannedIter};

use crate::tokens::Token;

pub trait Number {}

pub enum Decl {
    Store(Store),
    Operation(Operation),
}

pub struct Store {
    pub name: Identifier,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: Identifier,
    pub type_: Type,
}

pub enum Type {
    String,
    Int32,
    Int64,
    Uint32,
    Uint64,
    Boolean,
    Array(Array),
}

pub struct Array {
    type_: Box<Type>,
}

pub struct Operation {
    pub type_: OperationType,
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Statement>,
}

pub enum OperationType {
    Query,
    Modify,
    Delete,
}

pub struct Parameter {
    pub name: Identifier,
    pub type_: Type,
}

pub enum Statement {
    Expression(Expression),
    Assignment(Assignment),
}

pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    MethodChain(MethodChain),
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
}

pub struct Identifier {
    pub name: String,
    pub span: Range<usize>,
}

pub enum Literal {
    BooleanLiteral(BooleanLiteral),
    StringLiteral(StringLiteral),
    NumberLiteral(Box<NumberLiteral>),
}

pub enum BooleanLiteral {
    True,
    False,
}

pub struct StringLiteral {
    pub string: String,
}

pub struct NumberLiteral {
    pub number: dyn Number,
}

pub struct UnaryExpr {
    pub operator: Operator,
    pub operand: Operand,
}

pub struct BinaryExpr {
    pub operand1: Operand,
    pub operator: Operator,
    pub operand2: Operand,
}

pub enum Operand {
    Literal(Literal),
    Identifier(Identifier),
}

pub enum Operator {
    Equal,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    And,
    Or,
}

pub struct MethodChain {
    pub next: Option<Box<MethodChain>>,
    pub identifier: Identifier,
}

pub struct Assignment {
    pub identifier: Identifier,
    pub expression: Expression,
}

pub struct Parser<'a> {
    lexer: Peekable<SpannedIter<'a, Token>>,
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedEndOfInput,
    UnexpectedToken,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedEndOfInput => write!(f, "End of input"),
            ParserError::UnexpectedToken => write!(f, "Unexpected token"),
        }
    }
}

impl std::error::Error for ParserError {}

impl<'source> Parser<'source> {
    pub fn new(source: &'source <Token as Logos>::Source) -> Self {
        let lexer = Token::lexer(source).spanned().peekable();
        Self { lexer }
    }

    pub fn peek(&mut self) -> Option<(&Token, &Range<usize>)> {
        match self.lexer.peek() {
            Some(res_span_tuple) => match res_span_tuple {
                (Ok(token), span) => Some((token, span)),
                (Err(_), _) => None,
            },
            None => None,
        }
    }

    pub fn advance(&mut self) -> Result<(), ParserError> {
        match self.lexer.next() {
            Some(_) => Ok(()),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }

    pub fn parse_decl(&mut self) -> Result<Decl, ParserError> {
        match self.peek() {
            Some((&Token::Store, _)) => {
                let store = self.parse_store()?;
                Ok(Decl::Store(store))
            }
            Some((_, _)) => {
                let operation = self.parse_operation()?;
                Ok(Decl::Operation(operation))
            }
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }

    pub fn parse_operation(&mut self) -> Result<Operation, ParserError> {
        let op_type = self.parse_operation_type()?;
        self.advance()?;
        let identifier = self.parse_identifer()?;
        self.advance()?;
        let params = self.parse_param_list()?;
        self.advance()?;
        let body = self.parse_op_body()?;
        Ok(Operation {
            type_: op_type,
            name: identifier,
            parameters: params,
            body,
        })
    }

    pub fn parse_identifer(&mut self) -> Result<Identifier, ParserError> {
        if let Some((&Token::Identifier(ref n), span)) = self.peek() {
            Ok(Identifier {
                name: n.to_string(),
                span: span.clone(),
            })
        } else {
            Err(ParserError::UnexpectedToken)
        }
    }

    pub fn parse_store(&mut self) -> Result<Store, ParserError> {
        let store_name = self.parse_identifer()?;
        self.advance()?;
        let fields = self.parse_fieldlist()?;
        Ok(Store {
            name: store_name,
            fields,
        })
    }

    pub fn parse_fieldlist(&mut self) -> Result<Vec<Field>, ParserError> {
        self.parse_left_curly_bracket()?;
        self.advance()?;
        let mut results = vec![];
        while !matches!(self.peek(), Some((Token::RightCurlyBrace, _))) {
            results.push(self.parse_field()?);
            self.advance()?;
        }
        Ok(results)
    }

    pub fn parse_field(&mut self) -> Result<Field, ParserError> {
        let identifer = self.parse_identifer()?;
        self.advance()?;
        let type_ = self.parse_type()?;
        Ok(Field {
            name: identifer,
            type_,
        })
    }

    pub fn parse_type(&mut self) -> Result<Type, ParserError> {
        match self.peek() {
            Some((Token::LeftSquareBracket, _)) => {
                self.advance()?;
                if matches!(self.peek(), Some((Token::RightSquareBracket, _))) {
                    self.advance()?;
                    match self.peek() {
                        Some((token, _)) => match token {
                            &Token::Int32 => Ok(Type::Int32),
                            &Token::Int64 => Ok(Type::Int64),
                            &Token::Bool => Ok(Type::Boolean),
                            &Token::Uint32 => Ok(Type::Uint32),
                            &Token::Uint64 => Ok(Type::Uint64),
                            &Token::String => Ok(Type::String),
                            _ => Err(ParserError::UnexpectedToken),
                        },
                        None => Err(ParserError::UnexpectedEndOfInput),
                    }
                } else {
                    Err(ParserError::UnexpectedToken)
                }
            }
            Some((token, _)) => match token {
                &Token::Int32 => Ok(Type::Int32),
                &Token::Int64 => Ok(Type::Int64),
                &Token::Bool => Ok(Type::Boolean),
                &Token::Uint32 => Ok(Type::Uint32),
                &Token::Uint64 => Ok(Type::Uint64),
                &Token::String => Ok(Type::String),
                _ => Err(ParserError::UnexpectedToken),
            },
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }

    pub fn parse_semi_colon(&mut self) -> Result<(), ParserError> {
        match self.peek() {
            Some((&Token::SemiColon, _)) => Ok(()),
            Some(_) => Err(ParserError::UnexpectedToken),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }

    pub fn parse_colon(&mut self) -> Result<(), ParserError> {
        match self.peek() {
            Some((&Token::Colon, _)) => Ok(()),
            Some(_) => Err(ParserError::UnexpectedToken),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }

    pub fn parse_comma(&mut self) -> Result<(), ParserError> {
        match self.peek() {
            Some((&Token::Comma, _)) => Ok(()),
            Some(_) => Err(ParserError::UnexpectedToken),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }

    pub fn parse_left_curly_bracket(&mut self) -> Result<(), ParserError> {
        match self.peek() {
            Some((&Token::RightCurlyBrace, _)) => Ok(()),
            Some(_) => Err(ParserError::UnexpectedToken),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }
    pub fn parse_right_curly_bracket(&mut self) -> Result<Span, ParserError> {
        match self.peek() {
            Some((&Token::RightCurlyBrace, s)) => Ok(s.clone()),
            Some(_) => Err(ParserError::UnexpectedToken),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }

    pub fn parse_operation_type(&mut self) -> Result<OperationType, ParserError> {
        match self.peek() {
            None => Err(ParserError::UnexpectedEndOfInput),
            Some((token, _span)) => match token {
                Token::Modify => Ok(OperationType::Modify),
                Token::Delete => Ok(OperationType::Delete),
                Token::Query => Ok(OperationType::Query),
                _ => Err(ParserError::UnexpectedToken),
            },
        }
    }

    pub fn parse_left_paren(&mut self) -> Result<(), ParserError> {
        match self.peek() {
            Some((&Token::LeftParen, _)) => Ok(()),
            Some(_) => Err(ParserError::UnexpectedToken),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }

    pub fn parse_right_paren(&mut self) -> Result<(), ParserError> {
        match self.peek() {
            Some((&Token::RightParen, _)) => Ok(()),
            Some(_) => Err(ParserError::UnexpectedToken),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }

    pub fn parse_param_list(&mut self) -> Result<Vec<Parameter>, ParserError> {
        let mut result = vec![];
        self.parse_left_paren()?;
        self.advance()?;
        while !matches!(self.peek(), Some((Token::RightParen, _))) {
            result.push(self.parse_param()?);
            self.advance()?;
        }
        Ok(result)
    }

    pub fn parse_param(&mut self) -> Result<Parameter, ParserError> {
        let identifier = self.parse_identifer()?;
        self.advance()?;
        self.parse_semi_colon()?;
        let type_ = self.parse_type()?;
        Ok(Parameter {
            name: identifier,
            type_,
        })
    }

    pub fn parse_op_body(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut result = vec![];
        match self.peek() {
            Some((Token::LeftParen, _)) => {
                self.advance()?;
                while !matches!(self.peek(), Some((Token::RightParen, _))) {
                    result.push(self.parse_statement()?);
                }
                Ok(result)
            }
            _ => Ok(result),
        }
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        Err(ParserError::UnexpectedToken)
    }
}
