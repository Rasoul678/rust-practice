pub fn enumerations() {
    println!("################# enumerations #################");
    let fluffy = AnimalType::Dog;
    #[allow(clippy::excessive_precision)]
    const PI: f32 = 3.14159265358979323846264338327950288;

    println!("{:#?}", fluffy);

    if fluffy == AnimalType::Dog {
        println!("Fluffy is a dog!");
    }

    // match
    match fluffy {
        AnimalType::Dog => println!("Woof!"),
        AnimalType::Cat => println!("Meow!"),
        // AnimalType::Rabbit => println!("Hoot!"),
        _ => println!("Something else!"),
    }

    // Associated values
    #[allow(dead_code)]
    enum Shapes {
        Circle { radius: f64, center: (f64, f64) },
        Rectangle { width: f64, height: f64 },
    }

    #[allow(unused_variables)]
    let circle = Shapes::Circle {
        radius: 100.0,
        center: (0.0, 0.0),
    };

    let rectangle = Shapes::Rectangle {
        width: 35.5,
        height: 50.0,
    };

    if let Shapes::Rectangle { width, height } = rectangle {
        println!("Width: {}, Height: {}", width, height);
    }

    match rectangle {
        Shapes::Rectangle { width, height } => {
            println!("Width: {}, Height: {}", width, height);
        }
        _ => println!("Not a rectangle"),
    }

    // Unnamed associated values

    #[allow(dead_code)]
    struct Size {
        width: f32,
        height: f32,
    }

    #[allow(dead_code)]
    enum Shapes2 {
        Rectangle(f32, f32, Size),
        Circle(f32, f32, f32),
    }

    impl Shapes2 {
        fn area(&self) -> f32 {
            match self {
                Shapes2::Circle(_x, _y, radius) => PI * radius * radius,
                Shapes2::Rectangle(_x, _y, size) => size.width * size.height,
            }
        }
    }

    let rectangle2 = Shapes2::Rectangle(
        1.0,
        2.0,
        Size {
            width: 20.0,
            height: 30.0,
        },
    );

    if let Shapes2::Rectangle(x, y, Size { width, height }) = rectangle2 {
        println!("{}-{}-{}-{}", x, y, width, height);
    }

    match rectangle2 {
        Shapes2::Rectangle(x, y, Size { width, height }) => {
            println!("{}-{}-{}-{}", x, y, width, height);
        }
        _ => println!("Not Rectangle!"),
    }

    let area2 = rectangle2.area();
    println!("area: {}", area2);

    // Match is an expression
    #[allow(dead_code)]
    enum Pet {
        Cat { name: String },
        Dog { name: String },
    }

    let fluffy = Pet::Cat {
        name: "fluffy".to_string(),
    };

    let name = match fluffy {
        Pet::Cat { name } => name,
        Pet::Dog { name } => name,
    };

    println!("Hello, {}", name);

    println!("################# enumerations #################");
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum AnimalType {
    Dog,
    Cat,
    Rabbit,
}
