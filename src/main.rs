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

    // struct: a custom data type
    // struct <structName> { varname: datatype, varname2: datatype, }
    // struct let <varname>:<structname> = <structname>{<varname>:value,<varname>:value};
    struct CustomVar {
        n: i32,
        a: u32,
    }
    struct Goods<'a> {
        id: &'a str,
        weight: f64,
    }

    let kwame: CustomVar = CustomVar { n: -43, a: 333 };
    let shoes: Goods = Goods {
        id: "NIKE",
        weight: 34.333,
    };
    println!("Here are your details {} {}", kwame.n, kwame.a);
    println!("Your {} goods weighed {}", shoes.id, shoes.weight);

    // Enum ois just the same as solidity's
    // enum <varName> {state1,state2,state3}
    // if you want to display you renum val you have to use #[derive(Debug)] or #[derive(strum_macros::Display)]

    #[derive(Debug)]
    enum Colors {
        Red,
        Green,
        Blue,
        White,
    }
    let new_color: Colors = Colors::Red;
    println!("Your current color is {:?}", new_color);

    // Every variable is immutable by default use the "mut" keyword to make it mutable

    // infinite loop stops when theres a break statement or a condition is met
    let mut counter: i32 = 0;
    loop {
        println!("looping...{}", counter);
        counter += 1;
        if counter >= 5 {
            break;
        }
    }

    //while loop executes till a condition  is met
    let mut b: i32 = 0;
    while b < 5 {
        b += 1;
        println!("looping y'all....{}", b);
    }

    // for loops, for looping through coolections eg array,vectors etc
    let numbers: [i32; 6] = [23, 32, 45, 65, 23, 65];
    for element in numbers.iter() {
        println!("Your elements are {}", element);
    }
    for num in &numbers {
        println!("Your numbers are {}", num);
    }
    // range for loop
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz{}", n);
        }
    }

    // looping over an iterator
    let vec: Vec<i32> = vec![1, 2, 3];
    for (element) in vec.into_iter() {
        println!("Element  is {}", element);
    }

    //Predicate loop pattern
    let mut x: Vec<i32> = vec![1, 2, 3];

    while let Some(y) = x.pop() {
        println!("y = {}", y);
    }
}
