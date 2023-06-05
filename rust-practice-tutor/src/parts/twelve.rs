use std::ops::AddAssign;

pub fn generics() {
    println!("################# generics #################");
    let mut p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 0.8, y: 0.0 };
    let _p3 = Point { x: "Foo", y: "Bar" };

    p1.move_offset(1.0, 2.0);

    println!("{:?}", p1);

    // p1 += p2;

    println!("{:?}", p1);

    // partial Eq
    if p1 == p2 {
        println!("p1 and p2 are equal")
    } else {
        println!("Not equal")
    }

    let people = vec![
        Person {
            name: "John".to_string(),
        },
        Person {
            name: "Jane".to_string(),
        },
    ];

    people.run();
    people.walk();

    let elephants = vec![Elephant {
        name: "elephant".to_string(),
    }];

    elephants.walk();

    println!("################# generics #################");
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn move_offset(&mut self, x: T, y: T)
    where
        T: AddAssign,
    {
        self.x += x;
        self.y += y;
    }
}

impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

trait CanRun {
    fn run(&self);
}

trait CanWalk {
    fn walk(&self);
}

impl<T: CanRun> CanRun for Vec<T> {
    fn run(&self) {
        for item in self {
            item.run();
        }
    }
}

impl<T: CanWalk> CanWalk for Vec<T> {
    fn walk(&self) {
        for item in self {
            item.walk();
        }
    }
}

struct Person {
    name: String,
}

impl CanRun for Person {
    fn run(&self) {
        println!("{} is running!", self.name)
    }
}

impl CanWalk for Person {
    fn walk(&self) {
        println!("{} is walking!", self.name)
    }
}

struct Elephant {
    name: String,
}

impl CanWalk for Elephant {
    fn walk(&self) {
        println!("{} is walking!", self.name)
    }
}
