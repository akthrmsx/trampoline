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

fn fibonacci_trampoline(n: u128) -> u128 {
    fn recur(i: u128, a: u128, b: u128) -> Trampoline<u128> {
        if i == 0 {
            Trampoline::Val(a)
        } else {
            Trampoline::Recur(Box::new(move || recur(i - 1, b, a + b)))
        }
    }

    trampoline(recur(n, 1, 1))
}

fn main() {
    println!("{}", fibonacci_trampoline(120));
}
