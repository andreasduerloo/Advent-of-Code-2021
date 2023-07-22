use std::fs;

struct Location {
    horizontal: u32,
    fuel: u32,
}

impl Location {
    fn new(horizontal: u32, fuel: u32) -> Location {
        Location {
            horizontal,
            fuel,
        }
    }
}

enum Consumption {
    Constant,
    Increasing,
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let string_vec: Vec<&str> = contents.split(",").collect();

    let mut crab_vec: Vec<u32> = Vec::new();

    for index in 0..string_vec.len() {
        let crab: u32 = string_vec[index].trim().parse().unwrap();
        let _ = &mut crab_vec.push(crab);
    }

    let max_position: u32 = find_max(&crab_vec);

    // First star

    let mut locations: Vec<Location> = Vec::new();
    
    for point in 0..max_position {
        let mut temp_location: Location = Location::new(point, 0);
        for crab in &crab_vec {
            measure_fuel(*crab, point, &mut temp_location, Consumption::Constant);
        }
        let _ = locations.push(temp_location);
    }

    let winning_location: Location = find_lowest(&locations);

    println!("-- ⭐ First Star ⭐ --\nWinning location: {}, fuel use: {}", winning_location.horizontal, winning_location.fuel);

    // Second star

    let mut locations2: Vec<Location> = Vec::new();

    for point in 0..max_position {
        let mut temp_location: Location = Location::new(point, 0);
        for crab in &crab_vec {
            measure_fuel(*crab, point, &mut temp_location, Consumption::Increasing);
        }
        let _ = locations2.push(temp_location);
    }

    let winning_location2: Location = find_lowest(&locations2);

    println!("-- ⭐ Second Star ⭐ --\nWinning location: {}, fuel use: {}", winning_location2.horizontal, winning_location2.fuel);

}

fn measure_fuel(current_location: u32, point_to_try: u32, location: &mut Location, rule: Consumption) {

    match rule {
        Consumption::Constant => {
            if current_location > point_to_try {
                let distance: u32 = current_location - point_to_try;
                *&mut location.fuel += distance;
            } else if current_location < point_to_try {
                let distance: u32 = point_to_try - current_location;
                *&mut location.fuel += distance;
            }
        },
        Consumption::Increasing => {
            if current_location > point_to_try {
                let distance: u32 = current_location - point_to_try;
                *&mut location.fuel += calculate_fuel_use(distance);
            } else if current_location < point_to_try {
                let distance: u32 = point_to_try - current_location;
                *&mut location.fuel += calculate_fuel_use(distance);
            }
        }
    }
}

fn find_max(input_vec: &Vec<u32>) -> u32 {
    let mut highest_element: u32 = 0;

    for element in 0..input_vec.len() {
        if input_vec[element] > highest_element {
            *&mut highest_element = input_vec[element] as u32;
        }
    }

    return highest_element;
}

fn find_lowest(location_vec: &Vec<Location>) -> Location {
    let mut lowest_fuel: Location = Location::new(0, u32::MAX);
    for index in 0..location_vec.len() {
        if location_vec[index].fuel < lowest_fuel.fuel {
            lowest_fuel.horizontal = location_vec[index].horizontal;
            lowest_fuel.fuel = location_vec[index].fuel;
        }
    }
    return lowest_fuel;
}

fn calculate_fuel_use(distance: u32) -> u32 {
    let mut fuel_used: u32 = 0;
    if distance == 0 {
        return 0;
    } else {
        for tick in 0..(distance + 1) {
            fuel_used += tick;
        }
        return fuel_used;
    }
}