# Derivative calculator

- [Derivative calculator](#derivative-calculator)
- [Usage](#usage)
- [How it works](#how-it-works)
  - [Tokenization](#tokenization)
  - [Hierarchical parsing](#hierarchical-parsing)
  - [Derivation](#derivation)
  - [Simplification](#simplification)
- [Limitations and future development](#limitations-and-future-development)
- [License](#license)


A simple command line tool for calculating derivatives.  

This project was born as a low-effort challenge to take the mind off of more complicated stuff like compilers and virtual machines.

# Usage

The command line tool is very basic. To derive a function, call the executable with the function definition in quotes:

```bash
dcalc "2*x + 6^x - 31"
```

To change the variable with respect to which to derive, use the `-d` option:

```bash
dcalc "2*y + 6^y - 31" -d y
```

For more info about using the command line, run with the `--help` flag:

```bash
dcalc --help
```

# How it works

Note that this is only one of the many possible approaches to creating a derivative calculator. Other software may do things slightly differently, but this is the general idea.

The program takes a function definition, and optionally a derivation variable, as input in the form of a string. The following processing is divided into 4 main steps:
 - Tokenization
 - Hierarchical parsing
 - Derivation
 - Simplification

## Tokenization

Node that, from here on, I'll refer use the terms "source code", "input string", and "input function definition" interchangeably. They all refer to the original input function definition, in the form of a string, to be derived.

How can a computer program understand a function definition in the form of a string? Just like compilers, the first step of any text-based program is transforming the input string into some meaningful structure.

Tokens are the basic blocks of the tokenization process. A token is an indivisible unit of syntax. The tokenizer uses a regex to divide the source code (the input string) into individual string slices (`SourceToken`s), according to the mathematical syntax. These `SourceToken`s keep a reference to their location in the source code in order to highlight eventual syntax errors directly on the original function definition, exactly like compilers do.

The raw tokens are then converted into higher-level `Token` structures. `SourceToken`s are mapped to a `Token` struct based on their string value.  
For example:
 - the raw string `"54"` is used to construct a `Token` of type `Number`
 - the raw string `"+"` is mapped to a `Token` of type `Plus`
 - the raw string `"fkew"` is mapped to a `Token` of type `Identifier` (usually variable name)

During tokenization, an evaluation priority is assigned to each `Token`. Evaluation priority determines which tokens are to be evaluated first, based on syntactical hierarchy:
 - Tokens found inside parentheses have a higher priority than surrounding tokens. In the expression `"a * (b + c)"`, the tokens `b`, `+`, and `c` have a higher priority than `a` and `*` because they are located within parentheses.
 - Literals (numbers, in this case) and identifiers are evaluated first because they are subsistent, meaning they don't require any additional tokens to complete their meaning.
 - Mathematical operator tokens (e.g. `Plus`, `Minus`, etc...) are given a priority value according to their precedence. For example, `Mul` and `Div` have a higher priority than `Plus` and `Minus`. 

The priority rules ensure that the arguments of each operator are always evaluated before the operator they are required by.

## Hierarchical parsing

Hierarchical parsing consists in parsing a list of tokens into a hierarchical tree, which is an abstract representation of the original function. The position of each node in the AST (abstract syntax tree) is determined by the priority of the corresponding token.

During this stage, the units of interest are the nodes of the tree. For reference, the `OpNode` struct represents a node in the function tree. Usually, a `OpNode` corresponds to exactly one `Token`, and consequently also to a `SourceToken`. However, some tokens (like parentheses) don't map to any `OpNode` because they would result redundant or useless. These tokens are simply dropped because they don't add any information to the tree. Note that, while parentheses are needed to express the order of operation in a linear expression, the tree's hierarchical structure already encodes the operation priority in the relation between its nodes.

The AST is built inside a loop with the following steps:
 - get the node with the highest evaluation priority in the token list
 - evaluate the highest priority node based on its type

To build the tree, each operator node extracts its operands from the token list and takes them as its arguments, transforming the linear list into a two-dimensional tree structure.  
The initial list of unparsed nodes/tokens is implemented through a doubly-linked list because of the frequent extraction operations that are required to build the tree.

## Derivation

The derivation step traverses the function tree in a depth-first fashion and recursively applies the basic derivation rules to each `OpNode`. The resulting tree is the derivative of the original function.

The `OpNode`s are immutable, and they are kept behind immutable reference-counted smart pointers (`Rc<OpNode`>) to avoid copying them during derivation. Since derivatives often repeat operator functions multiple times, using shared immutable references allows borrowing the original nodes without copying.

## Simplification

Once the derivative function is calculated, it is simplified. The derivative function tree is traversed in a depth-first fashion and constant operation nodes are evaluated in a process known in compiler design as constant folding.  
For example:
 - the constant expression `1+1` is evaluated to `2`
 - the expression `x * (3 + 4)` is evaluated to `x * 7`
 - the expression `x^2 + 3*x + 5 + 2` is evaluated to `x^2 + 3*x + 7`

# Limitations and future development

This is a hobby project and, as such, is not meant to be production-ready or in continuous development. The [TODO.md](TODO.md) file contains a roadmap of the project, its current development state, and eventual future additions.

# License

This software and all the contents of this repository are published under the [MIT license](LICENSE).
