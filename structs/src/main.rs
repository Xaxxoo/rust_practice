// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     println!("Hello, world!");

//     //Tuple Struct
//     struct Color (i32, i32, i32);
//     struct Point (i32, i32, i32);

//     let mut user1 = User {
//         email: String::from("user1@gmail.com",),
//         username: String::from("Xaxxoo"),
//         active: true,
//         sign_in_count: 1,

//     };

//     let name = user1.username;
//     user1.username = String::from("Zazoo");

//     let user2: User = build_user(email: String::from("kiwomobileapp@gmail.com"), username: String::from("llins"));

//     let user3 = User {
//         email: String::from("xyz@gmail.com"),
//         username: String::from("xyz"),
//         ...user2
//     }
// }

// fn build_user (email: String, username:String) -> User {

//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }


// fn main() {
// let width1 = 30;
// let height1 = 20;

// println!("The are of the rectangle is {} square pixels", area(width1, height1));

// }

// fn area(width: u32, height: u32) -> u32{
//     width * height
// }

// fn main() {
//     let rect = (30, 50);
//     println!("The are of the rectangle is {} square pixels", area(rect));

// }

// fn area(dimensions: (u32, u32)) -> u32{
//     dimensions.0 * dimensions.1
// }

#[derive(Debug)]
struct Rectangle {
width: u32,
height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle {
width: 30,
height: 50
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50
    };

    let rect3 = Rectangle::square(25);



    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("rect: {:?}", rect);
    println!("rect: {:#?}", rect);

    println!("The are of the rectangle is {} square pixels", rect.area());

}

// fn area(rectangle: &Rectangle) -> u32{
//   rectangle.width * rectangle.height
// }