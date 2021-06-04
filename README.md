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

#[test]
fn default() {
    assert_eq!(Foo::answer(), 42);
}

#[test]
fn getter() {
    assert_eq!(Bar::answer(), 0);
}
```

So these derives would expand into

```rust
impl MyTrait for Foo {}

impl MyTrait for Bar {
    fn answer() -> i32 {
        0
    }
}
```

## Step 0: prerequisites

Install Cargo extended tools

```sh
cargo install cargo-edit
cargo install cargo-expand
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

Now let's make `#[derive(MyTrait)]` work. We'll need to add a few dependencies
to our macro crate

```sh
cd mytrait-derive
cargo add proc-macro2@1.0 quote@1.0
cargo add syn@1.0 --features full
```

And here's our default trait implementation (`mytrait-derive/src/lib.rs`):

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

Now this test (`src/lib.rs`) should pass:

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
fn default() {
    assert_eq!(Foo::answer(), 42);
}
```

Also you should be able to find the implementation in the output of [cargo-expand][]

```sh
cargo expand | grep 'impl MyTrait'
impl MyTrait for Foo {}
```

## Step 3: the getter initialization

Now it's time to make our getter initializable by `#[my_trait(answer = ...)]`
attribute.  We'll need one more crate for convenient parsing of the
initialization value

```sh
cd mytrait-derive
cargo add darling@0.13
```

Here's the final version of our macro (`mytrait-derive/src/lib.rs`):

```rust
use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(my_trait))]
struct Opts {
    answer: Option<i32>,
}

#[proc_macro_derive(MyTrait, attributes(my_trait))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;

    let answer = match opts.answer {
        Some(x) => quote! {
            fn answer() -> i32 {
                #x
            }
        },
        None => quote! {},
    };

    let output = quote! {
        impl MyTrait for #ident {
            #answer
        }
    };
    output.into()
}
```

Struct `Opts` describes parameters of the `#[my_trait(...)]` attribute. Here we
have only one of them - `answer`. Notice that it's optional, because we don't
want to overwrite the default `fn answer()` implementation if the attribute
wasn't used.

The `qoute!` macro is composable - we can use output of one of them in another.
So in the `match` we check if the initializer is passed and create the method
implementation or just nothing. And finally we use the result in the outer
`qoute!` template.

That's all, clone this repo to play with the code.

[cargo-expand]: https://github.com/dtolnay/cargo-expand
