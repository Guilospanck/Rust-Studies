# References 
How to use references in Rust.

## Concepts
A reference is a pointer to some variable address (like C, Go, etc);
In Rust, a reference to a variable is done the same way:
```rust
let mut x = 10;

let xReference = &x; // xReference is an immutable reference that points to the same address as x
```
In the example above, `xReference` only points to the address of `x`, it can't change it.

In order to change the <b>value</b> of `x` using its reference `xReference` we need to transform it
into a <b>mutable reference</b>:
```rust
let mut x = 10;

let xReference = &mut x; // now xReference is a mutable reference to x. That means that we can change the *value* of x by changing the *value* of xReference
```
In order to change the value of `x` through `xReference`, we can do:
```rust
*xReference += 1; // this will change the *value* that xReference points to, which is x
println!("{}", xReference); // 11
println!("{}", x); // error. x is already borrowed
```

In order to fix the error above, we can use <code>Scope</code>:
```rust
let mut x = 10;
{
  let xReference = &mut x; 
  *xReference +=1

  println!("{}", xReference); // 11
}

println!("{}", x); // 11
```
