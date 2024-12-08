fn main() {
    let name = "world"; // name is a string literal
    println!("Hello, {}!", name);

    let mut count = 0; // count is a mutable integer
    count += 1;
    println!("Count: {}", count);

    let is_rust_cool = true; // is_rust_cool is a boolean
    if is_rust_cool {
        println!("Rust is cool!");
    } else {
        println!("Rust is not cool!");
    }

    let mut numbers = vec![1, 2, 3, 4, 5]; // numbers is a vector
    numbers.push(6);
    for number in numbers.iter() {
        println!("Number: {}", number);
    }

    let doubled = numbers.iter().map(|x| x * 2).collect::<Vec<i32>>(); // doubled is a vector
    for number in doubled.iter() {
        println!("Doubled: {}", number);
    }
}
