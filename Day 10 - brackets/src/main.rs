use std::fs;

// First star
// Plan: loop through the line and keep track of the last 'opened' bracket, the next closing bracket must match this one.
// If a closing bracket matches the last opened bracket, we go one back in the 'history' of opened brackets
// That bracket becomes the new 'last opened bracket'
// Like a pop/push mechanism -> use a vector

// Second star
// Adapt the code for the first star so that it dumps incomplete lines in a new vector
// Loop through those lines, and at the end start unwinding the stack, storing the needed element in an output vector
// Calculate the value of the output vector, store that value in another new vector
// Find the middle value of the last vector

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file"); 
    let line_vec: Vec<&str> = contents.lines().collect();

    let left_brackets: [&str; 4] = ["(", "[", "{", "<"];
    let right_brackets: [&str; 4] = [")", "]", "}", ">"];

    let mut running_count: u32 = 0;

    let mut second_star_vec: Vec<&str> = Vec::new();

    for index in 0..line_vec.len() {
        let mut error: bool = false;

        let temp_vec: Vec<&str> = line_vec[index].split("").collect(); // Or can we directly loop over characters?

        let mut stack: Vec<&str> = Vec::new();

        for character in &temp_vec {
            if left_brackets.contains(character) {
                let _ = &mut stack.push(character);
            } else if right_brackets.contains(character) { // It's a closing bracket - compare it with the top of the stack
                let to_check: &str = &stack[stack.len() - 1];
                if match_brackets(&to_check, character) {
                    let _ = &mut stack.pop();
                    continue;
                } else {
                    // println!("Line {}: {} and {} don't match", index, to_check, character);
                    // println!("Adding {}", value(character));
                    *&mut running_count += value(character);
                    *&mut error = true;
                    break;
                }
            } else { // We've reached the newline character
                continue;
            }
        }
        if error == false {
            // println!("Keeping line {} for the second star.", index);
            let _ = &mut second_star_vec.push(line_vec[index]);
        }
    }

    println!("--- ⭐ First Star ⭐ ---");
    println!("The current score is: {}", running_count);

    // println!("There are {} elements in the second star vector.", second_star_vec.len());

    // Second star

    let mut score_vec: Vec<u64> = Vec::new();

    for index in 0..second_star_vec.len() {
        let temp_vec: Vec<&str> = second_star_vec[index].split("").collect();

        let mut stack: Vec<&str> = Vec::new();

        for character in &temp_vec {
            if left_brackets.contains(character) {
                let _ = &mut stack.push(character);
            } else if right_brackets.contains(character) { // It's a closing bracket - compare it with the top of the stack
                let to_check: &str = &stack[stack.len() - 1];
                if match_brackets(&to_check, character) {
                    let _ = &mut stack.pop();
                    continue;
                } else {
                    println!("There's been an issue: there's an error in the second star vector.")
                }
            } else { // We've reached the newline character, unwind the stack
                let mut temp_score: u64 = 0;
                for pos in 0..stack.len() {
                    *&mut temp_score = (temp_score * 5) + close_bracket(stack[stack.len() - (pos + 1)]);
                    // println!("Debug, temp score is now: {}", temp_score);
                }
                if temp_score != 0 {
                    let _ = &mut score_vec.push(temp_score);
                }
            }
        }
    }

    // println!("There are {} elements in the score vector", score_vec.len());

    let middle_score: u64 = find_middle(&mut score_vec);

    println!("--- ⭐ Second Star ⭐ ---");
    println!("The middle score is: {}", middle_score);

}

fn match_brackets(left: &str, right: &str) -> bool {
    if left == "(" && right == ")" || left == "[" && right == "]" || left == "{" && right == "}" || left == "<" && right == ">" {
        return true;
    } else {
        return false;
    }
}

fn value(input: &str) -> u32 {
    match input {
        ")" => {
            return 3;
        },
        "]" => {
            return 57;
        },
        "}" => {
            return 1197;
        },
        ">" => {
            return 25137;
        },
        _ => {
            println!("Invalid input: {}", input);
            return 0;
        }
    }
}

fn close_bracket(input: &str) -> u64 {
    match input {
        "(" => {
            return 1;
        },
        "[" => {
            return 2;
        },
        "{" => {
            return 3;
        },
        "<" => {
            return 4;
        },
        _ => {
            println!("Invalid input: {}", input);
            return 0;
        }
    } 
}

fn find_middle(input_vec: &mut Vec<u64>) -> u64 {
    while input_vec.len() != 1 {
        let mut smallest: u64 = u64::MAX;
        let mut smallest_index: usize = 0;
        let mut largest: u64 = 0;
        let mut largest_index: usize = 0;

        for index in 0..input_vec.len() {
            if input_vec[index] > largest {
                *&mut largest = input_vec[index];
                *&mut largest_index = index;
            }
            if input_vec[index] < smallest {
                *&mut smallest = input_vec[index];
                *&mut smallest_index = index;
            }
        }

        if largest_index > smallest_index { // Removing an element to the left of the other first messes with the indices
            input_vec.remove(largest_index);
            input_vec.remove(smallest_index);
        } else {
            input_vec.remove(smallest_index);
            input_vec.remove(largest_index);
        }
    }
    return input_vec[0];
}