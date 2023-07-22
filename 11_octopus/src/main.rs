use std::fs;

#[derive(Copy, Clone)] // Can also write these yourself: impl Copy for Octopus {}, impl Clone for Octopus {...}
struct Octopus {
    energy: u32,
    flashed: bool,
}

impl Octopus {
    fn new(energy: u32) -> Octopus {
        Octopus {
            energy,
            flashed: false,
        }
    }
}

impl Octopus {
    fn reset(&mut self) {
        if self.flashed == true {
            self.energy = 0;
            self.flashed = false;
        }
    }
}

fn main() {
    
    let contents = fs::read_to_string("input.txt").expect("Could not read file"); 
    let line_vec: Vec<&str> = contents.lines().collect();

    // Declare a 12 x 12 grid uf u32's. I want a column/row of margin so I don't have to bother with corner/edge logic

    let mut input_grid: [[u32; 12]; 12] = [[0; 12]; 12];

    // Now populate the 'inner' grid with the input data

    for index in 0..line_vec.len() { // Looping over the lines
        let temp_vec: Vec<&str> = line_vec[index].split("").collect();
        let mut charcount: usize = 0;

        let valid_characters = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]; // Trim the newlines

        for character in temp_vec { // Looping over the chars in each &str as &str
            if valid_characters.contains(&character) {
                *&mut input_grid[index + 1][charcount + 1] = character.parse().unwrap();
                *&mut charcount += 1;
            }
        }
    }

    // Debug - looks good.
    for row in 0..12 {
        println!("{} {} {} {} {} {} {} {} {} {} {} {}", input_grid[row][0], input_grid[row][1], input_grid[row][2], input_grid[row][3], input_grid[row][4], input_grid[row][5], input_grid[row][6], input_grid[row][7], input_grid[row][8], input_grid[row][9], input_grid[row][10], input_grid[row][11]);
    }

    // Provide a grid for Octopus structs

    let mut grid: [[Octopus; 12]; 12] = [[Octopus::new(0); 12]; 12];

    for row in 0..12 {
        for column in 0..12 {
            let temp_oct: Octopus = Octopus::new(input_grid[row][column]);
            *&mut grid[row][column].energy = temp_oct.energy;
        }
    }

    // Debug - still looks good.
    println!("---");
    for row in 0..12 {
        println!("{} {} {} {} {} {} {} {} {} {} {} {}", grid[row][0].energy, grid[row][1].energy, grid[row][2].energy, grid[row][3].energy, grid[row][4].energy, grid[row][5].energy, grid[row][6].energy, grid[row][7].energy, grid[row][8].energy, grid[row][9].energy, grid[row][10].energy, grid[row][11].energy);
    }

    let mut flashes: u32 = 0;

    let mut sync: bool = false;
    let mut steps: u32 = 0;

    while !sync {
        *&mut sync = step(&mut grid, &mut flashes);
        *&mut steps += 1;
        if steps == 100 {
            println!("--- ⭐ First Star ⭐ ---");
            println!("There were {} flashes 🐙", flashes);
        }
        if sync == true {
            println!("--- ⭐ Second Star ⭐ ---");
            println!("The octopi were synced after step {} 🐙", steps);
            break;
        }
    }
}

fn step(grid: &mut [[Octopus; 12]; 12], flashcount: &mut u32) -> bool {
    // Energy goes up with one
    for row in 1..11 {
        for column in 1..11 {
            grid[row][column].energy += 1;
        }
    }

    // Check how many are over 9 - loop until all flashes are done
    let mut to_flash: u32 = count_to_flash(&grid);

    while to_flash != 0 {
        for row in 1..11 {
            for column in 1..11 {
                if grid[row][column].energy > 9 && grid[row][column].flashed == false {
                    flash(row, column, grid);
                    *flashcount += 1;
                }
            }
        }
        *&mut to_flash = count_to_flash(&grid);        
    }

    // Check if they all flashed at the same time

    let mut running_count: u32 = 0;
    for row in 1..11 {
        for column in 1..11 {
            if grid[row][column].flashed == true {
                *&mut running_count += 1;
            }
        }
    }

    if running_count == 100 {
        return true;
    }

    // Set the Octopi that flashed to 0 and set their flags back to false

    for row in 1..11 {
        for column in 1..11 {
            grid[row][column].reset();
        }
    }

    false
}

fn count_to_flash(grid: &[[Octopus; 12]; 12]) -> u32 {
    let mut output: u32 = 0;
    for row in 1..11 {
        for column in 1..11 {
            if grid[row][column].energy > 9 && grid[row][column].flashed == false {
                *&mut output += 1;
            }
        }
    }
    return output;
}

fn flash(x: usize, y: usize, grid: &mut [[Octopus; 12]; 12]) {
    // Increment the energy of all surrounding Octopi (?)
    grid[x - 1][y - 1].energy += 1;
    grid[x - 1][y].energy += 1;
    grid[x - 1][y + 1].energy += 1;
    grid[x][y - 1].energy += 1;
    grid[x][y].flashed = true;
    grid[x][y + 1].energy += 1;
    grid[x + 1][y - 1].energy += 1;
    grid[x + 1][y].energy += 1;
    grid[x + 1][y + 1].energy += 1;
}