use std::fs;

fn main() {
    
    // Declare a 100 x 100 grid, assuming those are the dimensions:
    let mut grid: [[u8; 100]; 100] = [[0; 100]; 100];

    let contents = fs::read_to_string("input.txt").expect("Could not read file"); // The input is a list of ints (not separated)

    let line_vec: Vec<&str> = contents.lines().collect();

    for index in 0..line_vec.len() { // Looping over the lines
        let temp_vec: Vec<&str> = line_vec[index].split("").collect();
        let mut charcount: usize = 0;

        let valid_characters = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

        for character in temp_vec { // Looping over the chars in each &str as &str
            if valid_characters.contains(&character) {
                *&mut grid[index][charcount] = character.parse().unwrap();
                // *&mut grid[index][charcount] = character.to_digit(10).unwrap(); // Cast each character to a u8 -> not a char
                // *&mut grid[index][charcount] = u8::from_str_radix(character, 10).unwrap(); // Could work
                *&mut charcount += 1;
            }
        }
    }

    // The grid is now populated
    // First star

    let mut running_count: u32 = 0;
    let mut running_risk_count: u32 = 0;

    for x in 0..100 {
        for y in 0..100 {
            *&mut running_risk_count += check_point(x, y, &grid);
            if check_point(x, y, &grid) != 0 {
                *&mut running_count += 1;
            }
        }
    }

    println!("-- ⭐ First Star ⭐ --");
    println!("Low points found: {}", running_count);
    println!("Combined risk: {}", running_risk_count);
}

fn check_point(x: usize, y: usize, grid: &[[u8; 100]; 100]) -> u32 {
    if x == 0 && y == 0 { // Corner 1
        if grid[x][y] < grid[x][y + 1] && grid[x][y] < grid[x + 1][y] {
            return grid[x][y] as u32 + 1;
        } else {
            return 0 as u32;
        }
    } else if x == 0 && y == 99 { // Corner 2
        if grid[x][y] < grid[x][y - 1] && grid[x][y] < grid[x + 1][y] {
            return grid[x][y] as u32 + 1;
        } else {
            return 0 as u32;
        }
    } else if x == 99 && y == 0 { // Corner 3
        if grid[x][y] < grid[x][y + 1] && grid[x][y] < grid[x - 1][y] {
            return grid[x][y] as u32 + 1;
        } else {
            return 0 as u32;
        }
    } else if x == 99 && y == 99 { // Corner 4
        if grid[x][y] < grid[x][y - 1] && grid[x][y] < grid[x - 1][y] {
            return grid[x][y] as u32 + 1;
        } else {
            return 0 as u32;
        }
    } else if x == 0 { // Top edge
        if grid[x][y] < grid [x][y - 1] && grid[x][y] < grid[x][y + 1] && grid [x][y] < grid[x + 1][y] {
            return grid[x][y] as u32 + 1;
        } else {
            return 0 as u32;
        }
    } else if x == 99 { // Bottom edge
        if grid[x][y] < grid [x][y - 1] && grid[x][y] < grid[x][y + 1] && grid [x][y] < grid[x - 1][y] {
            return grid[x][y] as u32 + 1;
        } else {
            return 0 as u32;
        } 
    } else if y == 0 { // Left edge
        if grid[x][y] < grid [x - 1][y] && grid[x][y] < grid[x + 1][y] && grid[x][y] < grid[x][y + 1] {
            return grid[x][y] as u32 + 1;
        } else {
            return 0 as u32;
        }
    } else if y == 99 { // Right edge
        if grid[x][y] < grid [x - 1][y] && grid[x][y] < grid[x + 1][y] && grid[x][y] < grid[x][y - 1] {
            return grid[x][y] as u32 + 1;
        } else {
            return 0 as u32;
        }
    } else { // All other cases
        if grid[x][y] < grid [x - 1][y] && grid[x][y] < grid[x + 1][y] && grid[x][y] < grid[x][y - 1] && grid[x][y] < grid[x][y + 1] {
            return grid[x][y] as u32 + 1;
        } else {
            return 0 as u32;
        }
    }
}