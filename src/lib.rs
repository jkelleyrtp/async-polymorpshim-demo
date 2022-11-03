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
struct Element;

trait ReturnType<'a, M = ()> {
    fn alloc(self, scope: &'a Scope) -> SuspenseMode<'a>;
}

struct AsyncMarker;

impl<'a> ReturnType<'a> for Element {
    fn alloc(self, scope: &'a Scope) -> SuspenseMode<'a> {
        SuspenseMode::Sync(scope.alloc(self))
    }
}

enum SuspenseMode<'a> {
    Sync(&'a Element),
    Async(Box<dyn Future<Output = Element> + 'a>),
}
impl<'a, F> ReturnType<'a> for F
where
    F: Future<Output = Element> + 'a,
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
    Element
}

async fn example2(cx: &Scope) -> Element {
    Element
}

fn does_it_work() {
    it_goes(example as fn(_) -> _);
    it_goes(example2 as fn(_) -> _);
}
