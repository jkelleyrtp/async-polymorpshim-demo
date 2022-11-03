use std::future::Future;

trait AnyFn {}

struct Scope;
struct Element;

trait ReturnType<M = ()> {}

struct AsyncMarker;

impl ReturnType for Element {}

enum SuspenseMode<'a> {
    Sync(&'a Element),
    Async(Box<dyn Future<Output = Element> + 'a>),
}
impl<F> ReturnType for F where F: Future<Output = Element> {}
impl<F: ReturnType> AnyFn for fn(Scope) -> F {}

fn it_goes(f: impl AnyFn) {}

fn example(cx: Scope) -> Element {
    Element
}

async fn example2(cx: Scope) -> Element {
    Element
}

fn does_it_work() {
    it_goes(example as fn(_) -> _);
    it_goes(example2 as fn(_) -> _);
}
