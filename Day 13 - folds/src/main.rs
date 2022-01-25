use std::fs;

enum Axis {
    X,
    Y,
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file"); 
    let line_vec: Vec<&str> = contents.lines().collect();

    // Figuring out the starting dimensions of the grid
    let mut rows: u32 = 0;
    let mut columns: u32 = 0;

    for line in &line_vec {
        if *line == "" {
            break;
        }

        let mut coord_vec: Vec<u32> = Vec::new();

        let temp_vec: Vec<&str> = line.split(",").collect();
        for coord in temp_vec {
            let coord_int: u32 = coord.parse().unwrap();
            let _ = &mut coord_vec.push(coord_int);
        }

        if coord_vec[0] > columns {
            *&mut columns = coord_vec[0];
        }
        if coord_vec[1] > rows {
            *&mut rows = coord_vec[1];
        }
    }

    // println!("The dimensions are {} columns (X-axis) by {} rows (Y-axis).", columns + 1, rows + 1);
    // Turns out the dimensions are 1310 x 894, but zero is also an index
    // Declare a vector of vectors to contain this grid

    let mut starting_grid: Vec<Vec<char>> = vec![vec![' '; columns as usize + 1]; rows as usize + 1];

    // Populate the grid

    for line in &line_vec {
        if *line == "" {
            break;
        }

        let mut coord_vec: Vec<usize> = Vec::new();

        let temp_vec: Vec<&str> = line.split(",").collect();
        for coord in temp_vec {
            let coord_int: usize = coord.parse().unwrap();
            let _ = &mut coord_vec.push(coord_int);
        }

        *&mut starting_grid[coord_vec[1]][coord_vec[0]] = '#';
    }

    // First star: fold along x=655

    starting_grid = fold(&starting_grid, Axis::Y, 655);

    // Count the dots

    let mut count: u32 = 0;

    for row in 0..starting_grid.len() {
        for column in 0..starting_grid[0].len() {
            if *&starting_grid[row][column] == '#' {
                *&mut count += 1;
            }
        }
    }

    println!("--- ⭐ First Star ⭐ ---");
    println!("Visible points: {}", count);

    // Do the other folds:
    // fold along y=447
    starting_grid = fold(&starting_grid, Axis::X, 447);

    // fold along x=327
    starting_grid = fold(&starting_grid, Axis::Y, 327);

    // fold along y=223
    starting_grid = fold(&starting_grid, Axis::X, 223);

    // fold along x=163
    starting_grid = fold(&starting_grid, Axis::Y, 163);

    // fold along y=111
    starting_grid = fold(&starting_grid, Axis::X, 111);

    // fold along x=81
    starting_grid = fold(&starting_grid, Axis::Y, 81);

    // fold along y=55
    starting_grid = fold(&starting_grid, Axis::X, 55);

    // fold along x=40
    starting_grid = fold(&starting_grid, Axis::Y, 40);

    // fold along y=27
    starting_grid = fold(&starting_grid, Axis::X, 27);

    // fold along y=13
    starting_grid = fold(&starting_grid, Axis::X, 13);

    //fold along y=6
    starting_grid = fold(&starting_grid, Axis::X, 6);

    println!("--- ⭐ Second Star ⭐ ---");
    for row in 0..starting_grid.len() {
        for column in 0..starting_grid[0].len() {
            print!("{} ", starting_grid[row][column]);
        }
        println!();
    }
}

// Write a function that folds the grid, 'add' the chars at the same spot, copy into a new vector with new dimensions
fn fold(grid: &Vec<Vec<char>>, axis: Axis, line: usize) -> Vec<Vec<char>> {
    // Get the dimensions of the incoming array
    let input_rows: usize = grid.len();
    let input_columns: usize = grid[0].len();

    match axis {
        Axis::X => {
            let mut output_vec: Vec<Vec<char>> = vec![vec![' '; input_columns]; (input_rows - 1) / 2];
            for row in 1..(line + 1) {
                for column in 0..input_columns {
                    if grid[line - row][column] == '#' || grid[line + row][column] == '#' {
                        *&mut output_vec[line-row][column] = '#';
                    } else {
                        continue;
                    }
                }
            }
            return output_vec;
        }
        Axis::Y => {
            let mut output_vec: Vec<Vec<char>> = vec![vec![' '; (input_columns - 1) / 2]; input_rows];
            for row in 0..input_rows {
                for column in 1..(line + 1) {
                    if grid[row][line - column] == '#' || grid[row][line + column] == '#' {
                        *&mut output_vec[row][line - column] = '#';
                    } else {
                        continue;
                    }
                }
            }
            return output_vec;
        }
    }
}