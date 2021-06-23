#![allow(dead_code)]

struct Grounded;
struct Launched;

struct Kilograms(u32);
struct Color;

struct Rocket<Stage = Grounded> {
    stage: std::marker::PhantomData<Stage>,
}

impl Default for Rocket<Grounded> {
    fn default() -> Rocket<Grounded> {
        todo!()
    }
}

impl Rocket<Grounded> {
    pub fn launch(self) -> Rocket<Launched> {
        todo!()
    }
}
impl Rocket<Launched> {
    pub fn accelerate(&mut self) {}
    pub fn decelerate(&mut self) {}
}
impl<Stage> Rocket<Stage> {
    pub fn color(&self) -> Color {
        todo!()
    }
    pub fn weight(&self) -> Kilograms {
        todo!()
    }
}

//---

pub trait CanUseCannotImplement: sealed::Sealed {}
trait TraitBounds {}
mod sealed {
    use super::*;
    pub trait Sealed {}
    impl<T> Sealed for T where T: TraitBounds {}
}
impl<T> CanUseCannotImplement for T where T: TraitBounds {}

//---

fn is_normal<T: Sized + Send + Sync + Unpin>() {}

#[test]
fn normal_types() {
    struct MyType;
    is_normal::<MyType>();
}

fn main() {}
