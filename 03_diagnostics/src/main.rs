use std::fs;

enum Rule {
    Oxygen,
    Co2,
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let input_vec: Vec<&str> = contents.lines().collect();

    let mut int_vec: Vec<u16> = Vec::new();

    for line in input_vec { // Interpreting the binary as unsigned integers (input vec is moved).
        let temp_int: u16 = u16::from_str_radix(&line, 2).unwrap();
        let _ = &mut int_vec.push(temp_int);
    }

    // First star

    let mut gamma: u16 = 0;

    for position in 0..12 { // There are 12 positions to check, all the input consists of 12 bits. We're going left to right.
        let mut zeroes: u16 = 0;
        let mut ones: u16 = 0;

        let bitmask: u16 = 2_u16.pow(11 - &position);

        for number in &int_vec {
            if number & bitmask > 0 { // Bitwise AND
                ones += 1;
            } else {
                zeroes += 1;
            }
        }

        if &ones > &zeroes {
            *&mut gamma += 2_u16.pow(11 - &position);
        }
    }

    let epsilon: u16 = ( ! gamma ) & 0x0FFF; // Inverse of 16-bit integer, remove leading ones with bitwise AND.

    let gamma: u32 = gamma.try_into().unwrap(); // Cast to u32's to prevent overflow on multiplication.
    let epsilon: u32 = epsilon.try_into().unwrap();

    println!("--- ⭐ First star ⭐ ---");
    println!("Gamma rate: {}, Epsilon rate: {}", gamma, epsilon);
    println!("Product: {}", gamma * epsilon);

    // Second star

    // Declare an array to contain responses and ints to contain the final answers
    let mut answer_vec: Vec<u16> = Vec::new();
    let mut oxygen: u16 = 0;
    let mut co2: u16 = 0;

    // Oxygen
    for pass in 0..12 {
        if pass == 0 {
            answer_vec = vector_sieve(&int_vec, pass, Rule::Oxygen);
        } else {
            answer_vec = vector_sieve(&answer_vec, pass, Rule::Oxygen);
        }

        if answer_vec.len() == 1 { // We found the only number
            oxygen = answer_vec[0];
            break;
        } else { // We didn't, look at the next bit
            continue;
        }  
    }

    // Co2
    for pass in 0..12 {
        if pass == 0 {
            answer_vec = vector_sieve(&int_vec, pass, Rule::Co2);
        } else {
            answer_vec = vector_sieve(&answer_vec, pass, Rule::Co2);
        }

        if answer_vec.len() == 1 { // We found the only number
            co2 = answer_vec[0];
            break;
        } else { // We didn't, look at the next bit
            continue;
        }  
    }

    let oxygen: u32 = oxygen.try_into().unwrap(); // Cast to u32's to prevent overflow on multiplication.
    let co2: u32 = co2.try_into().unwrap();

    println!("--- ⭐ Second star ⭐ ---");
    println!("Oxygen: {}, Co2: {}", oxygen, co2);
    println!("Product: {}", oxygen * co2);

}

// This function takes a vector of ints and a position, 
// figures out what the most/least common bit is at that position
// and returns a vector of ints where the bit at that position has that value.
fn vector_sieve(value_vector: &Vec<u16>, position: u8, rule: Rule) -> Vec<u16> {
    let mut output_vector: Vec<u16> = Vec::new();
    let mut ones: u16 = 0;
    let mut zeroes: u16 = 0;

    let bitmask: u16 = 1 << (11 - position); // Make the bitmask for that position

    // Look at the individual bits and count ones and zeroes
    for entry in value_vector {
        if entry & bitmask == bitmask { // Bitwise AND: is there a 1 at this position?
            ones += 1; // There is
        } else {
            zeroes += 1; // There isn't
        }
    }

    let wanted_value: u16 = match rule { // Determine what value we want the bit to be
        Rule::Oxygen => {
            if &ones >= &zeroes { // More ones or equal -> take 1
                1
            } else {
                0
            }
        },
        Rule::Co2 => {
            if &ones >= &zeroes { // More ones or equal -> take 0
                0
            } else {
                1
            }
        }
    };

    // Collect entries where the bit matches the wanted value

    for entry in value_vector {
        if entry & bitmask == wanted_value << (11 - position) { // Compare the bit with the wanted value
            output_vector.push(*entry); // Looks good, add it to the output vector.
        }
    }
    return output_vector;
}
