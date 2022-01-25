use std::fs;

// Second star
// Every low point has exactly one basin. Basins are bordered by nines.
// Plan:
// 1_ Find all low points and flag give them a
// 2_ Loop over all points, if they're a ridge, move on, if they aren't, check if a neighbor has a number
// If the neighbor has a number, copy it into that point
// Repeat until no squares are left without a number
// Count occurences of numbers

fn main() {
    // Populate the base grid:
    let mut grid: [[u8; 100]; 100] = [[0; 100]; 100];

    let contents = fs::read_to_string("input.txt").expect("Could not read file"); 
    let line_vec: Vec<&str> = contents.lines().collect();

    for index in 0..line_vec.len() { // Looping over the lines
        let temp_vec: Vec<&str> = line_vec[index].split("").collect();
        let mut charcount: usize = 0;

        let valid_characters = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]; // Trim the newlines

        for character in temp_vec { // Looping over the chars in each &str as &str
            if valid_characters.contains(&character) {
                *&mut grid[index][charcount] = character.parse().unwrap();
                *&mut charcount += 1;
            }
        }
    }

    // Declare a grid of u16's.
    // Ridge = 0xFFFF
    // Undiscovered = 0x0FFF
    // Low points = 0x0000 -> 0x0FFE

    let mut basin_grid: [[u16; 100]; 100] = [[0x0FFF; 100]; 100];

    // Find the low points and ridges and copy them to the u16 grid

    let mut basin_id: u16 = 0x0000;

    for x in 0..100 {
        for y in 0..100 {
            draw_grid(x, y, &grid, &mut basin_grid, &mut basin_id);
        }
    }

    // Grid is drawn, time let the magic work

    println!("The grid has been drawn - there were {} lowest points.", basin_id);

    let mut unknowns: u32 = count_unknowns(&basin_grid);

    while unknowns != 0 {
        for x in 0..100 {
            for y in 0..100 {
                change_neighbors(x, y, &mut basin_grid)
            }
        }
        *&mut unknowns = count_unknowns(&basin_grid);
        println!("Last pass there were {} unknowns left.", &unknowns);
    }
    println!("All done!");

    // Now we have to count how often each of the basins appears

    let mut basin_arr: [u32; 226] = [0; 226];

    for index in 0..226 {
        for x in 0..100 {
            for y in 0..100 {
                if *&basin_grid[x][y] == index {
                    *&mut basin_arr[index as usize] += 1;
                }
            }
        }
    }

    let answer: u32 = product_of_three_biggest(&basin_arr);

    println!("-- ⭐ Second Star ⭐ --");
    println!("The product is: {}", answer);

    three_biggest(&basin_arr);

}

fn draw_grid(x: usize, y: usize, source_grid: &[[u8; 100]; 100], destination_grid: &mut [[u16; 100]; 100], index: &mut u16) {
    if source_grid[x][y] == 9 { // This place is a ridge, draw it and exit
        destination_grid[x][y] = 0xFFFF;
        return;
    }
    if x == 0 && y == 0 { // Corner 1
        if source_grid[x][y] < source_grid[x][y + 1] && source_grid[x][y] < source_grid[x + 1][y] {
            destination_grid[x][y] = *index;
            *index += 1;
            return;
        } else {
            destination_grid[x][y] = 0x0FFF;
            return;
        }
    } else if x == 0 && y == 99 { // Corner 2
        if source_grid[x][y] < source_grid[x][y - 1] && source_grid[x][y] < source_grid[x + 1][y] {
            destination_grid[x][y] = *index;
            *index += 1;
            return;
        } else {
            destination_grid[x][y] = 0x0FFF;
            return;
        }
    } else if x == 99 && y == 0 { // Corner 3
        if source_grid[x][y] < source_grid[x][y + 1] && source_grid[x][y] < source_grid[x - 1][y] {
            destination_grid[x][y] = *index;
            *index += 1;
            return;
        } else {
            destination_grid[x][y] = 0x0FFF;
            return;
        }
    } else if x == 99 && y == 99 { // Corner 4
        if source_grid[x][y] < source_grid[x][y - 1] && source_grid[x][y] < source_grid[x - 1][y] {
            destination_grid[x][y] = *index;
            *index += 1;
            return;
        } else {
            destination_grid[x][y] = 0x0FFF;
            return;
        }
    } else if x == 0 { // Top edge
        if source_grid[x][y] < source_grid[x][y - 1] && source_grid[x][y] < source_grid[x][y + 1] && source_grid[x][y] < source_grid[x + 1][y] {
            destination_grid[x][y] = *index;
            *index += 1;
            return;
        } else {
            destination_grid[x][y] = 0x0FFF;
            return;
        }
    } else if x == 99 { // Bottom edge
        if source_grid[x][y] < source_grid [x][y - 1] && source_grid[x][y] < source_grid[x][y + 1] && source_grid[x][y] < source_grid[x - 1][y] {
            destination_grid[x][y] = *index;
            *index += 1;
            return;
        } else {
            destination_grid[x][y] = 0x0FFF;
            return;
        } 
    } else if y == 0 { // Left edge
        if source_grid[x][y] < source_grid [x - 1][y] && source_grid[x][y] < source_grid[x + 1][y] && source_grid[x][y] < source_grid[x][y + 1] {
            destination_grid[x][y] = *index;
            *index += 1;
            return;
        } else {
            destination_grid[x][y] = 0x0FFF;
            return;
        }
    } else if y == 99 { // Right edge
        if source_grid[x][y] < source_grid [x - 1][y] && source_grid[x][y] < source_grid[x + 1][y] && source_grid[x][y] < source_grid[x][y - 1] {
            destination_grid[x][y] = *index;
            *index += 1;
            return;
        } else {
            destination_grid[x][y] = 0x0FFF;
            return;
        }
    } else { // All other cases
        if source_grid[x][y] < source_grid [x - 1][y] && source_grid[x][y] < source_grid[x + 1][y] && source_grid[x][y] < source_grid[x][y - 1] && source_grid[x][y] < source_grid[x][y + 1] {
            destination_grid[x][y] = *index;
            *index += 1;
            return;
        } else {
            destination_grid[x][y] = 0x0FFF;
            return;
        }
    }
}

