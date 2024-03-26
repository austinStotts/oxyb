use rand::prelude::*;
use std::fmt;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use rand::distributions::WeightedIndex; 


#[derive(Debug, Clone, PartialEq)]
pub enum CellType {
    Empty,
    Room,
    DeadEnd,
    Start,
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty=>"⬛",
                Self::Room=>"🟩",
                Self::DeadEnd=>"🟨",
                Self::Start=>"🟦",
            }
        )
    }
}

struct Room {
    location: (usize, usize),
}

fn check_for_bulk(matrix: &[Vec<Vec<CellType>>], x: usize, y: usize, z: usize, n: usize) -> bool {
    let mut count = 0;
    let x_min = max(0, x as isize - 1) as usize;
    let x_max = min(n - 1, x + 1);
    let y_min = max(0, y as isize - 1) as usize;
    let y_max = min(n - 1, y + 1);
    let z_min = max(0, z as isize - 1) as usize;
    let z_max = min(n - 1, z + 1);


    for i in x_min..=x_max {
        for j in y_min..=y_max {
            for k in z_min..=z_max { 
                if matrix[i][j][k] == CellType::Room {
                    count += 1;
                    if count > 3 { 
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn check_for_dead_end(matrix: &[Vec<Vec<CellType>>], x: usize, y: usize, z: usize, n: usize) -> bool {
    let mut count = 0;
    if x > 0 && matrix[x - 1][y][z] != CellType::Empty { count += 1; }
    if y > 0 && matrix[x][y - 1][z] != CellType::Empty { count += 1; }
    if z > 0 && matrix[x][y][z - 1] != CellType::Empty { count += 1; }
    if x < n - 1 && matrix[x + 1][y][z] != CellType::Empty { count += 1; }
    if y < n - 1 && matrix[x][y + 1][z] != CellType::Empty { count += 1; }
    if z < n - 1 && matrix[x][y][z + 1] != CellType::Empty { count += 1; }

    count == 1 // Potential dead end
}

fn weighted_rand(weights: &[usize]) -> usize {
    let mut rng = thread_rng();
    let total_weight: usize = weights.iter().sum(); // Calculate the sum of weights
    let dist = WeightedIndex::new(weights).unwrap(); 
    dist.sample(&mut rng)
}

pub fn create_matrix(n: usize) -> Vec<Vec<Vec<CellType>>> {
    let mut matrix: Vec<Vec<Vec<CellType>>> = 
        vec![vec![vec![CellType::Empty; n]; n]; n];

    let mut rooms_to_go = (n as f32 * 1.5 - 1.0) as usize;
    let mut rng = thread_rng();

    let sx = rng.gen_range(0..n);
    let sy = rng.gen_range(0..n);
    let sz = n / 2;
    matrix[sx][sy][sz] = CellType::Start;

    let mut room_list: HashSet<(usize, usize, usize)> = HashSet::new();
    room_list.insert((sx, sy, sz));

    while rooms_to_go > 0 {
        let (room_x, room_y, room_z) = *room_list.iter().choose(&mut rng).unwrap();

        let direction_weights = [2, 2, 1, 1, 2, 2]; // More weight for horizontal directions
        let direction = weighted_rand(&direction_weights);
        let new_x = room_x.saturating_sub(if direction == 0 { 1 } else { 0 });
        let new_y = room_y.saturating_sub(if direction == 3 { 1 } else { 0 });
        let new_z = room_z.saturating_sub(if direction == 5 { 1 } else { 0 });

        if new_x < n && new_y < n &&  new_z < n && matrix[new_x][new_y][new_z] == CellType::Empty {
            if check_for_bulk(&matrix, new_x, new_y, new_z, n) {
                matrix[new_x][new_y][new_z] = CellType::Room;
                room_list.insert((new_x, new_y, new_z)); 
                rooms_to_go -= 1;
            }
        }
    }

    // Dead-end detection and filtering
    for x in 0..n {
        for y in 0..n {
            for z in 0..n { 
                if check_for_dead_end(&matrix, x, y, z, n) && matrix[x][y][z] == CellType::Room {
                    matrix[x][y][z] = CellType::DeadEnd;
                }
            }
        }
    }

    
    // Restart if conditions are not met
    if  check_total_fills(&matrix) == n && check_total_dead_ends(&matrix) >= 2 {
        return create_matrix(n);
    } else {
        return matrix;
    }
        // return matrix;
}

    
// Helper functions to check map conditions
fn check_total_fills(matrix: &[Vec<Vec<CellType>>]) -> usize {
    let mut count = 0;
    for x in matrix {
        for y in x {
            for z in y {
                if *z != CellType::Empty {
                    count += 1;
                }
            }
        }
    }
    count
}

fn check_total_dead_ends(matrix: &[Vec<Vec<CellType>>]) -> usize {
    let mut count = 0;
    for x in matrix {
        for y in x {
            for z in y {
                if *z != CellType::DeadEnd {
                    count += 1;
                }
            }
        }
    }
    count
}

fn print_matrix(matrix: &[Vec<CellType>]) {
    for row in matrix {
        for cell in row {
            print!("{}", cell);  // Uses the Display trait implementation you defined
        }
        println!();
    }
}

// fn map() {
//     // let matrix = create_matrix(20);
//     // print_matrix(&matrix);
// }
