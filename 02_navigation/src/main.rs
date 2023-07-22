use std::fs;

// Declaring structs for coordinates and instructions

struct Coordinates {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl Coordinates {
    fn reset(&mut self) {
        self.horizontal = 0;
        self.depth = 0;
        self.aim = 0;
    }
}

struct Instruction {
    direction: String,
    distance: u32,
}

fn main() {
    // Initial position
    let mut position = Coordinates { horizontal: 0, depth: 0, aim: 0 };

    // Read file and store in a vector
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let input_vec: Vec<&str> = contents.lines().collect();

    // Loop over the vector, add an Instruction struct to a new vector for each line
    // The Vector will own the Instruction structs, scope is main()
    let mut instruction_vec: Vec<Instruction> = Vec::new();

    for line in input_vec {
        let temp_vec: Vec<&str> = line.split_whitespace().collect();
        let temp_direction = temp_vec[0].to_string();
        let temp_distance: u32 = temp_vec[1].parse().unwrap(); // Cast to u32

        let _ = &mut instruction_vec.push(build_instruction(temp_direction, temp_distance)); // Ownership passes to Vec
    }

    // ⭐ First Star ⭐

    for instr in &instruction_vec { // Borrow the vector so it is not moved
        apply_instruction(&mut position, &instr); // Never takes ownership
    }

    println!("--- ⭐ First Star ⭐ ---");
    println!("Your current position is: depth {}, horizontal {}, their product is {}\n", position.depth, position.horizontal, (position.depth * position.horizontal));

    // ⭐ Second Star ⭐

    position.reset();

    for instr in &instruction_vec {
        apply_instruction_aim(&mut position, &instr); // Never takes ownership
    }
    
    println!("--- ⭐ Second Star ⭐ ---");
    println!("Your current position is: depth {}, horizontal {}, aim {}, the product is {}\n", position.depth, position.horizontal, position.aim, (position.depth * position.horizontal));
}

// Helper functions

// Build an Instruction struct
fn build_instruction(direction: String, distance: u32) -> Instruction {
    Instruction {
        direction,
        distance,
    }
}

// Modifying a Coordinates struct (mutable borrow) by applying an Instruction struct (immutable borrow)
// ⭐ First Star ⭐
fn apply_instruction(coordinates: &mut Coordinates, instruction: &Instruction ) {
    match instruction.direction.as_str() {
        "down" => {
            coordinates.depth += instruction.distance
        },
        "forward" => {
            coordinates.horizontal += instruction.distance
        },
        "up" => {
            coordinates.depth -= instruction.distance
        },
        _ => {
            println!("Instruction contained invalid direction: {}", instruction.direction);
            panic!();
        }
    }
}

// Modifying a Coordinates struct (mutable borrow) by applying an Instruction struct (immutable borrow), includes aim
// ⭐ Second Star ⭐
fn apply_instruction_aim(coordinates: &mut Coordinates, instruction: &Instruction ) {
    match instruction.direction.as_str() {
        "down" => {
            coordinates.aim += instruction.distance
        },
        "forward" => {
            coordinates.horizontal += instruction.distance;
            coordinates.depth += instruction.distance * coordinates.aim
        },
        "up" => {
            coordinates.aim -= instruction.distance
        },
        _ => {
            println!("Instruction contained invalid direction: {}", instruction.direction);
            panic!();
        }
    }
}
