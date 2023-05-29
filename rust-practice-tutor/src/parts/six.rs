use ::std::collections::HashMap;

pub fn collections() {
    println!("################# collections #################");
    // Tuple
    // Unnamed grouped data
    let values = ("Hello", "World", 30);
    // let hello = values.0;
    // let world = values.1;
    // let age = values.2;
    let (hello, world, age) = values;
    println!("{}-{}-{}", hello, world, age);

    let (hello1, world1, age1) = get_values();
    println!("{}-{}-{}", hello1, world1, age1);

    //Vectors
    let vector_values: [&str; 2] = ["Foo", "Bar"];

    for value in vector_values.iter() {
        println!("{}", value);
    }

    let foo_value = &vector_values[0];
    println!("{}", foo_value);

    let lenght = vector_values.len();
    println!("{}", lenght);

    let numbers: [i32; 2] = [10, 20];
    let doubled = numbers.iter().map(|x| x * 2);

    for number in doubled {
        println!("{}", number);
    }

    let mut vec_numbers = vec![1, 23, 4, 5, 67, 8];
    vec_numbers.push(90);
    let ninety = vec_numbers.pop();

    println!("{:#?}", ninety);

    vec_numbers.clear();

    println!("{:#?}", vec_numbers);

    vec_numbers.extend_from_slice(&[69]);

    println!("{:#?}", vec_numbers);

    if vec_numbers.contains(&69) {
        println!("Yes!")
    } else {
        println!("No!")
    }

    vec_numbers.clear();

    if vec_numbers.is_empty() {
        println!("Vector is empty!")
    }

    // Hashmaps (dictionaries)
    let mut hash_value: HashMap<&str, &str> = HashMap::new();

    hash_value.insert("Foo", "Bar");
    println!("{:#?}", hash_value);

    if hash_value.contains_key("name") {
        println!("Name exists")
    } else {
        println!("No name")
    }

    // hash_value.remove("Foo");
    println!("{:#?}", hash_value);

    // Unsafe read value
    let bar = hash_value["Foo"];
    println!("bar 1 is: {}", bar);

    // Safe read value
    match hash_value.get("Foo") {
        Some(value) => println!("bar 2 is: {}", value),
        _ => println!("Not Found"),
    }

    for (&k, &v) in &hash_value {
        println!("key: {}, value: {}", k, v);
    }

    let entry = hash_value.entry("Foo");
    println!("{:?}", entry);

    // match entry {
    //     std::collections::hash_map::Entry::Occupied(value =>  {
    //         println!("{}", value.get());
    //     }),
    //     _ => println!("Not Found!"),
    // }

    hash_value.insert("name", "John Doe");
    hash_value.entry("name").or_insert("Jane Doe");
    hash_value.entry("name2").or_insert("Another name");

    // Inserting custom structs into HashMap
    let mut _person_hash: HashMap<Person, &str> = HashMap::new();

    // Iterators
    let values_vec = vec![1, 2, 3, 4, 5, 6, 7];
    for value in values_vec.iter() {
        println!("{}", value)
    }

    let iter = values_vec.iter();
    let _sum: i32 = iter.sum();
    // let sum2: i32 = iter.sum();

    let map_vl: Vec<i32> = values_vec.iter().map(|v| v * 2).collect();
    println!("{:?}", map_vl);

    // Unowned iterators
    let names = vec!["Jane", "Jack", "Bob"];
    for name in names.iter() {
        if name.len() != 3 {
            continue;
        }

        // if name.len() == 3 {
        //     break;
        // }

        println!("{}", name);
    }

    // Owned iterators
    // for name in names.into_iter() {
    //     println!("{}", name);
    // }

    // Filter
    for name in names.into_iter().filter(|name| name.len() == 3) {
        println!("Filtered: {}", name);
    }
    println!("################# collections #################");
}

fn get_values() -> (String, String, i32) {
    ("Hello1".to_string(), "World1".to_string(), 31)
}
#[derive(Hash, Eq, PartialEq, Debug)]
struct Person {
    name: String,
    age: u8,
}
