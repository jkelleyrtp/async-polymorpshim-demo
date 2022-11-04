use std::future::Future;

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

fn it_goes<'a, F: ReturnType<'a>>(f: fn(&'a Scope) -> F) {}

fn example(cx: &Scope) -> Element {
    todo!()
}

async fn example2(cx: &Scope) -> Element {
    todo!()
}

fn does_it_work() {
    it_goes(example);
    it_goes(example2);
}
