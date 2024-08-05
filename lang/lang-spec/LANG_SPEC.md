# Language Specification

This is a rough specification of the language used to interact with LokiDB.
## Keywords

### `store`

**Example**
```
store Dog {
	name: string,
	age: int,
	...
};
```

The `store` keyword is used to define the schema of an object in Loki.

For the rest of this section, let's assume we've defined these stores

```
store Course {
	Name String
	ID UUID
	Teachers Teacher[]
	Students Student[]
}

store Teacher {
	Name String
	ID UUID
	CoursesTaught Course[]
}

store Student {
	Name String
	ID UUID
	Enrolled Course[]
}
```

### `query`
```
query students_taught_by {
	Course
	.where(Name = $ AND ID > $)
	.Students
}
```

### `modify`
```
modify students_taught_by() {
	Name: $,
}
```

### `delete`
```
delete students_by_id()
```

### Grammar

```
// Top-level declarations
Decl := Store | Operation

// Store declaration
Store := 'store' Identifier '{' FieldList '}'
FieldList := Field | FieldList ',' Field
Field := Identifier ':' Type
Type := 'String' | 'UUID' | 'int' | Identifier '[]' 

// Operations
Operation := OperationType Identifier '(' ParameterList ')' '{' OperationBody '}'
OperationType := 'query' | 'modify' | 'delete'
ParameterList := /* empty */ | Parameter | ParameterList ',' Parameter
Parameter := Identifier ':' Type

// Operation body
OperationBody := /* empty */ | Statement | OperationBody Statement
Statement := Expression | Assignment

// Expressions
Expression := Identifier | Literal | MethodChain | BinaryExpr | UnaryExpr | '(' Expression ')'
MethodChain := Identifier | MethodChain '.' MethodCall
MethodCall := Identifier '(' ArgumentList ')'
ArgumentList := /* empty */ | Argument | ArgumentList ',' Argument
Argument := Expression | Identifier '=' Expression

// Binary and Unary Expressions
BinaryExpr := Expression Operator Expression
UnaryExpr := UnaryOperator Expression
Operator := '=' | '<' | '>' | '<=' | '>=' | 'AND' | 'OR'
UnaryOperator := 'NOT'

// Assignment (for modify operations)
Assignment := Identifier ':' Expression

// Literals and Identifiers
Literal := StringLiteral | NumberLiteral | BooleanLiteral | '$'
Identifier := [a-zA-Z_][a-zA-Z0-9_]*
StringLiteral := '"' /* any characters except " */ '"'
NumberLiteral := [0-9]+
BooleanLiteral := 'true' | 'false'
```

