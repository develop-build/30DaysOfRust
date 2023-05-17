pub fn run(){
    let name = "Deepak";
    let mut age = 25;
    println!("My name is {} and my age is {}", name, age);

    age = 38;
    println!("My name is {} and my age is {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let ( my_name, my_age) = ("Deepak", 18);
    println!("My name is {} and my age is {}", my_name, my_age);
}