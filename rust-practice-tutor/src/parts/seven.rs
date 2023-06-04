pub fn optionals() {
    println!("################# optionals #################");
    let _value = Some(10);
    let _name1: Option<&str> = None;
    let _name: Option<&str> = Some("John Doe");

    match _name {
        Some(name) => println!("Hello {}!", name),
        None => println!("There is no name!"),
    }

    // Unwrap unsafely
    // let unwrapped_name = _name1.expect("Name was not provided!");
    // println!("unwrappd name is {}!", unwrapped_name);

    let unwrapped_name2 = _name.unwrap();
    println!("unwrappd name is {}!", unwrapped_name2);

    //
    let mut _age: Option<i8> = Some(35);

    match _age.as_mut() {
        Some(age) => *age += 10,
        None => println!("No age!"),
    }

    println!("Age is {}!", _age.unwrap());

    let mut _age1: Option<i8> = Some(15);
    let mut _age2: Option<i8> = Some(25);
    let mut _age3: Option<i8> = Some(35);

    if let (Some(_age1), Some(_age2), Some(_age3)) = (_age1, _age2, _age3) {
        println!("{}, {}, {}", _age1, _age2, _age3);
    }

    let name: Option<&str> = None;

    let unwrapped_name3 = name.unwrap_or("Jane Doe");
    // let unwrapped_name4 = name.unwrap_or_else(|| {
    //     // Do some code!
    //     "John Doe"
    // });

    println!("{}", unwrapped_name3);
    // println!("{}", unwrapped_name4);

    if name.is_some() {
        println!("There is value")
    } else {
        println!("There is no value")
    }

    let age5: Option<i32> = Some(12);
    // let age5 = age5.unwrap_or_default();

    // println!("{}", age5);

    let age_multiplied_by_2 = age5.map(|age| age * 2);
    println!("{}", age_multiplied_by_2.unwrap_or_default());

    println!("################# optionals #################");
}
