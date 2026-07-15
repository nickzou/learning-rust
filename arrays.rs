fn main() {
    let numbers:[i32;5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);

    let fruits:[&str;3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("First Fruit: {:?}", fruits[0]);
}

