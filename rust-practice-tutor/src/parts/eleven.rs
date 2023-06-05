use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;

pub fn pointers() {
    println!("################# pointers #################");
    // Box data type (store in heap)
    let age = Box::new(30);
    let twice = *age * 2;
    println!("{}", twice);

    // implement our own Box
    let age2 = BoxedValue::new(35);
    let twice2 = *age2 * 2;
    let _ref_to_value = age2.deref();
    let _actual_value = *(age2.deref()); // shrthand: *age2
    println!("{}", twice2);

    // implicit Deref coersion in functions
    print_integer(&age2);

    // RC pointer (reference counting)
    let array = vec!["John".to_string(), "Jane".to_string()];
    let rc = Rc::new(array);
    let _weak_ref = Rc::downgrade(&rc);

    // drop(rc);

    // let value = _weak_ref.upgrade().unwrap();
    // println!("{:?}", value);

    match _weak_ref.upgrade() {
        Some(rc) => println!("{:?}", rc),
        None => println!("None"),
    }

    // Cloning an Rc
    let rc2 = rc.clone(); // or Rc::clone(rc)
    drop(rc);

    println!("{:?}", rc2);

    // Mutability of Rc
    let person = Person {
        name: "John".to_string(),
        age: Cell::new(35),
    };

    let new_age = person.increment_age();
    let person_age = person.age.get();

    println!("{}", new_age);
    println!("{}", person_age);

    // RefCell for multiple immutable borrows or 1 mutable
    let ref_cell = RefCell::new(vec![1, 2, 3]);
    // mutable
    let mut mut_ref = ref_cell.borrow_mut();
    let length = ref_cell.borrow().len();

    // mut_ref.push(100); // Not good

    print!("Lenght is {}", length);

    println!("################# pointers #################");
}

struct BoxedValue<T> {
    value: T,
}

impl<T> BoxedValue<T> {
    fn new(value: T) -> Self {
        BoxedValue { value }
    }
}

impl<T> Deref for BoxedValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn print_integer(value: &i32) {
    println!("{}", value);
}

struct Person {
    name: String,
    age: Cell<u8>,
}

impl Person {
    fn increment_age(&self) -> u8 {
        self.age.set(self.age.get() + 1);

        self.age.get()
    }
}
