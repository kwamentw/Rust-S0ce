use std::collections::HashMap;

fn main() {
    // data types;
    // signed integers: i8, i16, i32, i64,i128
    let a: i32 = -10;
    let b: i32 = -15;

    // unsigned integers: u8, u16, u32, u64, u128
    let c: u32 = 34;
    let d: u32 = 2342;

    //float: f64, f32
    let e: f64 = 1.2232;

    // character: char -- used for a single character
    let character: char = 'D';

    // boolean: bool -- used for true or false asserions
    let affirm: bool = false;

    // tuple: representation:{:?}| is used for grouping different datatypes
    let set: (i32, bool, u32, f64) = (-2, true, 454, 0.8877);

    println!(
        "Hello, world! {} {} {} {} {} {} {} {:?}",
        a, b, c, d, e, character, affirm, set
    );

    // Array: [datatype;size]=[elements]; | re: {:?}|use to group items of the same datatype
    let ages: [u8; 4] = [23, 43, 5, 2];
    println!("Your ages are: {:?} ", ages);

    // String: &str | used for grouping chars | its the normal strings we know
    let name: &str = "Kukuyetti Safachi";
    println!("Your name is: {}", name);

    // vector: let mut nameofvariable: Vec<datatype> = vec![elements]; | used for grouping same data types and it is dynamic
    let mut height_in_feet: Vec<f64> = vec![4.7, 5.77, 6.1, 5.9, 5.89, 5.95];
    println!("Heights => {:?}", height_in_feet);

    height_in_feet.push(7.112);
    println!("Refreshed list of heights => {:?}", height_in_feet);

    // HashMap: Is a keyValue pair data structure like solidity
    // HashMap let mut <mapName>: HashMap<keyDatatype,valueDatatype> = std::collections::HashMap::new();
    let mut name_to_age: HashMap<&str, u32> = std::collections::HashMap::new();
    name_to_age.insert("BiqDawq", 12);
    name_to_age.insert("safachi", 45);
    name_to_age.insert("Kukugulu", 34);
    println!("name|age: {:?}", name_to_age);
}
