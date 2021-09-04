/// # Integer types
/// 
/// ```
/// 
/// let myvar1 = 1; //   non mutable variable. By default uses i32 bit
/// let mut myvar2 = 2; //   To make a variable mutable we use the key word mut. By default uses i32
/// # Annotating the datatype
/// let mut a: i8 = 3; //  8 bit mutable variable
/// let mut b: i16 = 4; // 16 bit variable
/// let mut c: i32 = 5; //32 bit
/// let mut d: i64 = 6; // 64 bit variable
/// let mut e: i128 = 7; // 128 bit variable
/// slet mut f: isize = 8; // depends on the machine. On 32 bit machine it uses i32 and on 64 bit uses i64
/// 
/// ```
///              
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 

fn main() {


    let myvar1 = 1; //   non mutable variable. By default uses i32 bit
    let mut myvar2 = 2; //   To make a variable mutable we use the key word mut. By default uses i32

    // Annotating the datatype

    let mut a: i8 = 3; //  8 bit mutable variable
    let mut b: i16 = 4; // 16 bit variable
    let mut c: i32 = 5; //32 bit
    let mut d: i64 = 6; // 64 bit variable
    let mut e: i128 = 7; // 128 bit variable

    let mut f: isize = 8; // depends on the machine. On 32 bit machine it uses i32 and on 64 bit uses i64

    // Floating point numbers

    let mut g: f32 = 9.00000; // 32 bit
    let mut h: f64 = 10.45454; // 64 bit

    // constants

    const MAX_POINTS: u32 = 100_000; // used throughtout the program. Use only uppercase with underscore. Always immutable. Can't use the mut key word.

    // Boolean types

    let i = true; // without type annotation annotation
    let j: bool = false; // with explicit type annotation

    // Character types

    let mychar1 = 'z';
    let mychar2 = 'ℤ';
    let heart_eyed_cat = '😻';

    // Tuple types

    let tup_1 = (2, 5.6565, 23); // wihtout type annotation
    let tup_2: (i32, f64, u8) = (500, 6.4, 1); // with type annotation. Typle values can be accessed using the dot operator
    let (x, y, z) = tup_1; // access tuple values using destructing

    // Arrays. Use array when you want to allocate the data on stack. Use arrays if the data is unlikely to change

    let myarr1 = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5]; // An array of i32 bit type and length of 5.
    let arr3 = [3; 5]; // Array of length 5 with the initial values 3. Which is similar to [3,3,3,3,3,]

    // Referencing and Borrowing

    let s1 = String::from("hello");
    let len = calculate_length(&s1); // & operator is used as a reference to the variable

    println!("The length of '{}' is {}.", s1, len);

    // Mutable references

    let mut s = String::from("hello");
    change(&mut s); // use &mut to make the reference mutable

    println!("{}", &mut a);
    println!("{}", &mut b);
    println!("{}", &mut c);
    println!("{}", &mut d);
    println!("{}", &mut e);
    println!("{}", &mut f);
    println!("{}", &mut g);
    println!("{}", &mut h);
    println!("{}", tup_1.1); // print tuple values
    println!("{}", x); // print tuple values

    // Calling the functions

    another_function(); // calling a function

    yet_another_function(5); // Calling a function with parameter

    let returned_value = function_with_return_value(); // A function call that returns a value

    println!("Returned value is {}", returned_value); // print the returned value
}

// functions

fn another_function() {
    println!("This is another function");
}

fn yet_another_function(x: i32) {
    println!("the value of x is {}", x)
}

fn function_with_return_value() -> i32 {
    2980 // this value is retuned.
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
