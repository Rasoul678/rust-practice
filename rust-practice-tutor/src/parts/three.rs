pub fn functions() {
    println!("################# functions #################");
    let message = get_hello_world();
    // println!("{}", message);
    say_hello_world(&message);
    let hello = say_hello_to(String::from("Rasoul"));
    println!("{}", hello);

    // Inline function
    let inline_func = |name: &str| format!("Hey {}", name);
    let msg = inline_func(&message);
    println!("{}", msg);

    let ask_for_age = || {
        let mut age = String::new();
        println!("How old are you?");
        std::io::stdin()
            .read_line(&mut age)
            .expect("Failed to read line");
        age.trim().parse::<u32>().expect("Please type a number: ");
        age
    };

    let age = ask_for_age();
    println!("You are {} years old!", age);

    // Function pointer
    let multiply_by_2 = |x: i32| x * 2;
    let ptr = multiply_by_2;
    println!("{}", ptr(2));

    // Function as argument
    process_name(&message, say_hello_world);
    println!("################# functions #################");
}

fn process_name(name: &str, callback: fn(&str) -> ()) {
    callback(name);
}

fn get_hello_world() -> String {
    String::from("Hello World!")
}

fn say_hello_world(message: &str) {
    println!("{}", message);
}

fn say_hello_to(to_person: String) -> String {
    format!("Hello {}!", to_person)
}
