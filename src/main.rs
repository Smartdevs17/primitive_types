// fn main() {
//     let x: u8 = 10;
//     let y: u8 = 20;
//     let z: i64 = 45003923;
//     let sum: u8 = x + y;
//     let is_snowing: bool = true;
//     println!("x = {}, y = {}, z = {}, sum = {}", x, y, z, sum);
//     println!("is_snowing = {}", is_snowing);
// }


//comound Data Types
//Tuple, Array, Struct, Enum, Vector, Option, Result

//Main function is our entry point to the program and it contains the code that is executed when the program is run
//fn is a function keyword
//main is the name of the function
//() is the parameter list
//-> is the return type
//{} is the body of the function
fn main() {
    //Tuple is a collection of values of different types
    let x: (u8, bool, f64, &str) = (10, true, 10.5, "Hello");
    println!("x = {:?}", x);

    //Array is a collection of values of the same type
    let _y: [i32; 5] = [1, 2, 3, 4, 5];
    println!("y = {:?}", _y);

    // Struct is a collection of values of different types
    #[derive(Debug)] // Add this line to derive the Debug trait
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };
    println!("user1 = {:?}", user1);

    //Arry of Strings
    let a: [String; 3] = [
        String::from("Apple"),
        String::from("Banana"),
        String::from("Cherry"),
    ];
    println!("a = {:?}", a);

    //Array of References to Strings
    let fruits: [&str; 5] = [ "Apple", "Banana", "Cherry", "Date", "Elderberry"];
    println!("fruits = {:?}", fruits);
    println!("fruits[0] = {}", fruits[0]);
    println!("fruits[1] = {}", fruits[1]);
    println!("fruits[2] = {}", fruits[2]);
    println!("fruits[3] = {}", fruits[3]);
    println!("fruits[4] = {}", fruits[4]);

    //Slice is a view into a string
    let s = String::from("Hello, world!");
    let hello = &s[0..5];
    let world = &s[7..12];
    println!("hello = {}, world = {}", hello, world);

    //Differences between String and &str
    //String is a growable, heap-allocated data structure
    //&str is an immutable view into a string
    //String is a type, &str is a type
    //String [growable, heap-allocated, mutable, owned string type]
    //&str [immutable, view into a string, can be coerced into a String]
    //Example of a String
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("s = {}", s);

    //Example of a &str
    let s = "Hello, world!";
    println!("s = {}", s);

    //Slices are a type of reference
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice = {:?}", slice);
    let b: &[i32] = &a[..];
    println!("b = {:?}", b);
    let c: &[i32] = &a[..3];
    println!("c = {:?}", c);
    let d: &[i32] = &a[2..];
    println!("d = {:?}", d);

    let animals = ["Dog", "Cat", "Bird", "Rabbit"];
    let first = animals[0];
    let second = animals[1];
    println!("first = {}, second = {}", first, second);

    //Slices are a type of reference
    let s = String::from("Hello, world!");
    let hello = &s[0..5];
    let world = &s[7..12];
    println!("hello = {}, world = {}", hello, world);

    //Animals slices
    let animals: &[&String] = &[&"Dog".to_string(), &"Cat".to_string(), &"Bird".to_string(), &"Rabbit".to_string()];
    println!("animals = {:?}", animals);

    let mut say_something: String = String::from("Say somethinng");
    say_something.push_str(" to the world");
    println!("I told you to {}", say_something);

    let string: String = String::from("Hello World");
    let slice: &str = &string;
    println!("Slice value: {}", slice);

    say_hello();
    say_goodbye();
    print_value(say_something);
    print_slice(animals);

    let sum = add_number(10, 20);
    println!("sum = {}", sum);

    let sub = sub_number(10, 5);
    println!("sub = {}", sub);

    let sub_1 = sub_number(3, 5);
    println!("sub_1 = {}", sub_1);

    let mul = mul_number(5, 2);
    println!("mul = {}", mul);

    let div = div_number(10.0, 2.0);
    println!("div = {}", div);

    let div_1 = div_number(10.0, 3.0);
    println!("div_1 = {}", div_1);

    human_identity(String::from("John"), 20, 170, 60);  
}


//functions
fn print_value(value: String) {
    println!("===========Value: {}", value);
}

fn print_slice(slice: &[&String]) {
    println!("===========Slice: {:?}", slice);
}

fn say_hello() {
    println!("Hello");
}

fn say_goodbye() {
    println!("===========>>>>>>>>>Goodbye");
}

fn add_number( a: u32, b: u32) -> u32{
    return a + b;
}


fn sub_number( a: i32, b: i32) -> i32{
    return a - b;
}

fn mul_number( a: u32, b: u32) -> u32{
    return a * b;
}

fn div_number( a: f32, b: f32) -> f32{
    return a / b;
}

fn human_identity(name: String, age: u8, height: u32, weight: u32) {
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Height: {}", height);
    println!("Weight: {}", weight);
}

