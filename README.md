# Slang
A macro expansion program for simple language abstractions, built in rust

The goal of Slang is to allow simple macro languages to be built out of pattern matching rules.
These rules will support matching any sequence of tokens, variables and blocks.
However, the pattern for any rule may not be the prefix of any other rule.

## Examples

### C-style if to Python-style
Pattern
```
#define if ( $cond ) { $block }
if $cond:
  $block
```

Input
```
if (a == b) {
  some_func();
}
```

Output
```
if a == b:
  some_func();
```
