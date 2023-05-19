fn main() {
    parts::one();
    parts::two();
    parts::three();
}

mod parts {
    pub mod ownership;
    pub mod variables;
    pub mod functions;
    pub use ownership::show as two;
    pub use variables::show as one;
    pub use functions::show as three;
}
