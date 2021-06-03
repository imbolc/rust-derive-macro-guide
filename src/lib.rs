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
fn goal() {
    assert_eq!(Foo::answer(), 42);
    assert_eq!(Bar::answer(), 0);
}
