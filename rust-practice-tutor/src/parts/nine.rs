pub fn lifetimes() {
    println!("################# lifetimes #################");
    let _full_name = get_full_name_static();
    let _full_name2 = get_full_name();
    let _random_name = get_random_name("John", "Jane");

    let _person = Person { name: "Rasoul" };

    println!("################# lifetimes #################");
}

fn get_full_name_static() -> &'static str {
    "John Doe"
}

// Generic life time
fn get_full_name<'t>() -> &'t str {
    "John Doe"
}

// fn get_random_name<'l1, 'l2>(a: &'l1 str, b: &'l2 str) -> &'l1 str {
//     a
// }

fn get_random_name<'l>(a: &'l str, b: &'l str) -> &'l str {
    a
}

struct Person<'a> {
    name: &'a str,
}

// lifetime elision
// automatic lifetime
fn get_first_name(full_name: &str) -> &str {
    full_name
}

// fn get_first_name<'a>(full_name: &'a str) -> &'a str {
//     full_name
// }

struct Person1<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

impl<'a> Person1<'a> {
    // fn first_char_of_first_name(&'a self) -> &'a str {
    //     &self.first_name[0..1]
    // }

    fn first_char_of_first_name(&self) -> &str {
        &self.first_name[0..1]
    }

    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

enum Animal<'a> {
    Cat { name: &'a str },
    Dog { name: &'a str },
}
