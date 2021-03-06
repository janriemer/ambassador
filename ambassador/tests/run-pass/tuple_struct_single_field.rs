extern crate ambassador;

use ambassador::{delegatable_trait, Delegate};

#[delegatable_trait]
pub trait Shout {
    fn shout(&self, input: &str) -> String;
}

pub struct Cat;

impl Shout for Cat {
    fn shout(&self, input: &str) -> String {
        format!("{} - meow!", input)
    }
}

#[derive(Delegate)]
#[delegate(Shout)]
pub struct WrappedCat(Cat);

pub fn main() {
    let foo_animal = WrappedCat(Cat);
    println!("{}", foo_animal.shout("BAR"));
}
