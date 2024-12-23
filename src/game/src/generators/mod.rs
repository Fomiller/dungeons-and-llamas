pub mod battle;
pub mod boss;
pub mod rest;
pub mod shop;
pub mod theme;

pub trait Generator {
    fn generate(&self) -> String;
}
