use std::fs;

// Struct for a bingo board. Holds two 2D-arrays, one for numbers 
// and one to track what numbers have been picked.
struct Board {
    numbers: [[u32; 5]; 5],
    drawn: [[bool; 5]; 5],
    has_won: bool, // Second star
}

impl Board {
    fn check_number(&mut self, number: u32) {
        for row in 0..5 {
            for column in 0..5 {
                if self.numbers[row][column] == number {
                    self.drawn[row][column] = true;
                }
            }
        }
    }
}

impl Board {
    fn check_win(&mut self) -> bool {
        // Check row per row
        for row in 0..5 {
            let mut drawn: u32 = 0;

            for column in 0..5 {
                if self.drawn[row][column] == true {
                    drawn += 1;
                }
            }
            if drawn == 5 {
                self.has_won = true;
                return true;
            } else {
                continue;
            }
        }

        // Check column per column
        for column in 0..5 {
            let mut drawn: u32 = 0;

            for row in 0..5 {
                if self.drawn[row][column] == true {
                    drawn += 1;
                }
            }
            if drawn == 5 {
                self.has_won = true;
                return true;
            } else {
                continue;
            }
        }
        return false;
    }
}

impl Board {
    fn reset_draws(&mut self) {
        self.drawn = [[false; 5]; 5];
    }
}

impl Board {
    fn copy(&self) -> Board {
        Board {
            numbers: self.numbers,
            drawn: self.drawn,
            has_won: self.has_won,
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let input_vec: Vec<&str> = contents.lines().collect();

    // The first line is a comma-separated list of drawn numbers

    let mut draw_vec: Vec<u32> = Vec::new();

    let temp_vec: Vec<&str> = input_vec[0].split(",").collect();
    for line in temp_vec {
        let temp_int: u32 = line.parse().unwrap(); // Cast to u32
        let _ = &mut draw_vec.push(temp_int); // The draw_vew vector now holds the drawn numbers
    }

    let block_vec: Vec<&str> = contents.split("\r\n\r\n").collect();

    // Now we have to turn the other lines into boards and store them in a big vector
    let mut board_vec: Vec<Board> = Vec::new();

    for block in 1..block_vec.len() {
        let temp_board: Board = new_board(block_vec[block]);
        let _ = &mut board_vec.push(temp_board);
    }

    // Now we have a vector containing all boards.

    // ⭐ First Star ⭐

    let mut win_flag: bool = false;

    for number in &draw_vec {
        for board in &mut board_vec {
            board.check_number(*number);
            if board.check_win() {
                println!("Game was won when number {} was drawn.", number);
                win_flag = true;
                let sum_of_not_drawn = count_not_drawn(board);
                println!("Magic number: {}", number * sum_of_not_drawn);
                break;
            }
        }
        if win_flag == true {
            break;
        }
    }

    // ⭐ Second Star ⭐

    for board in &mut board_vec {
        board.reset_draws();
    }

    // Declare a vector to contain all the boards that have won and a vector to contain the matching number

    let mut won_vec: Vec<Board> = Vec::new();
    let mut winning_draw: Vec<u32> = Vec::new();

    for number in &draw_vec {
        for board in &mut board_vec {
            if !board.has_won {
                board.check_number(*number);
                if board.check_win() {
                    let temp_board: Board = board.copy();
                    let temp_win_num: u32 = *number;
                    let _ = won_vec.push(temp_board);
                    let _ = winning_draw.push(temp_win_num);
                }
            }
        }       
    }
    
    println!("The last win was with number {}", winning_draw[winning_draw.len() - 1]);
    let sum_of_not_drawn = count_not_drawn(&won_vec[won_vec.len() - 1]);
    println!("Magic number for that board: {}", winning_draw[winning_draw.len() - 1] * sum_of_not_drawn);
}

fn new_board(board_string: &str) -> Board {
    let mut out_array = [[0; 5]; 5];
    let bool_array = [[false; 5]; 5];
    let string_vec: Vec<&str> = board_string.split_whitespace().collect(); // Now we have a vector with 25 strings

    for number in 0..25 {
        out_array[number / 5][number % 5] = string_vec[number].parse().unwrap();
    }
    Board {
        numbers: out_array,
        drawn: bool_array,
        has_won: false,
    }
}

fn count_not_drawn(board: &Board) -> u32 {
    let mut running_sum: u32 = 0;
    for row in 0..5 {
        for column in 0..5 {
            if board.drawn[row][column] == false {
                running_sum += board.numbers[row][column];
            }
        }
    }
    return running_sum;
}
