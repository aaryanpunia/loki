pub trait Number {}

pub enum Decl {
    Store(Store),
    Operation(Operation),
}

pub struct Store {
    pub name: String,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: String,
    pub type_: Type,
}

// tbd
pub enum Type {
    String,
    UUID,
    Int,
    Array(Box<Type>),
    Custom(String),
}

pub struct Operation {
    pub type_: OperationType,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Statement>,
}

pub enum OperationType {
    Query,
    Modify,
    Delete,
}

pub struct Parameter {
    pub name: String,
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
