
# async polymorphism demo

Let your rust functions take
- fn()
- async fn()

In the same interface. Used by dioxus to implement suspense.

## How it works
- A trait for the interface itself
- A trait for the return type
