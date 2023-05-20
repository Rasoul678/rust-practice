pub fn structures() {
    println!("################# structures #################");
    let name = "Rasoul".to_string();
    let age = 35;
    let person = Person { name, age };

    // Struct update
    let person2 = Person {
        name: "Doe".to_string(),
        ..person
    };
    println!("{} is {} years old", person.name, person.age);
    println!("{} is {} years old", person2.name, person2.age);

    // Tuple
    let mut point = Point(1.2, 6.7, 8.8);
    println!("X: {}, Y: {}, Z: {}", point.0, point.1, point.2);
    point.make_double();
    point.describe();
    let db_point = point.double();
    println!("X: {}, Y: {}, Z: {}", db_point.0, db_point.1, db_point.2);

    // Debugging
    println!("{:?}", point);

    let _point3 = Point::zero();
    println!("################# structures #################");
}

// Struct
struct Person {
    name: String,
    age: u8,
}

// Tuple
#[derive(Debug)]
struct Point(f64, f64, f64);

// Implementation for structs
impl Point {
    fn describe(&self) {
        println!("X: {}, Y: {}, Z: {}", self.0, self.1, self.2);
    }

    fn double(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }

    fn make_double(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }

    // Non-method functions
    fn zero() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}
