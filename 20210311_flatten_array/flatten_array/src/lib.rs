#![feature(allow_internal_unstable)]

/// Using an enum with embedded values allows me to treat single values and
/// arbitrarily nested values as the same, at least as far as the type system
/// is concerned. I can use 'match' statements where necessary to differentiate
/// between the two.
#[derive(Debug)]
pub enum Nested<T: Copy> {
    Vec(NestedVec<T>),
    Value(T),
}

pub type NestedVec<T> = Vec<Nested<T>>;


/// I ended up not actually *needing* any of these traits, but implementing Nest
/// for any type gives it access to the `.nest()` method, which would convert
/// that type to a `Nested` enum, which can be stored in a NestedVec along with other
/// nested items. There is probably a better way to do this.
pub trait Nest<T>
where
    T: Copy,
{
    fn nest(self) -> Nested<T>;
}

impl<T: Copy> Nest<T> for T {
    fn nest(self) -> Nested<T> {
        Nested::Value(self)
    }
}

impl<T: Copy> Nest<T> for Vec<T> {
    fn nest(self) -> Nested<T> {
        let nested_vals: Vec<_> = self.iter().map(|x| Nested::Value(*x)).collect();
        Nested::Vec(nested_vals)
    }
}

impl<T: Copy> Nest<T> for &[T] {
    fn nest(self) -> Nested<T> {
        let nested_vals: Vec<_> = self.iter().map(|x| Nested::Value(*x)).collect();
        Nested::Vec(nested_vals)
    }
}

/// Trait providing the `.flatten()` function for nested elements
pub trait Flatten<T> where T: Copy {
    fn flatten(&self) -> Vec<T>;
}

impl<T: Copy> Flatten<T> for NestedVec<T> {
    fn flatten(&self) -> Vec<T> {
        let mut output = Vec::new();
        for item in self {
            match item {
                Nested::Value(val) => output.push(*val),
                Nested::Vec(vec) => output.extend(&vec.flatten())
            }
        }
        output
    }
}

pub trait PushNested<T> where T: Copy + Nest<T> { 
    fn push_nested(&mut self, value: T);
}

impl<T: Copy + Nest<T>> PushNested<T> for NestedVec<T> {
    fn push_nested(&mut self, value: T) {
        self.push(value.nest())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_my_work() {

        let z: Nested<i32> = vec![7, 8, 9].nest();
        let mut y: NestedVec<i32> = NestedVec::new();
        y.push_nested(4);
        y.push_nested(5);
        y.push_nested(6);
        y.push(z);
        let mut x: NestedVec<i32> = NestedVec::new();
        x.push_nested(1);
        x.push_nested(2);
        x.push_nested(3);
        x.push(Nested::Vec(y));

        assert_eq!(x.flatten(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
