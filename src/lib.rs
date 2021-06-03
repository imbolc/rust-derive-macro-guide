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
