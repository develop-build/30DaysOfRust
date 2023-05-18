// Arrays - Fixed list where elements are the same data types

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}", numbers);

    // Re-assign Val
    numbers[2] = 50;

    // Add on to vector
    numbers.push(6);
    numbers.push(7);

    // Pop off the last value
    numbers.pop();

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice 
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}",slice);

    // Loop through vectors
    for x in numbers.iter() {
        println!("Number: {:?}", x);
    }

    // Loop and mutate value
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);
}