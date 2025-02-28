# trampoline

This is a trampoline function in Rust.

```rs
enum Trampoline<T> {
    Val(T),
    Recur(Box<dyn Fn() -> Trampoline<T>>),
}

fn trampoline<T>(t: Trampoline<T>) -> T
where
    T: Clone,
{
    let mut t = t;

    loop {
        match t {
            Trampoline::Val(t) => {
                return t.clone();
            }
            Trampoline::Recur(f) => {
                t = f();
            }
        }
    }
}
```
