use std::fs;

fn main() {

    // Read file and store it into a vector (could theoretically use an array)
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let input_vec: Vec<&str> = contents.lines().collect();

    // Mutable global variables
    let mut first_star: u32 = 0;
    let mut second_star: u32 = 0;

    for index in 0..(input_vec.len() - 1) {
        let first_number: u32 = input_vec[index].parse().unwrap(); // Casting str as u32
        let second_number: u32 = input_vec[(index + 1)].parse().unwrap();

        if first_number < second_number {
            first_star += 1;
        }
    }

    for index in 0..(input_vec.len() - 3) {
        let first_number: u32 = input_vec[index].parse().unwrap();
        let second_number: u32 = input_vec[(index + 3)].parse().unwrap();

        if first_number < second_number {
            second_star += 1;
        }
    }

    println!("--- ⭐ First Star ⭐ ---");
    println!("Drops: {}", first_star);
    println!("--- ⭐ Second Star ⭐ ---");
    println!("Drops: {}", second_star);
}
