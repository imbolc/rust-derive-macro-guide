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

So these derives wold expand into

```rust
impl MyTrait for Foo {}

impl MyTrait for Bar {
    fn answer() -> i32 {
        0
    }
}
```

## Step 1: a separate crate for the macro

Proc macros should live in a separate crate. Let's create one in a sub-folder
and make it a dependency for our root crate

```sh
cargo new --lib mytrait-derive
cargo add mytrait-derive --path mytrait-derive
```

We should also tell Cargo that `mytrait-derive` is a proc-macro crate:
```sh
cat >> mytrait-derive/Cargo.toml << EOF
[lib]
proc-macro = true
EOF
```

## Step 2: default trait implementation

Now let's make `#[derive(MyTrait)]` work. We'll need to add a few goto
dependencies to our proc-macro crate

```sh
cd mytrait-derive
cargo add proc-macro2@1.0 quote@1.0
cargo add syn@1.0 --features full
```

And here's our default trait implementation (in `mytrait-derive/src/lib.rs`):

```rust
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyTrait)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl MyTrait for #ident {}
    };
    output.into()
}
```

You can think of `ident` as a name of a struct or enum we're deriving the
implementation for. We're getting it from the `parse_macro_input!` and then we
use it in the `quote!`, which is like a template engine for Rust code
generation.

Now this test (in `src/lib.rs`) should pass:

```rust
use mytrait_derive::MyTrait;

trait MyTrait {
    fn answer() -> i32 {
        42
    }
}

#[derive(MyTrait)]
struct Foo;

#[test]
fn default_impl() {
    assert_eq!(Foo::answer(), 42);
}
```

Also you should be able to find the implementation in the output of [cargo-expand][]

```sh
cargo expand | grep 'impl MyTrait'
impl MyTrait for Foo {}
```


[cargo-expand]: https://github.com/dtolnay/cargo-expand
