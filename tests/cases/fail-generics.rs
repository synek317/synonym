use synonym::Synonym;

#[derive(Synonym)]
pub struct Foo<T> {
    _foo: std::marker::PhantomData<T>
}

fn main() {}
