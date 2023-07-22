use std::fs;

// NOTE: Full of loops and all the data lives on the heap. Continually has to recreate the vector. Extremely slow.
// Can't put a 1000 x 1000 array of u8's on the stack though -> stack overflow
// Could change the coordinates to u16's and the vents to u8's to save on memory.

// TO DO: Refactoring:
// 1000 x 1000 Vector of u8's, representing how many lines pass through the point
// Add columns and rows of margin to simplify the code

struct Point {
    x: u32,
    y: u32,
    vents: u32,
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let input_vec: Vec<&str> = contents.lines().collect(); // A line looks like this: 629,581 -> 123,75

    let mut line_vec: Vec<[Point; 2]> = Vec::new(); // Vector with point pairs

    for line in input_vec {
        let temp_array = point_pair_from_string(line);
        let _ = &mut line_vec.push(temp_array);
    }

    let mut points_vec: Vec<Point> = Vec::new(); // Vector that holds points with at least one vent

    
    for line in &mut line_vec {
        draw_line(line, &mut points_vec);
    }
    
    let mut count_ventpoints: u32 = 0;

    for index in 0..points_vec.len() {
        if points_vec[index].vents >= 2 {
            count_ventpoints += 1;
        }
    }

    println!("-- ⭐ First Star ⭐ --");
    println!("Points with more than two vents: {}", count_ventpoints);

    for line in &mut line_vec {
        draw_diagonal(line, &mut points_vec);
    }

    let mut count_ventpoints: u32 = 0;

    for index in 0..points_vec.len() {
        if points_vec[index].vents >= 2 {
            count_ventpoints += 1;
        }
    }

    println!("-- ⭐ Second Star ⭐ --");
    println!("Points with more than two vents: {}", count_ventpoints);

}

fn new_point(x: u32, y: u32, vents: u32) -> Point {
    Point {
        x,
        y,
        vents,
    }
}

fn point_pair_from_string(pair_as_string: &str) -> [Point; 2] { // Incoming strings look like this: 629,581 -> 123,75
    let input_vec: Vec<&str> = pair_as_string.split(" -> ").collect(); // "629,581","123,75"
    
    let mut coord_vec: Vec<u32> = Vec::new();

    for line in input_vec {
        let temp_vec: Vec<&str> = line.split(",").collect();
        for coord in temp_vec {
            let coord_int: u32 = coord.parse().unwrap();
            let _ = &mut coord_vec.push(coord_int);
        }
    }

    let start_point: Point = new_point(coord_vec[0], coord_vec[1], 0);

    let end_point: Point = new_point(coord_vec[2], coord_vec[3], 0);

    return [start_point, end_point];
}

fn point_exists(x: u32, y: u32, point_vec: &Vec<Point>) -> bool {
    if point_vec.len() == 0 {
        return false;
    } else {
        for index in 0..point_vec.len() {
            if x == point_vec[index].x && y == point_vec[index].y {
                return true;
            }
        }
        return false;
    }
}

fn add_vent(x: u32, y: u32, point_vec: &mut Vec<Point>) {
    for index in 0..point_vec.len() {
        if x == point_vec[index].x && y == point_vec[index].y {
            point_vec[index].vents += 1;
        }
    }
}

// We have to be able to draw in both directions
// Don't assume the first point is to the left or higher than the second

