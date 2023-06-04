use std::fmt;

pub fn traits() {
    println!("################# traits #################");
    // traits are shared functionality
    let person = Person {
        first_name: "Rasoul".to_string(),
        last_name: "Hesami".to_string(),
        age: 35,
    };

    println!("{:?}", person);

    let person2 = Person::new("John Doe");
    // println!(
    //     "First name: {}, last name: {}, age: {}",
    //     person2.first_name, person2.last_name, person2.age
    // );

    // With Display trait
    println!("{}", person);

    // print_full_name_and_age(&person);
    // print_details(&person);

    let full_name = person2.full_name();
    println!("{}", full_name);

    println!("################# traits #################");
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

trait HasFullName {
    fn full_name(&self) -> String;
    fn get_age(&self) -> u8;
}

// impl HasFullName for Person {
//     fn full_name(&self) -> String {
//         format!("{} {}", self.first_name, self.last_name)
//     }

//     fn get_age(&self) -> u8 {
//         self.age
//     }
// }

// trait with new function
trait CanInitializeWithFullName {
    fn new(full_name: &str) -> Self;
}

impl CanInitializeWithFullName for Person {
    fn new(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split(' ').collect();

        Person {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
            age: 35,
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} is {} years old",
            self.first_name, self.last_name, self.age
        )
    }
}

// traits as parameters
fn print_full_name_and_age(value: &impl HasFullName) {
    println!("{}", value.full_name());
}

// trait bound syntax
// fn print_details<T: HasFullName + CanRun>(value: &T) {
//     println!("{}", value.full_name());
//     value.run();
// }

fn print_details<T>(value: &T)
where
    T: HasFullName + CanRun,
{
    println!("{}", value.full_name());
    value.run();
}

// multiple traits
trait CanRun {
    fn run(&self);
}

impl CanRun for Person {
    fn run(&self) {
        todo!()
    }
}

// implement trait on another trait
trait HasName {
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
}

trait HasFullName2
where
    Self: HasName,
{
    // fn full_name(&self) -> String {
    //     format!("{} {}", self.first_name(), self.last_name())
    // }
    fn full_name(&self) -> String;
}

impl<T> HasFullName2 for T
where
    T: HasName,
{
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name(), self.last_name())
    }
}

impl HasName for Person {
    fn first_name(&self) -> &str {
        &self.first_name
    }

    fn last_name(&self) -> &str {
        &self.last_name
    }
}
