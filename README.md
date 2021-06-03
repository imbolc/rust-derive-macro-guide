# #[derive(MyTrait)]

A copypastable guide to implementing simple derive macros in Rust.

## The goal

Let's say we have a trait with a getter

```rust
trait MyTrait {
    fn answer() -> i32 {
        42
    }
}
```

And we want to be able to derive it and initialize the getter

```rust
#[derive(MyTrait)]
struct Foo;

#[derive(MyTrait)]
#[my_trait(answer = 0)]
struct Bar;

fn main() {
    assert_eq!(Foo::answer(), 42);
    assert_eq!(Bar::answer(), 0);
}
```

So these derives should expand into:
```rust
impl MyTrait for Foo {}

impl MyTrait for Bar {
    fn answer() -> i32 {
        0
    }
}
```

