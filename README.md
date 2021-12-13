# Basics of Rust

### [derive(Debug)]

### Tuple iteration
We can get each index value by specifiying index after the variable
```
let tuple = ("hello", 5, 'c');

assert_eq!(tuple.0, "hello");
assert_eq!(tuple.1, 5);
assert_eq!(tuple.2, 'c');
```

### TryInto