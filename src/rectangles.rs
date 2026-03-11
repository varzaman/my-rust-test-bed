/*
    Structs allow us to meaningfully gather different values together, kind of like an object in a OOP language.
    An alternative is storing this as a tuple, but structs let us have property like names, so we can derive meaning.
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
    Instead of having a separate function, we can change it into a method just like a normal OOP language,
    just with a slightly separate paradigm. If the struct is like our interface, we can assign it functions
    to become methods with implementations, where it can reference itself. &self is an alias for self:&Self, the
    type the implementation block is for.

    The associated functions in the implementation blocks can be split into multiple implementation blocks.
*/

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }

    fn create_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn rectangles() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square1 = Rectangle::create_square(20);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.calculate_area()
    );
    println!("rect1 is {rect1:?}");

    dbg!(&rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("The square is {square1:?}");
}
