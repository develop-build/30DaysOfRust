// 1. Primitive str = Immutable fixed-length string somewhere in memory
// 2. String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
    // Type:1
    let hello = "Hello";
    // Type:2
    let mut name = String::from("Deepak ");

    // Get length
    println!("length: {}", name.len());

    // modifying string - insert char
    name.push('W'); // DeepakW

    // modifying string - insert string
    name.push_str("hire");  // Deepak Whire

    // Capacity in bytes
    println!("capacity: {}", name.capacity());  // capacity: 12

    //Check if empty
    println!("Is Empty: {}", name.is_empty());  // Is Empty: false

    // Contains substring
    println!("Contains substring: {}", name.contains("Deepak"));    // Contains substring: true

    // Replace 
    println!("Replace: {}", name.replace("Deepak", "Rust"));    // Replace: Rust Whire

    // Loop through string by whitespace
    for word in name.split_whitespace(){        // Deepak /n Whire
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing 
    assert_eq!(2, s.len()); //If true- passes, otherwise shows error!
    println!("{}", s);


    // printing strings
    println!("{}", hello); // Hello
    println!("{}", name);  // DeepakWhire
}