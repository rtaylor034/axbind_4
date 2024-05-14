pub use optwrite_derive::OptWrite;
pub trait OptWrite {
    fn optwrite(&mut self, other: Self);
    fn overriden_by(self, other: Self) -> Self;
}
impl<T> OptWrite for Option<T> {
    fn optwrite(&mut self, other: Self) {
        if let Some(v) = other {
            *self = Some(v);
        }
    }
    fn overriden_by(self, other: Self) -> Self {
        match other {
            Some(_) => other,
            None => self,
        }
    }
}
//easy way out, at some point should add attribute ignore support (or just be better ig)
//its silly really
impl<T> OptWrite for core::marker::PhantomData<T> {
    fn optwrite(&mut self, _other: Self) {}
    fn overriden_by(self, _other: Self) -> Self { core::marker::PhantomData }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(OptWrite, Debug)]
    struct Tester {
        foo: Option<usize>,
        bar: Option<usize>,
    }
    #[test]
    fn it_works() {
        let old = Tester {
            foo: Some(2),
            bar: Some(0),
        };
        let now = Tester {
            foo: None,
            bar: Some(35),
        };
        let bruh = old.overriden_by(now);
        panic!("{:?}", bruh);
    }
}
