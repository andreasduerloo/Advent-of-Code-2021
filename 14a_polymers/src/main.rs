use std::fs;

struct Injection {
    value: char,
    index: usize,
}

impl Injection {
    fn new(value: char, index: usize) -> Injection {
        Injection {
            value,
            index,
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file"); 
    let line_vec: Vec<&str> = contents.lines().collect();

    let mut polymer: Vec<char> = Vec::new();
    let mut instructions: Vec<&str> = Vec::new();

    // Get the info out of the input file

    for index in 0..line_vec.len() {
        if index == 0 {
            *&mut polymer = line_vec[index].chars().collect();
        } else if index == 1 {
            continue;
        } else {
            let _ = &mut instructions.push(line_vec[index]);
        }
    }

    for _pass in 0..10 {
        let mut injection_vec: Vec<Injection> = Vec::new();
        for index in 1..polymer.len() {
            let _ = &mut injection_vec.push(find_injection(&polymer[index - 1], &polymer[index], &instructions, index));
        }
    
        // Insert the injections from back to front so we don't mess up the indices
        for injection in injection_vec.iter().rev() {
            let _ = &mut polymer.insert(injection.index, injection.value);
        }
    }
    
    let count: Vec<Injection> = count_values(&polymer);

    for index in 0..count.len() {
        println!("Element {} occurs {} times.", count[index].value, count[index].index);
    }

    let lowest: usize = find_lowest(&count);
    let highest: usize = find_highest(&count);

    println!("--- ⭐ First Star ⭐ ---");
    println!("Difference: {}", highest - lowest);
}

fn find_injection(left: &char, right: &char, instr_vec: &Vec<&str>, index: usize) -> Injection {
    for instruction_index in 0..instr_vec.len() {
        let temp_vec: Vec<char> = instr_vec[instruction_index].chars().collect();
        if *left == temp_vec[0] && *right == temp_vec[1] {
            let output = Injection::new(temp_vec[6], index);
            return output;
        }
    }
    panic!("Could not resolce call to find_injection for {} and {}", left, right);
}

fn count_values(input_vec: &Vec<char>) -> Vec<Injection> { // We can re-use the Injection struct to count occurences
    let mut output_vec = Vec::new();
    for item in input_vec {
        if output_vec.len() == 0 {
            let temp_inj = Injection::new(*item, 1);
            let _ = &mut output_vec.push(temp_inj);
            continue;
        } else {
            let mut elem_present: bool = false;
            for index in 0..output_vec.len() {
                if *item == output_vec[index].value {
                    *&mut output_vec[index].index += 1;
                    *&mut elem_present = true;
                    break;
                }
            }
            if !elem_present {
                let temp_inj = Injection::new(*item, 1);
                let _ = &mut output_vec.push(temp_inj);
            }
        }
    }
    return output_vec;
}

fn find_lowest(input_vec: &Vec<Injection>) -> usize {
    let mut lowest: usize = usize::MAX;
    for elem in input_vec {
        if elem.index < lowest {
            *&mut lowest = elem.index;
        }
    }
    return lowest;
}

fn find_highest(input_vec: &Vec<Injection>) -> usize {
    let mut highest: usize = 0;
    for elem in input_vec {
        if elem.index > highest {
            *&mut highest = elem.index;
        }
    }
    return highest;
}