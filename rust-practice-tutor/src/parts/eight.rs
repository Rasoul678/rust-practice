pub fn errors() -> Result<(), Box<dyn std::error::Error>> {
    println!("################# errors #################");
    let ok_value: Result<&str, Box<dyn std::error::Error>> = Ok("This is Ok!");

    match ok_value {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error),
    }

    let err_value: Result<&str, ()> = Err(());
    match err_value {
        Ok(value) => println!("{}", value),
        Err(()) => println!("Some error occurred"),
    }

    let _user_name = get_user_name().expect("Failed to get user name");
    // get_user_name().expect_err("I did not expect user name");

    let is_ok = get_user_name().is_ok();
    let is_err = get_user_name().is_err();

    println!("is Ok: {}, is error: {}", is_ok, is_err);

    let _full_name = get_full_name()?;

    // match full_name {
    //     Ok(name) => println!("{}", name),
    //     Err(()) => println!("Error"),
    // }

    // let length = full_name.map(|s| s.len()).unwrap_or_default();
    // println!("{:?}", length);

    // let err_len = full_name.map_err(|e| e.len());
    // println!("{:?}", err_len);

    
    println!("################# errors #################");
    Ok(())
}

fn get_user_name() -> Result<String, String> {
    Ok("John".to_string())
}

fn get_first_name() -> Result<String, String> {
    // Ok("John".to_string())
    Err("I do not know the first name".to_string())
}

fn get_last_name() -> Result<String, String> {
    Ok("Doe".to_string())
}

fn get_full_name() -> Result<String, String> {
    let first = get_first_name()?;
    let last = get_last_name()?;

    Ok(format!("{} {}", first, last))
}
