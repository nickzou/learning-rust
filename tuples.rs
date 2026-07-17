fn main() {
    let humans:(String, i32, bool) = ("Alice".to_string(), 30, true);
    println!("Human: {:#?}", humans);

    let my_mixed_tuples = ("Something", "Something Else", 33, true, [3,4,5,5]);
    println!("Human: {:#?}", my_mixed_tuples);
}
