fn main() {
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

    // Borrowing and hanky panky but not using it will throw error if you borrow again
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
    println!("a is not the owner now, b is {}", b); // this will give us errors

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