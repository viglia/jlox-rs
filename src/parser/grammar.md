### Grammar

```
expression     → literal
               | unary
               | binary
               | grouping ;

literal        → NUMBER | STRING | "true" | "false" | "nil" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
               | "+"  | "-"  | "*" | "/" ;
```

Without well-defined precedence and associativity, an expression that uses multiple operators is 
ambiguous—it can be parsed into different syntax trees, which could in turn evaluate to different results.


### Precedence rules going from lowest to highest.

```
Name           Operators     Associates
Equality       == !=         Left
Comparison     > >= < <=     Left
Term           - +           Left
Factor         / *           Left
Unary          ! -           Right
```

Right now, the grammar stuffs all expression types into a single expression rule. 
That same rule is used as the non-terminal for operands, 
which lets the grammar accept any kind of expression as a subexpression, 
regardless of whether the precedence rules allow it.

We define a separate rule for each precedence level.
Each rule here only matches expressions at its precedence level or higher.

```
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
```