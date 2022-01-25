use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");

    // First star

    let line_vec: Vec<&str> = contents.lines().collect();

    let simple_digits: [u32; 4] = [2, 4, 3, 7];
    let mut simple_count: u32 = 0;

    for line in &line_vec {
        let temp_vec: Vec<&str> = line.split(" | ").collect();
        let output: Vec<&str> = temp_vec[1].split_whitespace().collect();
        for elem in output {
            let elem_len: u32 = elem.len().try_into().unwrap();
            if simple_digits.contains(&elem_len) {
                *&mut simple_count += 1;
            }
        }
    }

    println!("-- ⭐ First Star ⭐ --");
    println!("Simple digits found: {}", simple_count);

    // Second star

    let mut running_count: u32 = 0;

    for line in &line_vec {
        let temp_vec: Vec<&str> = line.split(" | ").collect();
        let input: &str =  temp_vec[0];
        let output: Vec<&str> = temp_vec[1].split_whitespace().collect();

        let mapping: [&str; 10] = deduce_numbers(&input);

        for position in 0..4 { // Output always consists of four numbers
            let to_add: u32 = translate(output[position], &mapping, position.try_into().unwrap());
            *&mut running_count += to_add;
        }
    }

    println!("-- ⭐ Second Star ⭐ --");
    println!("Total output: {}", running_count);
}

fn deduce_numbers(input: &str) -> [&str; 10] {
    let temp_vec: Vec<&str> = input.split_whitespace().collect();
    let mut output_array: [&str; 10] = ["a"; 10];
    
    for elem in &temp_vec { // Find the one
        if elem.len() == 2 as usize {
            let number_one: &str = elem;
            *&mut output_array[1] = number_one;
        }
    }

    for elem in &temp_vec { // Find the seven
        if elem.len() == 3 as usize {
            let number_seven: &str = elem;
            *&mut output_array[7] = number_seven;
        }
    }

    for elem in &temp_vec { // Find the four
        if elem.len() == 4 as usize {
            let number_four: &str = elem;
            *&mut output_array[4] = number_four;
        }
    }

    for elem in &temp_vec { // Find the eight
        if elem.len() == 7 as usize {
            let number_eight: &str = elem;
            *&mut output_array[8] = number_eight;
        }
    }

    for elem in &temp_vec { // Find the zero: length is six, one entirely present, 3/4 four present
        if elem.len() == 6 as usize && segments_present(output_array[1], elem) && segments_common_with_ref(elem, output_array[4]) == 3 {
            let number_zero: &str = elem;
            *&mut output_array[0] = number_zero;
        }
    }

    for elem in &temp_vec { // Find the nine: length is six and both parts of one are present, 4/4 four present
        if elem.len() == 6 as usize && segments_present(output_array[1], elem) && segments_common_with_ref(elem, output_array[4]) == 4 {
            let number_nine: &str = elem;
            *&mut output_array[9] = number_nine;
        }
    }

    for elem in &temp_vec { // Find the six: length is six and only one of one's parts is present
        if elem.len() == 6 as usize && ! segments_present(output_array[1], elem) {
            let number_six: &str = elem;
            *&mut output_array[6] = number_six;
        }
    }

    for elem in &temp_vec { // Find the three: length is five and both parts of one are present
        if elem.len() == 5 as usize && segments_present(output_array[1], elem) {
            let number_three: &str = elem;
            *&mut output_array[3] = number_three;
        }
    }

    for elem in &temp_vec { // Find the five: length is five and there are five segments in common with nine, one not entirely present
        if elem.len() == 5 as usize && segments_common_with_ref(elem, &output_array[9]) == 5 && ! segments_present(output_array[1], elem) {
            let number_five: &str = elem;
            *&mut output_array[5] = number_five;
        }
    }

    for elem in &temp_vec { // Find the two: length is five, four elements in common with nine, one not entirely present (would be a three)
        if elem.len() == 5 as usize && segments_common_with_ref(elem, &output_array[9]) == 4 && ! segments_present(output_array[0], elem) {
            let number_two: &str = elem;
            *&mut output_array[2] = number_two;
        }
    }

    return output_array;
}

fn segments_present(small_number: &str, large_number: &str) -> bool {
    for segment in small_number.chars() { // That's a &str, so we should be looping over chars
        if large_number.contains(segment) {
            continue;
        } else {
            return false;
        }
    }
    return true;
}

fn segments_common_with_ref(to_check: &str, number_reference: &str) -> u32 {
    let mut counter: u32 = 0;

    for elem in number_reference.chars() {
        if to_check.contains(elem) {
            *&mut counter += 1
        }
    }

    return counter;
}

fn translate(digit: &str, mapping: &[&str; 10], position: u32) -> u32 {
    let length: usize = mapping.len().try_into().unwrap(); // 10
    for index in 0..length {
        if digit.len() == mapping[index].len() { // They're the same length, good start
            let mut common_chars: u32 = 0;
            for character in digit.chars() {
                if mapping[index].contains(character) {
                    *&mut common_chars += 1;
                }
            }
            if common_chars == mapping[index].len().try_into().unwrap() {
                return ( index as u32 ) * 10_u32.pow(3 - position);
            }
        }
    }
    println!("Couldnt translate digit {}", digit);

    for index in 0..10 {
        println!("Position {}: {}", index, mapping[index]);
    }

    return 0; // There's been a problem
}
