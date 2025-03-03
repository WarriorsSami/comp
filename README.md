# Rust proc macro for Python list comprehension

This is a simple example of a Rust proc macro that generates Python list comprehension code based on Logan Smith's [Comprehending Proc Macros](https://youtu.be/SMCRQj9Hbx8?si=AdoJqrFkIrD5NE61) video. 

Grammar:
```
comp: mapping for_if_clause

mapping: expression

for_if_clause:
 | 'for' pattern 'in' expression ('if' expression)*

pattern: name (, name)*
```

Example:
```rust
comp![x * x for x in 0..10 if x % 2 == 0];
```