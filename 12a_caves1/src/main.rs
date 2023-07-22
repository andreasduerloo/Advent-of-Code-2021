use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file"); 
    let line_vec: Vec<&str> = contents.lines().collect();

    let mut cave_vec: Vec<Vec<&str>> = Vec::new();
    let mut route_vec: Vec<Vec<&str>> = Vec::new();

    // Populate the vector-of-vectors

    for line in &line_vec {
        let temp_vec: Vec<&str> = line.split("-").collect();
        for cave in &temp_vec {
            if cave_vec.len() != 0 {
                for index in 0..cave_vec.len() {
                    if cave_vec[index][0] == *cave { // There's already an array for this cave
                        break;
                    }
                    if index == cave_vec.len() - 1 { // We made it to the end without finding the cave
                        let mut new_vec: Vec<&str> = Vec::new();
                        let _ = &mut new_vec.push(cave);
                        let _ = &mut cave_vec.push(new_vec);
                    }
                }
            }
            else {
                let mut new_vec: Vec<&str> = Vec::new();
                let _ = &mut new_vec.push(cave);
                let _ = &mut cave_vec.push(new_vec);
            }
        }
    }

    // Now we put each cave's neighbors in the arrays.
    for index in 0..cave_vec.len() {
        for line in &line_vec {
            let temp_vec: Vec<&str> = line.split("-").collect();

            if cave_vec[index][0] == temp_vec[0] {
                if ! cave_vec[index].contains(&temp_vec[1]) {
                    let _ = &mut cave_vec[index].push(temp_vec[1]);
                }
            } else if cave_vec[index][0] == temp_vec[1] {
                if ! cave_vec[index].contains(&temp_vec[0]) {
                    let _ = &mut cave_vec[index].push(temp_vec[0]);
                }
            }
        }
    }

    // Start at start and all its neighbours, then loop until the last element is 'end'
    let start_index: usize = find_index("start", &cave_vec);

    for index in 1..cave_vec[start_index].len() {
        let mut temp_route: Vec<&str> = Vec::new();
        let _ = temp_route.push(cave_vec[start_index][0]);
        let _ = temp_route.push(cave_vec[start_index][index]);
        let _ = &mut route_vec.push(temp_route);
    }

    let mut routes_to_check = count_unfinished_routes(&route_vec);

    while routes_to_check != 0 {
        for index in 0..route_vec.len() {
            if route_vec[index][route_vec[index].len() - 1] != "end" && route_vec[index][route_vec[index].len() - 1] != "to_remove" { // This route is not finished, make a route for each of its neighbors, then remove the original
                let cave_index: usize = find_index(route_vec[index][route_vec[index].len() - 1], &cave_vec);
                for neighbor_index in 1..cave_vec[cave_index].len() {
                    if cave_vec[cave_index][neighbor_index] == cave_vec[cave_index][neighbor_index].to_lowercase() && ! route_vec[index].contains(&cave_vec[cave_index][neighbor_index]) {
                        let mut temp_route: Vec<&str> = vec!["a"; route_vec[index].len()]; // Initialize the vector to the right size for the copy
                        temp_route.copy_from_slice(&route_vec[index][..]);
                        let _ = &mut route_vec.push(temp_route);
                        let new_index: usize = route_vec.len() - 1;
                        let _ = &mut route_vec[new_index].push(cave_vec[cave_index][neighbor_index]);
                    } else if cave_vec[cave_index][neighbor_index] == cave_vec[cave_index][neighbor_index].to_uppercase() {
                        let mut temp_route: Vec<&str> = vec!["a"; route_vec[index].len()];
                        temp_route.copy_from_slice(&route_vec[index][..]);
                        let _ = &mut route_vec.push(temp_route);
                        let new_index: usize = route_vec.len() - 1;
                        let _ = &mut route_vec[new_index].push(cave_vec[cave_index][neighbor_index]);
                    }
                }
                let _ = &mut route_vec[index].push("to_remove");
            }
        }

        *&mut routes_to_check = count_unfinished_routes(&route_vec);
    }

    // Count the routes that go from start to finish without passing through the same small cave twice
    let mut good_route_count: u32 = 0;

    for route in &route_vec {
        if route[0] == "start" && route[route.len() - 1] == "end" {
            let mut good_route: bool = true;
            for cave in route {
                if *cave == cave.to_lowercase() {
                    let passed_by_cave = count_cave(cave, &route);
                    if passed_by_cave >= 2 {
                        *&mut good_route = false;
                    }
                }
            }
            if good_route {
                *&mut good_route_count += 1;
            }
        }
    }

    println!("--- ⭐ First Star ⭐ ---");
    println!("Good routes: {}", good_route_count);

    // Second star: adapt logic so that it goes back to another lowercase cave ONCE (not start, not end)
}

fn find_index(cave: &str, input_vec: &Vec<Vec<&str>>) -> usize {
    for index in 0..input_vec.len() {
        if input_vec[index][0] == cave {
            return index;
        }
    }
    return 0;
}

fn count_unfinished_routes(input_vec: &Vec<Vec<&str>>) -> u32 {
    let mut count: u32 = 0;

    for index in 0..input_vec.len() {
        if input_vec[index][input_vec[index].len() - 1] != "end" && input_vec[index][input_vec[index].len() - 1] != "to_remove" {
            *&mut count += 1;
        }
    }

    return count;
}

fn count_cave(cave: &str, route: &Vec<&str>) -> u32 {
    let mut count: u32 = 0;
    for stop in route {
        if *stop == cave {
            *&mut count += 1;
        }
    }
    return count;
}