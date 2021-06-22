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

fn main() {}