fn draw_line(point_pair: &mut [Point; 2], point_vec: &mut Vec<Point>) { 
    if ( point_pair[0].x != point_pair[1].x ) && ( point_pair[0].y != point_pair[1].y ) { // Diagonal line, do nothing
        return;
    } else if point_pair[0].x == point_pair[1].x { // Vertical line, find out who is higher
        if point_pair[0].y < point_pair[1].y { // Writing down
            for y in point_pair[0].y..(point_pair[1].y + 1) {
                if point_exists(point_pair[0].x, y, &point_vec) {
                    add_vent(point_pair[0].x, y, point_vec);
                } else {
                    let temp_point: Point = new_point(point_pair[0].x, y, 1);
                    let _ = point_vec.push(temp_point);
                }
            } 
        } else { // Writing up
            for y in point_pair[1].y..(point_pair[0].y + 1) {
                if point_exists(point_pair[0].x, y, &point_vec) {
                    add_vent(point_pair[0].x, y, point_vec);
                } else {
                    let temp_point: Point = new_point(point_pair[0].x, y, 1);
                    let _ = point_vec.push(temp_point);
                }
            }
        }      
    } else if point_pair[0].y == point_pair[1].y { // Horizontal line, find out who is to the left
        if point_pair[0].x < point_pair[1].x { // Writing left to right
            for x in point_pair[0].x..(point_pair[1].x + 1) {
                if point_exists(x, point_pair[0].y, &point_vec) {
                    add_vent(x, point_pair[0].y, point_vec);
                } else {
                    let temp_point: Point = new_point(x, point_pair[0].y, 1);
                    let _ = point_vec.push(temp_point);
                }
            }
        } else { // Writing right to left
            for x in point_pair[1].x..(point_pair[0].x + 1) {
                if point_exists(x, point_pair[0].y, &point_vec) {
                    add_vent(x, point_pair[0].y, point_vec);
                } else {
                    let temp_point: Point = new_point(x, point_pair[0].y, 1);
                    let _ = point_vec.push(temp_point);
                }
            }
        }
    }
}

fn draw_diagonal(point_pair: &mut [Point; 2], point_vec: &mut Vec<Point>) {
    if point_pair[0].x == point_pair[1].x || point_pair[0].y == point_pair[1].y { // Not a diagonal line.
        return;
    } else { // We want to go RIGHT and DOWN or RIGHT and UP
        if point_pair[0].x < point_pair[1].x { // Point one is to the LEFT of point two
            if point_pair[0].y < point_pair[1].y { // Point one is HIGHER than point two
                for coord in 0..(point_pair[1].x - point_pair[0].x + 1) {
                    if point_exists(point_pair[0].x + coord, point_pair[0].y + coord, &point_vec) {
                        add_vent(point_pair[0].x + coord, point_pair[0].y + coord, point_vec);
                    } else {
                        let temp_point: Point = new_point(point_pair[0].x + coord, point_pair[0].y + coord, 1);
                        let _ = point_vec.push(temp_point);
                    }
                }
            } else { // Point one is LOWER than point two
                for coord in 0..(point_pair[1].x - point_pair[0].x + 1) {
                    if point_exists(point_pair[0].x + coord, point_pair[0].y - coord, &point_vec) {
                        add_vent(point_pair[0].x + coord, point_pair[0].y - coord, point_vec);
                    } else {
                        let temp_point: Point = new_point(point_pair[0].x + coord, point_pair[0].y - coord, 1);
                        let _ = point_vec.push(temp_point);
                    }
                }
            }
        } else { // Point one is to the RIGHT of point two
            if point_pair[0].y < point_pair[1].y { // Point one is HIGHER than point two
                for coord in 0..(point_pair[0].x - point_pair[1].x + 1) {
                    if point_exists(point_pair[0].x - coord, point_pair[0].y + coord, &point_vec) {
                        add_vent(point_pair[0].x - coord, point_pair[0].y + coord, point_vec);
                    } else {
                        let temp_point: Point = new_point(point_pair[0].x - coord, point_pair[0].y + coord, 1);
                        let _ = point_vec.push(temp_point);
                    }
                }
            } else { // Point one is LOWER than point two
                for coord in 0..(point_pair[0].x - point_pair[1].x + 1) {
                    if point_exists(point_pair[0].x - coord, point_pair[0].y - coord, &point_vec) {
                        add_vent(point_pair[0].x - coord, point_pair[0].y - coord, point_vec);
                    } else {
                        let temp_point: Point = new_point(point_pair[0].x - coord, point_pair[0].y - coord, 1);
                        let _ = point_vec.push(temp_point);
                    }
                }
            }
        }
    }
}