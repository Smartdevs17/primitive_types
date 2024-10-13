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

fn is_login(username: String, password: String) -> bool{
    if(username == "admin" && password == "testpassword"){
        return true;
    }
    return false;
}


//main function and is the entry point of the program
//fn is used to define a function
//main is the name of the function
//() is the parameter list of the function
//{} is the body of the function
///any function or variable should be written in Snake Case
///Hoisting is a feature in Rust that allows you to declare a function before its usage
fn main(){

    say_hello();
    say_goodbye();

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

    let is_logged_in = is_login(String::from("admin"), String::from("testpassword"));
    println!("is_logged_in = {}", is_logged_in);

    let is_logged_in_1 = is_login(String::from("admin"), String::from("wrongpassword"));
    println!("is_logged_in_1 = {}", is_logged_in_1);

    let stock_value: u32 = {
        let price: u32 = 100;
        let quantity: u32 = 5;
        price * quantity
    };
    println!("stock_value result = {}", stock_value);

}