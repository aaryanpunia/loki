This is a rough specification of the language used to interact with LokiDB. It uses [Backus-Naur form][https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form] to describe the syntax.
# Keywords

## Struct

**Example
```
struct Dog {
	name: string,
	age: int,
	...
};
```

The `struct` keyword is used to define the schema of an object in Loki.

**BNF
```
<struct> ::= <struct> <{> <field>+ <}> <;>
<field> ::= <type> <:> <name> <,>
```


# Structs and stores

A store in Loki is a type of data structure. A struct is data that a store sends and receives. 


# Stores
A store in Loki is a type of data structure. Its primary purpose is to store data. Each store may have a different API to interact with structs due to difference in underlying implementation and purpose.

## Kinds of stores
### HashMap

### List

### Set

### LinkedList

### SelfBalancingBinaryTree (Name TBD)

# Types in Loki

## Primitives
Loki utilizes primitives to allow users to create their own types using *structs*.
### Scalar Types
#### Integer Types

#### Floating-Point Types

#### The Boolean Type

#### The Character Type

### Strings


