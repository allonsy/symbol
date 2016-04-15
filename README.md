# symbol [![Build Status](https://travis-ci.org/allonsy/symbol.svg?branch=master)](https://travis-ci.org/allonsy/symbol)
A small esoteric symbolic programming language by allonsy. This language is a language inspired by
the esoteric programming language '<><'

# Building
* Please clone the repo
* Make sure that you have rust and cargo installed
* run `cargo build --release` to build the `symbol` binary without debugging symbols
* You may know call the binary with `cargo run <filename>` or locate the `symbol` binary and call `./symbol <filename>`

# Language Specs
The language is written in 2D source files. Execution starts in the top left corner with control flow proceeding from left to right.
As it encounters symbols, it performs the operation designated by that symbol.
Program ends when it hits a ';' or a 'wall' which is to say that control flow enters a place where no character is written.
Think like the game 'snake' where the snake keeps on going, turning and eating until it hits a wall.
Note that you can have lines of different lengths however, the evaluator will not extend all lines to match the largest line length.
Therefore, if control flow travels along a long line and then goes up or down where the lines immediately above and below it
have no characters written, execution will terminate as if it had hit a wall.

The evaluator maintains a stack of signed 32bit integers (negatives allowed). I may extend this to 64bit ints in the future.
Certain actions may push numbers onto the stack, pop them off, or conditionally change control flow based on values on the stack.

The symbols in the language are as follows:

* `>` : Control flow direction changes to moving right (the way english is read)
* `<` : Control flow direction changes to moving left
* `^` : Control flow direction changes to moving up
* `v` : Control flow direction changes to moving down
* `0`, `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, `9` : Pushes the corresponding number onto the stack (eg `1` pushes the number 9 onto the stack)
* `p` : Unconditionally pop off the top of the stack
* `@` : Duplicate the top item of the stack and push it onto the top (so if `1` is currently at the top of the stack, `@` will result in the top looking looking like: `1` (the new one) followed by `1` (the original one))
* `+` : pops the first two elements off the top of the stack, adds them and pushes the result onto the stack
* `-` : pops the first two elements off the top, subtracts them, and pushes the result onto the stack
* `*` : pops the first two elements off the top, multiplies them, and pushes the result onto the stack
* `/` : pops the first two elements off the top, integer divides them, and pushes the result onto the stack
* `;` : Unconditionally stops execution of the program
* `|` : pops the first element off the top of the stack, if that element is zero, then control flow changes to moving upward, otherwise, control flow moves downward.
* `_` : pops the first element off the top of the stack, if that element is zero, then control flow changes to moving left and otherwise, control flow moves right
* `e` : If the stack is empty, the number `1` is pushed onto the stack and `0` is pushed on to the stack otherwise
* `!` : pops the top element of the stack. If that element is 0, `1` is pushed onto the stack. `0` is pushed onto the stack otherwise (logical negation)
* `~` : pops the top element off the top of the stack, multiplies that element by `-1` and pushes the result onto the stack
* `.` : pops the top element off the stack, truncates it to an unsigned bye character (ASCII encoded) and prints it to the screen (standard output)
* `$` : reads one character from standard input, converts it to a signed 32 bit integer, and pushes it onto the stack.
* `g` : pops the top element off the stack, if it is greater than 0, `1` is pushed onto the stack and `0` is pushed on otherwise
* `l` : pops the top element off the stack, if it is less than 0, `1` is pushed onto the stack and `0` is pushed on otherwise
* ` ` : The space character is ignored and control flow moves on to the next character in the reading direction
* `,` : Ignored exactly like the space character. This helps if you are using an editor that likes to remove lines with only spaces (cough cough atom cough cough)
* If the reader encounters any character it doesn't recognize, it will panic with an error
* Note that there are no comments in the language. However, you can always include comments in places where you are certain control
flow will never enter (see the hello world example)
# Examples
The following is hello world written in symbol. It will run as is even with the included comments:
```
91+v          pushes newline onto the stack (ascii 10)
v  <
>91+3*3+v     pushes ! onto the stack (ascii 33)
v       <
>91+@*v       pushes d onto the stack
v     <
>91+@*8+v     pushes l onto the stack
v       <
>91+@*77++v   pushes r onto the stack
v         <
>91+@*56++v   pushes o onto the stack
v         <
>891+*7+v     pushes w onto the stack
v       <
>84*v         etc...
v   <
>91+4*4+v
v       <
>91+@*56++v
v         <
>91+@*8+v
v       <
>91+@*8+v
v       <
>91+@*1+v
v       <
>98*v
v   <
,
   ;
>e!|        iterates over the stack until empty, popping and printing each character
   .
^  <
```
Please see the `examples` directory for more examples

# Issues and Contributing
* If you have a feature request for the language or bug, please don't hesitate to fill out an issue!
* If you would like to contribute, I welcome all PRs. Just fork the repo, make changes, and submit a PR!
* Please run `cargo test` before submitting any PRs to make sure that all tests pass
