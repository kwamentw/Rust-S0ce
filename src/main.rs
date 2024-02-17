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

    // Looping over an iterator
    let array = [100, 200, 300, 400, 500];
    for (index, value) in array.iter().enumerate() {
        println!("Value at index {}: {}", index, value);
    }

    //Predicate loop pattern
    let mut x: Vec<i32> = vec![1, 2, 3];

    while let Some(y) = x.pop() {
        println!("y = {}", y);
    }

    // Match is similar to switches in other languages
    // match value {
    //     pattern => code,
    //     pattern => code,
    //     ...
    //     _ => code, // _ is a catch-all pattern
    //     }

    let nigga: i32 = 4;
    match nigga {
        1 => println!("He is the man"),
        2 => println!("he the assistant"),
        3 => println!("he is a follower"),
        4 => println!("He is the real niqqa"),
        _ => println!("Don't take any of this too serious"),
    }

    enum size {
        Big,
        Small,
        Medium,
        BSM(u8, u8, u8), // tuple variant
    }
    let Size: size = size::BSM(0, 0, 255);
    match Size {
        size::Big => println!("This is a big house"),
        size::Small => println!("This is a small house"),
        size::Medium => println!("The size of the house is in between"),
        size::BSM(b, s, m) => println!("BSM: {} {} {}", b, s, m),
    }

    let bottle: i32 = 3;
    match bottle {
        0..=5 => println!("You have drunk {} bottles", bottle),
        _ => println!("You have either drunk more or no bottles"),
    }

    let pair: (i32, i32) = (0, -2);
    match pair {
        (0, y) => println!("First is 0 and other is {:?}", y),
        (x, 0) => println!("first is {} and other is 0", x),
        _ => println!("This didnt match the pattern expected"),
    }

    //// FUNCTIONS
    // how functions are declared
    // fn function_name(parameter_list) -> return_type {// code}
    greet();
    print_number(5233);
    print_numbers(23, 44, 23.45333);
}

fn greet() {
    println!("Maakye ooo brothers of zion");
}
fn print_number(n: u32) {
    println!("I am printing {}", n);
}
fn print_numbers(a: u32, b: u32, c: f64) {
    println!("The numbers are: {} {} {}", a, b, c);
}