fn change_neighbors(x: usize, y: usize, grid: &mut[[u16; 100]; 100]) {
    if grid[x][y] == 0xFFFF { // This is a ridge, bye
        return;
    }
    if grid[x][y] == 0x0FFF { // This is an unknown, bye
        return;
    }
    if x == 0 && y == 0 {
        if grid[x][y + 1] == 0x0FFF { // It's unknown, set it to own value
            grid[x][y + 1] = grid[x][y];
        }
        if grid[x + 1][y] == 0x0FFF {
            grid[x + 1][y] = grid[x][y];
        }
    } else if x == 0 && y == 99 { // Corner 2
        if grid[x][y - 1] == 0x0FFF {
            grid[x][y - 1] = grid[x][y];
        }
        if grid[x + 1][y] == 0x0FFF {
            grid[x + 1][y] = grid[x][y];
        }
    } else if x == 99 && y == 0 { // Corner 3
        if grid[x][y + 1] == 0x0FFF {
            grid[x][y + 1] = grid[x][y];
        }
        if grid[x - 1][y] == 0x0FFF {
            grid[x - 1][y] = grid[x][y];
        }
    } else if x == 99 && y == 99 { // Corner 4
        if grid[x][y - 1] == 0x0FFF {
            grid[x][y - 1] = grid[x][y];
        }
        if grid[x - 1][y] == 0x0FFF {
            grid[x - 1][y] = grid[x][y];
        }
    } else if x == 0 { // Top edge
        if grid[x][y - 1] == 0x0FFF {
            grid[x][y - 1] = grid[x][y];
        }
        if grid[x][y + 1] == 0x0FFF {
            grid[x][y + 1] = grid[x][y];
        }
        if grid[x + 1][y] == 0x0FFF {
            grid[x + 1][y] = grid[x][y];
        }
    } else if x == 99 { // Bottom edge
        if grid[x][y - 1] == 0x0FFF {
            grid[x][y - 1] = grid[x][y];
        }
        if grid[x][y + 1] == 0x0FFF {
            grid[x][y + 1] = grid[x][y];
        }
        if grid[x - 1][y] == 0x0FFF {
            grid[x - 1][y] = grid[x][y]; 
        }
    } else if y == 0 { // Left edge
        if grid[x + 1][y] == 0x0FFF {
            grid[x + 1][y] = grid[x][y];
        }
        if grid[x - 1][y] == 0x0FFF {
            grid[x - 1][y] = grid[x][y];
        }
        if grid[x][y + 1] == 0x0FFF {
            grid[x][y + 1] = grid[x][y]; 
        } 
    } else if y == 99 { // Right edge
        if grid[x + 1][y] == 0x0FFF {
            grid[x + 1][y] = grid[x][y];
        }
        if grid[x - 1][y] == 0x0FFF {
            grid[x - 1][y] = grid[x][y];
        }
        if grid[x][y - 1] == 0x0FFF {
            grid[x][y - 1] = grid[x][y]; 
        }
    } else { // All other cases
        if grid[x + 1][y] == 0x0FFF {
            grid[x + 1][y] = grid[x][y];
        }
        if grid[x - 1][y] == 0x0FFF {
            grid[x - 1][y] = grid[x][y];
        }
        if grid[x][y - 1] == 0x0FFF {
            grid[x][y - 1] = grid[x][y]; 
        }
        if grid[x][y + 1] == 0x0FFF {
            grid[x][y + 1] = grid[x][y];
        }
    }
}

fn count_unknowns(grid: &[[u16; 100]; 100]) -> u32 {
    let mut output: u32 = 0;
    for x in 0..100 {
        for y in 0..100 {
            if grid[x][y] == 0x0FFF {
                *&mut output += 1;
            }
        }
    }
    return output;
}

fn product_of_three_biggest(input: &[u32; 226]) -> u32 {
    let mut third: u32 = 0;
    let mut second: u32 = 0;
    let mut first: u32 = 0;

    for number in input { // Find the biggest
        if *number > first {
            *&mut first = *number;
        }
    }

    for number in input { // Find the second
        if *number == first {
            continue;
        } else if *number > second {
            *&mut second = *number;
        }
    }

    for number in input { // Find the third
        if *number == first || *number == second {
            continue;
        } else if *number > third {
            *&mut third = *number;
        }
    }

    return first * second * third;
}

fn three_biggest(input: &[u32; 226]) {
    let mut third: u32 = 0;
    let mut second: u32 = 0;
    let mut first: u32 = 0;

    for number in input { // Find the biggest
        if *number > first {
            *&mut first = *number;
        }
    }

    for number in input { // Find the second
        if *number == first {
            continue;
        } else if *number > second {
            *&mut second = *number;
        }
    }

    for number in input { // Find the third
        if *number == first || *number == second {
            continue;
        } else if *number > third {
            *&mut third = *number;
        }
    }

    println!("{} {} {}", first, second, third);
}