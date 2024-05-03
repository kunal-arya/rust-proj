use std::{fs, option};

fn main() {

    // NOTION DOC - https://projects.100xdevs.com/tracks/rust-bootcamp/Rust-Bootcamp-1

    // numbers -> Easy and Faster
    let x: i8 = -12;
    let y: u32 = 1000;
    let z: f32 = 1000.001;

    println!("x: {}",x);
    println!("y: {}",y);
    println!("z: {}",z);

    // boolean -> Easy and Faster
    let is_male = true;
    let is_above_18 = true;

    if is_male {
        println!("You are a male");
    } else {
        println!("You are a female");
    }

    if is_male && is_above_18 {
        println!("You are above 18");
    }

    // strings -> Harder and slower
    // This can change space in runtime
    // That is why Strings will be saved in heap
    let greeting = String::from("Hello world");
    println!("{}", greeting);

    // let str = "asdfadf"; 
    // let char1 = greeting.chars().nth(0);
    // println!("{}", char1.unwrap()); 


    // Conditionals
    let num: i32 = 8;
    
    let is_even = find_even(num);

    if is_even {
        println!("{} is even",num);
    } else {
        println!("{} is odd", num)
    }


    // Loops
    for i in 1..3 {
        println!("i: {}", i);
    }
    /*
     output =>
        i: 1
        i: 2
     */

     // Make something mutable , means you can change it again
     // use 'mut' keyword
     let mut a = 12;
     a = 20;
     println!("a: {}",a);

     // GET First word
     let sentence = String::from("Kunal is my Name");
     let first_word = get_first_word(sentence);

     println!("First word is: {}",first_word);
    //  println!("sentence {}",sentence);

    //  MEMORY IN ACTION

    stack_fn();
    heap_fn();
    update_string();


    println!("");
    println!("");
    println!("Ownership");
    println!("");
    println!("");
    

    // Ownership
    ownership();

    println!("");
    println!("");
    println!("Borrowing");
    println!("");
    println!("");
    
    // Borrowing
    borrowing();

    println!("");
    println!("");
    println!("Struct");
    println!("");
    println!("");

    structs();

    println!("");
    println!("");
    println!("implementing structs");
    println!("");
    println!("");

    rectangle();


    println!("");
    println!("");
    println!("enums");
    println!("");
    println!("");

    enums();

    println!("");
    println!("");
    println!("error_handling");
    println!("");
    println!("");

    error_handling();

    println!("");
    println!("");
    println!("Option enum");
    println!("");
    println!("");

    option_enum();

}

fn find_first_a(s: String) -> Option<usize> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index);
        }
    }
    return None;
}

fn option_enum() {
    let my_string = String::from("kk");

    match find_first_a(my_string) {
        Some(index) => println!("The letter 'a' is found in the index: {}",index),
        None => println!("The letter 'a' not found in the string")
    }
}

fn error_handling() {
    let read_file = fs::read_to_string("example.txt");

    // This "example.txt" can exist or not
    // That's why read_file type is Result<String,Error>
    match read_file {
        Ok(file_content) => {
            println!("These are the content of file {:?}",file_content);
        }
        Err(error) => {
            println!("Failed to read file: {:?}",error);
        }
    }

    // Incase you are ok with runtime errors (crashing the process while it runs if an error happens), then you can unwrap a Result
    // read_file.unwrap();
    
}

enum Direction {
    East,
    West,
    North,
    South
}

enum Shape { 
    Circle(f64),  // Variant with associated data (radius)
    Square(f64), // Variant with associated data (side length)
    Rectangle(f64,f64) // Variant with associated data (width, height)
}


// WHY ENUMS, because it will make our code more strict
// instead of a string , we use ENUM because now move_around
// fn can take only 4 values as the input.
fn enums() {
    let my_direction = Direction::East;
    move_around(my_direction);

    println!("Enum with value");

    let circle = Shape::Circle(4.2);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(4.0, 5_f64);

    println!("Pattern Matching...");

    let area_c = calculate_area(circle);
    let area_s = calculate_area(square);
    let area_r = calculate_area(rectangle);

    println!("Area of cirlce {}, Area of Square {}, Area of Rectangle {}",area_c,area_s,area_r);
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(width, height) => width * height
    }
}

