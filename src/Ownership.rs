
//FIRST RULE OF OWNERSHIP
// fn main(){
//     let s1 = String::from("RUST IS THE BEST MEMORY MANAGEMENT LANGUAGE");
//     let len = string_length(&s1);
//     println!("The length of the string is {}", len);
// }

// fn string_length(s: &String) -> usize {
//     s.len()
// }

//SECOND RULE OF OWNERSHIP
// the owner of the value must be in scope
// fn main(){
//     let s1 = String::from("RUST IS THE BEST MEMORY MANAGEMENT LANGUAGE");
//     let s2 = s1;
//     let len = string_length(&s2);
//     println!("The length of the string is {}", len);
// }

// fn string_length(s: &String) -> usize {
//     s.len()
// }

//THIRD RULE OF OWNERSHIP
//when a variable goes out of scope, the value is dropped
fn main(){
    let s1 = String::from("RUST IS THE BEST MEMORY MANAGEMENT LANGUAGE");
    let s2 = s1;
    let len = string_length(&s2);
    println!("The length of the string is {}", len);
}

fn string_length(s: &String) -> usize {
    s.len()
}

fn print_string(some_string: &String){
    println!("{}", &s1s);
}

