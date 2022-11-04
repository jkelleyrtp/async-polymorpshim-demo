use std::future::Future;

/// A trait that lets us do something with the fn itself
trait AnyFn {
    fn call(self, scope: &Scope) -> SuspenseMode;
}

struct Scope;

impl Scope {
    fn alloc<T>(&self, val: T) -> &T {
        todo!()
    }
}

struct Element<'a>(&'a str);

trait ReturnType<'a> {
    fn alloc(self, scope: &'a Scope) -> SuspenseMode<'a>;
}

impl<'a> ReturnType<'a> for Element<'a> {
    fn alloc(self, scope: &'a Scope) -> SuspenseMode<'a> {
        SuspenseMode::Sync(scope.alloc(self))
    }
}

enum SuspenseMode<'a> {
    Sync(&'a Element<'a>),
    Async(Box<dyn Future<Output = Element<'a>> + 'a>),
}
impl<'a, F> ReturnType<'a> for F
where
    F: Future<Output = Element<'a>> + 'a,
{
    fn alloc(self, scope: &'a Scope) -> SuspenseMode<'a> {
        SuspenseMode::Async(Box::new(self))
    }
}

impl<'a, F: ReturnType<'a>> AnyFn for fn(&'a Scope) -> F {
    fn call(self, scope: &Scope) -> SuspenseMode {
        todo!()
    }
}

fn it_goes(f: impl AnyFn) {}

fn example(cx: &Scope) -> Element {
    todo!()
}

async fn example2(cx: &Scope) -> Element {
    todo!()
}

fn does_it_work() {
    it_goes(example as fn(_) -> _);
    it_goes(example2 as fn(_) -> _);
}
