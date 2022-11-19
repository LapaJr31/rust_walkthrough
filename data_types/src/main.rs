use std::io;

fn main() {

    // numbers
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    //Floating numbers

    let x = 2.0;

    let y: f32 = 3.0;
    println!("{x} {y}");

    //Operations

    let sum = 5 + 10; // addition

    let dif = 97.2 - 10.2; // subtraction

    let prod = 4 * 22; // multiplcation

    let quo = 56.2 / 32.2;
    let floo = 2 /3; // results in 0

    let rem = 43 % 5; // remainder

    println!("{sum} {dif} {prod} {quo} {floo} {rem}");

    //Booleans

    let t = true;
    
    let f = false; // with explicit type annotation

    println!("{t} {f}");

    // Character

    let c = 'z';
    let z = 'Z'; //with explicit type annotation
    let heart_eyed_cat  = '😻';

    println!("{c} {z} {heart_eyed_cat}");


    // Tuple types
    let tup:(i32, f64, u8) = (500, 6.4, 1);

    let (b, n, m) = tup;

    println!("{b}, {n}, {m}");

    let tupp = (600, 7.2, 3);
    let (p, o, i) = tupp;

    println!("{p}, {o}, {i}");

    //Arrays

    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("Array{:?}", arr);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

              println!("Months{:?}", months);
    
             

              
     let arrr = [1, 2, 3, 4, 5];
              
     println!("Please enter an array index.");
              
    let mut index = String::new();
              
    io::stdin()
      .read_line(&mut index)
      .expect("Failed to read line");
              
    let index: usize = index
      .trim()
      .parse()
      .expect("Index entered was not a number");
              
    let element = arrr[index];
              
    println!("The value of the element at index {index} is: {element}");
              
              


    }

 