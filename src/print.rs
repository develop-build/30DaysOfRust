pub fn run(){
    // Print to console
    println!("Hello from the print.rs file");

    //Basic Formatting
    println!("{} is from {}", "Deepak", "Mass");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Deepak", "India", "Code");

    //Named  Arguments
    println!("{name} likes to play {activity}", name="Deepak", activity="football");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    //Baisc Math
    println!("10 + 10 = {}", 10 + 10);
}