use mytrait_derive::MyTrait;

trait MyTrait {
    fn answer() -> i32 {
        42
    }
}

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
