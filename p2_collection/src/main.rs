fn main() {
    // Array
    let lucky_numbers: [i32; 3] = [5, 8, 12];

    if lucky_numbers.is_empty() {
        println!("Array empty")
    } else {
        println!("Array not emepty")
    }
    println!("Array have {} items", lucky_numbers.len());

    let my_number = 121;
    let has_my = lucky_numbers.contains(&my_number);
    println!("Array has my number: {}", has_my);

    let my_index = lucky_numbers.iter().position(|&v| v == my_number);
    println!("Index of my number: {}", my_index.unwrap_or(0));

    let sum: i32 = lucky_numbers.iter().sum();
    println!("Array sum: {}", sum);

    let slice = &lucky_numbers[0..2];
    println!("Slice: {:?}", slice);
}
