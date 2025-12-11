// // fn main() {

// //     let x:u8 = 5;

// //     helper(x);
// //     println!("Hello, world! {}", x);
// // }

// // fn helper(x: u8){
// //     println!("Helper received: {}", x);
// // }
// fn main() {

//     let x:String = String::from("hello");

//     helper(x); // trasferring ownership of x
//     // println!("Hello, world! {}", x);
// }

// fn helper(x: String){
//     println!("Helper received: {}", x);
// }

// fn swap_integer(a: u8, b: u8) -> (u8, u8){
    
    
// }



// // avoiding ownership by using Tuple

// fn main() {

//     let s1:String = String::from("hello");

//     let (s1, length) = string_length_cal(s1);

//     print!("String: {}, Length: {}", s1, length);
// }


// fn string_length_cal(s: String) -> (String, usize){
//     let length: usize = s.len();
//     (s, length)
// } 


// avoiding ownership by cloning
// fn main() {

//     let s1:String = String::from("hello");

//     let length = string_length_cal(s1.clone());

//     print!("String: {}, Length: {}", s1, length);
// }


// fn string_length_cal(s: String) ->  usize{
//     let length: usize = s.len();
//     return length;
// } 

// avoiding ownership by Borrowing
// fn main() {

//     let mut s1:String = String::from("hello");

//     // for writing in reference we use keyword mut
//     let length = string_length_cal(&mut s1);

//     print!("String: {}, Length: {}", s1, length);
// }


// fn string_length_cal(s: &mut String) ->  usize{

//     s.push_str(" World");
//     let length: usize = s.len();

//     return length;
// } 



// fn main(){
//     let mut s1:String = String::from("hello");



//     let mut r2 = &mut s1;

//     // r2.push_str(" sajid");
//     // print!("r2 {}",r2);

//     let r3 = &mut r2;

//     // r3.push_str(" world");

//     print!("r3 {} r2: {}",r3,r2);
// }


// arra
// fn main () {
//     // let arr:[u8;5]; // array declaration

//     let mut arr;
//     arr = [1,2,3,4,5]; // array initialization

//     println!("{:?}", arr)
// }

// pass array directly to function -> make new copy of orginal array
// fn main () {
//     let arr:[u8;3]  = [1,2,3]; // array initialization
//     helper(arr);
//     println!("{:?}", arr);
// }

// fn helper(mut a:[u8;3]){
//     a[0] = 5;
//     println!("{:?}", a);
// }

// pass array by reference -> borrow orginal array
// fn main () {
//     let mut arr:[u8;3]  = [1,2,3]; // array initialization
//     helper(&mut arr);
//     println!("{:?}", arr);
// }   
// fn helper(a:&mut [u8;3]){
//     a[0] = 5;
//     println!("{:?}", a);
// }

// Vector
// fn main(){
//     // let mut v: Vec<&str> = Vec::new(); // declaration of vector
//     // let mut v = Vec::<&str>::new(); // another way of declaration of vector
//     // let mut v = Vec::new(); // another way of declaration of vector

//     let mut v = vec!["Hello", "This is", "Sajid"]; // declaration and initialization of vector

//     v.push("Hi");
//     v.push("there");

//     helper(&mut v);

//     println!("{:?}", v);
// }

// fn helper(v: &mut Vec<&str>) {
//     v.push("New Element");
//     println!("{:?}", v);
// }


// for loop with array
// fn main(){

//     let arr = [10,20,30,40,50];

//     for i in &arr{
//         println!("Element: {}", i);
//     }

//     println!("Array after loop: {:?}", arr);
// }


// match
// fn main() {

//     fn is_even(n: u8) -> bool {
        
//         if n % 2 == 0 {
//             true
//         } else {
//             false
//         }
//     }   

//     let x = 50;

//     match x {
//         x if is_even(x) => println!("{} is even", x),
//         _ => println!("{} is odd", x),
//     }
// }



// i/o operations

// use std::io;
// fn main(){

//     println!("Enter your name: ");

//     let mut name = String::new();

//     io::stdin()
//         .read_line(&mut name)
//         .expect("Failed to read line");

//     println!("Hello, {}!", name.trim());

// }


fn main() {


    let s1 = String::from("Hello");

    helper(&s1);

    println!("{}",s1);
}

fn helper(s2: &String){ 

    println!("{}",s2);
}

