fn main() {
    let mut some_string:String = String::from("some value");
    println!("This is the value of the string: {:?}", some_string);
    some_string.push_str(" some extra");
    println!("To see extra: {:?}", some_string);
}
