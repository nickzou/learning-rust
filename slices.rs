fn main() {
    //untyped/inferred type number slice
    let number_slice = &[1,2,3,4,5];

    println!("Untyped/Inferred Type Slice: {:?}", number_slice);

    //explicitedly typed slice
    let named_number_slice:&[i32] = &[1,2,3,4,5];
    println!("Explicitedly Typed Slice: {:?}", named_number_slice);

    //String slice... slice
    let string_slice_slice:&[&str] = &["strings", "now", "bitch"];
    println!("String Slice... slice: {:?}", string_slice_slice);
    //
    //String slice
    let string_slice:&[String] = &["strings".to_string(), "not slices".to_string(), "bitch".to_string()];
    println!("String Slice: {:?}", string_slice);
}
