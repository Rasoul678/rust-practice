fn main() {
    parts::one::variables();
    parts::two::ownership();
    parts::three::functions();
    parts::four::structures();
}

mod parts {
    pub mod four;
    pub mod one;
    pub mod three;
    pub mod two;
}
