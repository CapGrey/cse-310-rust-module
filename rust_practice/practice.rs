fn main() {
    println!(); // prints a newline
    println!("Hello "); // prints hello
    // prints formats some arguments
    println!("formats {} arguments", "some"); 

    let company_string = "TutorialsPoint"; // sting
    let rating_float = 4.5; // float
    let is_growing_boolean = true; //boolean
    let icon_char = '♥'; // unicode character char

    println!("company Name: {}", company_string);
    println!("rating: {}", rating_float);
    println!("growing: {}", is_growing_boolean);
    println!("company Icon: {}", icon_char);

    // int can be specified:
    let _default_num = 10; // i32 by default.
    let _unsigned_32:u32 = 20; //unsigned 32 bits
    let _signed_8:i8 = 2; // signed 8 bits
    /* signed int size determined by machine architecture:
        32 bits for x86 and 
        64 bits for x64 */
    let _signed_arch:isize = 300; 
    let _unsigned_arch: usize = 390; // unsigned based on machine

    // when you go out of range, an overflow error is thrown
    // let weight:u8 = 256; // overflow value is 0
    //let height:u8 = 257; // overflow value of 1

    // floats can be specified:
    let _default_float = 10.00;  //f64 by default double precision
    let _single_precision:f32 = 8.35; // single precision
    let _double_precision:f64 = 1600.6000; // double precision

    // when working with numeric types, cannot automatically typecast
    //let error_float = 8; results in an error
    let _float_valid = 8.0; // this will work
    
    // to better read numbers, you can use _ to like commas
    let float_with_separator = 11_000.555_001; // same as 11000.555001
    // this only improves readability, no actual change happens.
    println!("Float with separator: {}", float_with_separator);

    // booleans only have two options: true or false
    let _bool_var:bool = true;

    // Rust uses Unicode rather than ASCII, this allows for more options
    let _special_character = '@'; //default
    let _alphabet:char = 'A';
    let emoji:char = '😁';
    println!("{}", emoji);


    // variables can or can not be specified.
    let _auto_name = "name";
    let _specified_char:char = 'a';

    // by default, variables are immutable
    // to make them mutable, use mut keyword
    let mut mutable_variable = 45;
    println!("Before change: {}", mutable_variable);
    mutable_variable = 22;
    println!("After: {}", mutable_variable);

    // constants are all CAPS and use const and need to be
    // explicitly typed
    const CONSTANT_VARIABLE:i32 = 3004;
    // const must be hardcoded, whereas immutable variables can have
    // their value determined from a function call/ calculation.

    // shadowing is when a variable can have the same name as another.
    // the second variable overrides the first one.
    let _shadow = 300;
    // the second variable can change data types
    let _shadow = "three-hundred";
    // constants cannot be shadowed.

    // strings come in two types: literals and objects
    // literals are hardcoded:
    let _hard_string:& str = "Hard coded";

    // objects are more complicated. To create a new object
    //  to create an empty string:
    String::new();

   // to create a string object from a literal
    String::from("Needs a string parameter");

    // example:
    let empty_string = String::new();
    println!("Length is {}", empty_string.len());

    let content_string = String::from("TutorialsPoint");
    println!("Length is {}", content_string.len());

    // string methods:
        // new() empty string
        let mut z = String::new();
        z.push_str("hello");
        println!("{}", z);

        // to_string() must be used to convert literal to object
            // this gives access to all string methods
        let name1 = "Hello World!".to_string();
        println!("{}", name1);

        // replace() replaces all instances of a pattern with a new one
        let name2 = name1.replace("Hello", "Howdy");
        println!("{}", name2);

    // string concatenation
        let n1 = "Hello".to_string();
        let n2 = "World".to_string();

        let n3 = n1 + &n2; // second variable is passed by reference
        println!("{}", n3);

        // convert number to string
        let number = 2002;
        let num_as_str = number.to_string();

        // concatenation with format! macro
        let n1 = "Hello".to_string();
        let n3 = format!("{}, {}", n1, n2);
        println!("{}", n3);

        // -- and ++ are not supported

    // conditionals:
        // if statements
        let num = 5;
        if num > 0 {
            println!("number is positive");

        } else if num < 0 {
            println!("Number is negative");
        }

        // match (switch case) statements
        let state_code = "HH";
        let state = match state_code {
            "MH" => "Mohanda",
            "PJ" => "Pensuljerry",
            // underscore is used for default case
            _ => "Unknown"
        };

        println!("{}", state);

    
    // loops
        // for loop can be used when the number of loos is known
        for x in 1..11 { // 11 is not included
            if x == 5 {
                continue;
            }
            println!("x is {}", x);
        }

        // indefinite loops use while or loop
        let mut x = 0;
        while x < 10 {
            x += 1;
            println!("Inside loop x is {}", x);
        }
        println!("Outside loop x is {}", x);

        // loop requires break
        let mut x = 0;
        loop {
            x += 1;
            println!("x={}", x);

            if x == 15 {
                break;
            }
        }
    // passing number values remain, but passing strings destroys
    // sting object after passing: the function kind of steals it.

    let num1 = 5;
    let num2 = 6;
    let num3 = num1 + num2;
    println!("sum: {}", num3);
    println!("test: {}", num1);



    // functions
    let mut num = 4;
    mutate_no_to_zero(num);
    // passing by value does not change the value outside the function
    println!("The value of num is: {}", num);

    let mut num = 55;
    pass_by_ref(&mut num);
    println!("The value of num is: {}", num);

    // tuples cannot change size once allocated
    let tuple_name:(i32, i32, &str) = (300, 200, "Money");
    let tuple_name = (300, 200, "money"); // shorthand

    //to print a tuple, use the following syntax
    println!("{:?}", tuple_name);
    
    // or for individual lines:
    println!("{:?}", tuple_name.0);
    println!("{:?}", tuple_name.1);

    // tuples are passed by value
    let b = (110, true, 10.9);
    print(b);

    // destructing = unpacking a tuple
    let (age, is_male, rating) = b;
    println!("Age: {}, is male: {}, rating: {}", age, is_male, rating);
    print(b);

    // arrays
    let array_name = [1, 2, 3]; // data type auto assigned
    let array_name:[i32; 5]; // data type; size
    let array_name:[i32; 5] = [0; 5]; // = [default value, size]
    
    // looping through an array
    for index in 0..array_name.len()
    {
        println!("index is: {} and value is: {}", 
        index, array_name[index]);
    }

    // looping with iter()
    for val in array_name.iter()
    {
        println!("Value is {}", val);
    }

    // mut is used to make the array mutable, but cannot change size
    let mut array_name = [10, 20, 30];
    array_name[1] = 0;
    
    // arrays can be passed by value or by reference
        // by value
        let mut array_3 = [20, 30, 20];
        update(array_3);
        println!("{:?}", array_3); // does not change array

        // pass by reference
        update_by_ref(&mut array_3);
        println!("{:?}", array_3); // doeschange array

        // variables cannot be used to determine array sizes
        // let n = 20;
        // array_name:[usize; n]; this will result in an error
        const N: usize = 20; // const works though
        let arr:[usize; N] = [0; N];
    
    // ownership
        // primative data types are copied, not moved.
        // heap matters: things only known at run time
        let mut val = 30;
        let mut val2 = val;
        println!("{}", val); // doesn't throw error because its in the stack
        
        /*
        let v = vec![1, 2, 3];
        v2 = v;
        println!("{:?}", v); error thrown in this case

        the same thing happens when passign by value to a function.
        display(v2);
        println!("{:?}", v2); error thrown: value has been moved
        */

        // you can return ownership back with a return in a function
        let mut v = vec![1, 2, 3];
        v = display(v);
    
    // returning ownership becomes frustrating. Borrowing provides
    // a better option
    // done by passing by reference
    let v = vec![10, 20, 30];
    print_vec(&v); // print_vec is only borrowing v
    println!("{:?}", v); 
    println!("{:?}", v); // macros do not take ownership

    let mut name:String = String::from("John");
    display2(&mut name);
    println!("{}", name);

}   

fn display2(name: &mut String)
{
    println!("in display2: {}", name);
    name.push_str(" Rocks");
}

fn print_vec(x:&Vec<i32>)
{
    println!("iside print_vec {:?}", x);
}

// returns in fuctions must be specified:
fn get_pi() -> f64 {
    return 22.0 / 7.0;
}
 // shorthand: statment without semi-colon is returned
 fn get_pi_shorthand() -> f64 {
    22.0 / 7.0
 }

 // parmaeters are similar:
 fn mutate_no_to_zero(mut param_no: i32) {
    param_no = 0;
 }

 fn pass_by_ref(param_no:&mut i32) {
    *param_no = 0;
 }

 fn print(x:(i32, bool, f64))
 {
    println!("{:?}", x);
 }

 fn update(mut arr:[i32;3])
{
    for i in 0..3
    {
        arr[i] = 0;
    }
}
fn update_by_ref(arr:&mut [i32; 3])
{
    for i in 0..3
    {
        arr[i] = 0;
    }
}

fn display(v:Vec<i32>) -> Vec<i32>
{
    println!("inside display {:?}", v);
    return v;
}