pub fn show() {
    println!("################# variables #################");

    let first_name: &str = "John";

    #[allow(unused_assignments)]
    let mut last_name: &str = "Doe";
    last_name = "Hesami";

    let age: u8 = 35;

    let _population = 62_000_000u32;

    let ptr: *const u8 = last_name.as_ptr();

    // Hex
    let _red = 0xFA;
    let _rgb = 0xFF0000;

    // Float
    let float_number = 4.5;
    let float_number2 = 4.5f32;

    let _distance = float_number + float_number2;

    // Variable shadowing
    let _data = "Bar";
    let _data = 100;

    {
        let _data = _data.to_string();
        assert_eq!(String::from("100"), _data);
    }

    // Constant
    const MY_AGE: u8 = 35;

    let _my_age = MY_AGE;

    // Tuples
    let tuple = ("Rasoul", 32u32);
    let (name, age2) = tuple;

    println!("Name: {}, Age: {}", name, age2);

    println!(
        "Your name is {} {} at age {} in {:?} location!",
        first_name, last_name, age, ptr
    );
    println!("################# variables #################");
}
