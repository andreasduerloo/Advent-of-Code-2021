use std::fs;

fn main() {
    let mut school = [0 as u64; 9]; // Empty fish array. Every 'box' is a group of fish at the same point in their cycle.

    // Populate this array with the initial situation
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let string_vec: Vec<&str> = contents.split(",").collect();

    let mut fish_vec: Vec<u8> = Vec::new();

    for index in 0..string_vec.len() {
        let fish: u8 = string_vec[index].trim().parse().unwrap();
        let _ = &mut fish_vec.push(fish);
    }

    for index in 0..fish_vec.len() {
        school[fish_vec[index as usize] as usize] += 1;
    }

    // DEBUG -> looks fine.
    for day in 0..9 {
        println!("Day {} contains {} fish at start", day, school[day]);
    }
 
    for day in 0..256 {
        spawn(day % 9, &mut school);

        if day == 79 {
            let mut total_fish: u64 = 0;
            for day_now in 0..9 {
                *&mut total_fish += school[day_now] as u64;
            }
            println!("-- ⭐ First Star ⭐ --\nTotal fish after 80 days: {}", total_fish);
        }

        if day == 255 {
            let mut total_fish: u64 = 0;
            for day_now in 0..9 {
                *&mut total_fish += school[day_now] as u64;
            }
            println!("-- ⭐ Second Star ⭐ --\nTotal fish after 256 days: {}", total_fish);
        }

    }
}

fn spawn(day: u32, school: &mut [u64]) {
    // Days go from 0 -> 8, once the spawn day hits the position of a given fish, it will spawn.
    // Spawn day starts at 0, which is empty.

    // Figure out what block new fish will spawn in
    let new_fish_day: u32 = ( day + 7 ) % 9;

    school[new_fish_day as usize] += school[day as usize];
}
