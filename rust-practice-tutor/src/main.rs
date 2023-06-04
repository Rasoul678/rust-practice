fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parts::one::variables();
    // parts::two::ownership();
    // parts::three::functions();
    // parts::four::structures();
    // parts::five::enumerations();
    // parts::six::collections();
    // parts::seven::optionals();
    // parts::eight::errors()?;
    parts::nine::lifetimes();

    Ok(())
}

mod parts {
    // pub mod five;
    // pub mod four;
    // pub mod one;
    // pub mod three;
    // pub mod two;
    // pub mod six;
    // pub mod seven;
    // pub mod eight;
    pub mod nine;
}
