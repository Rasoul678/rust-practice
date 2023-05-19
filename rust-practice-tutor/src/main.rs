fn main() {
    parts::one();
    parts::two();
}

mod parts {
    pub mod ownership;
    pub mod variables;
    pub use ownership::show as two;
    pub use variables::show as one;
}
