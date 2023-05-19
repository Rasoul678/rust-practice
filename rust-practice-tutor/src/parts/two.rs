pub fn ownership() {
    println!("################# ownership #################");
    // Strings are stored in both stack and heap
    // They are moved by default, unless we copy by borrowing
    let mut s1 = String::from("Rasoul");
    let s2 = &s1;

    println!("Your name is {}", s1);
    println!("Your name is {}", s2);

    greet(&s1);
    greet(s2);

    empty_string(&mut s1);

    let mut name = String::from("John");
    let name1 = &mut name;
    // It is not allowed to borrow as mutable more than once
    // let mut name2 = &mut name;

    empty_string(name1);

    let another_name = String::from("Jack");
    let another_name1 = &another_name;
    // It is not allowed to borrow as mutable because it is also borrowed as immutable
    // let another_name2 = &mut another_name;

    println!("{}", another_name);
    println!("{}", another_name1);
    // println!("{}", another_name2);

    // Integers are stored in stack
    // They are copied by default
    let age1 = 10;
    let age2 = age1;

    println!("You are {} years old", age1);
    println!("You are {} years old", age2);

    // Dangling references
    let my_name00 = get_name();
    println!("{}", my_name00);
    println!("################# ownership #################");
}

fn greet(name: &String) {
    println!("Hello, {}", name)
}

fn empty_string(value: &mut String) {
    value.clear();
}

fn get_name() -> String {
    String::from("Rasoul in function")
}