fn move_around(direction: Direction) {
    // implement login here
}

struct Rect {
   width: u32,
   height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
         self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn rectangle() {
    let rect = Rect {
        width: 40,
        height: 50
    };

    println!("Area of the rectangle is {}",rect.area());
    println!("Perimete of the rectangle is {}",rect.perimeter());
}

struct User {
    name: String,
    age: u32,
    active: bool,
}

fn structs() {
    let name = String::from("User 1");

    let user = User {
        name,
        age: 39,
        active: true
    };

    println!("{} is {} years old",user.name, user.age);
}

fn borrowing() {
    let str = String::from("Hello");
    
    // Borrowing only, no hanky pancky
    borrow_str(&str);

    // borrowing and making it mutable
    let mut str2 = String::from("Hello");
    borrow_mut_str(&mut str2);
    println!("Str2 is borrowed and mutate to this -> {}",str2);

    // unlimited borrowed strings but no hanky panky
    let str3 = String::from("Unlimited Borrowing");
    let b1 = &str3;
    let b2 = &str3;
    let b3 = &str3;
    let b4 = &str3;
    let b5 = &str3;

    println!("str3 -> {}, is borrowed but no hanky panky by b1 {}, b2 {}, b3 {}, b4 {}, b5 {}",str3,b1,b2,b3,b4,b5);

    // Borrowing and hanky panky with one owner and one borrower only
    let mut str4 = String::from("One Borrower & One Owner");
    {
        let mut_borrower = &mut str4; // Mutable reference is created within a block
        println!("mutBorrower -> {}", mut_borrower); // Using the mutable reference
    } // Mutable reference goes out of scope here

    // Now you can use str4 because the mutable reference no longer exists
    println!("Str 4 -> {} is no longer borrowed", str4);

    // Borrowing and hanky panky but not using it will not throw error 
    let mut str5 = String::from("One Owner , one mut borrower and one simple borrower");
    let _m_str5 = &mut str5;
    let b_str5 = &str5;

    println!("I can use bStr5 -> {}, because I have not used the mutable borrowed that is mStr5",b_str5);

}

fn borrow_str(a: &String) {
    println!("I have borrowed str, cannot mutate it but use it only, str -> {}",a);
}

fn borrow_mut_str(b: &mut String) {
    b.push_str(" World");
}


fn ownership() {
    // EXPLICITLY CHANGING OWNERS
    let a = String::from("Hello I am Kunal");
    let mut b = a;
    println!("a is not the owner now, b is {}", b); // this will give us errors if we try to use a again as ownership is changed

    // OWNER GOT CHANGED WHEN PASSED IN FUNCTION
    change_ownership(b);

    // b will remain the owner 
    // using clone() -> this will clone() the b value to pass it to c
    b = String::from("Hello");
    change_ownership(b.clone());

    // by returning the ownership by fn return
    b = return_ownership(b);
    println!("Ownership of the string comes again to b, {}",b);
}

fn change_ownership(c: String) {
    println!("Ownership of b changed to c {}",c);
}

fn return_ownership(d: String) -> String {
    println!("Ownership of b is with me, I am d, {}... passing it again to b by returning", d);
    return d;
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");

    for char in sentence.chars() {
        ans.push(char);
        if char == ' ' {
            break;
        }
    } 

    return ans;
}

fn find_even(num: i32) -> bool {
    return num % 2 == 0;
}


fn stack_fn() {
    let a = 1;
    let b = 10;
    let c = 32;
    let d = a + b + c;
    println!("Stack Function: The sume of {}, {} and {} is {}", a,b,c,d);
}

fn heap_fn() {
    let str1 = String::from("String");
    let str2 = String::from("string 2");
    let combined = format!("{} {}",str1,str2);
    println!("Heap Function: Combined String is '{}'", combined);
}

fn update_string() {
    let mut s = String::from("Intital String");
    println!("Before update: {}",s);

    for _i in 1..10 {
        s.push_str(" and some additional text");
        println!("Capacity: {}, Length: {}, Pointer: {:p}",s.capacity(),s.len(),s.as_ptr());
    }
}