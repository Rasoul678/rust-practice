fn main() {
    parts::one::variables();
    parts::two::ownership();
    parts::three::functions();
}

mod parts {
    pub mod one;
    pub mod three;
    pub mod two;
}
