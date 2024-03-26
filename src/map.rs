use rand::prelude::*;
use std::fmt;
use std::cmp::max;
use std::cmp::min;
use rand::distributions::WeightedIndex; 
use std::collections::HashSet;
use std::hash::{Hash, Hasher};


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

#[derive(Clone, Copy)]
pub struct Room {
    pub position: (f32, f32, f32),
    pub dimensions: (f32, f32, f32),
    pub color: [f32; 4], // RGBA color for now (assuming no textures)
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position // Assuming rooms are uniquely identified by position
    }
}

impl Eq for Room {} // Required if you implement PartialEq

impl Hash for Room {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.position.0 as i32).hash(state); 
        (self.position.1 as i32).hash(state); 
        (self.position.2 as i32).hash(state); 
    }
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

pub fn create_matrix(n: usize) -> HashSet<Room> {
    let mut matrix: Vec<Vec<Vec<CellType>>> = vec![vec![vec![CellType::Empty; n]; n]; n];

    let mut rooms_to_go = (n as f32 * 1.5 - 1.0) as usize;
    let mut rng = thread_rng();

    let sx = rng.gen_range(0..n);
    let sy = rng.gen_range(0..n);
    let sz = n / 2; // Start approximately in the middle vertically
    matrix[sx][sy][sz] = CellType::Start;

    // Use a HashSet of Room structs
    let mut room_list: HashSet<Room> = HashSet::new(); 
    room_list.insert(Room {
         position: (sx as f32, sy as f32, sz as f32),
         dimensions: (1.0, 1.0, 1.0), 
         color: [0.8, 0.8, 0.8, 1.0], // Example color
    });

    while rooms_to_go > 0 {
        let mut room = *room_list.iter().choose(&mut rng).unwrap();

        let room_x = room.position.0 as usize;
        let room_y = room.position.1 as usize;
        let room_z = room.position.2 as usize;

        let direction_weights = [2, 2, 1, 1, 2, 2]; // More weight for horizontal directions
        let direction = weighted_rand(&direction_weights);
        let new_x = room_x.saturating_sub(if direction == 0 { 1 } else { 0 });
        let new_y = room_y.saturating_sub(if direction == 3 { 1 } else { 0 });
        let new_z = room_z.saturating_sub(if direction == 5 { 1 } else { 0 });

        if new_x >= 0 && new_y >= 0 && new_z >= 0 &&  new_x < n && new_y < n && new_z < n && matrix[new_x][new_y][new_z] == CellType::Empty {
            if check_for_bulk(&matrix, new_x, new_y, new_z, n) {

                // Standard room placement
                if !rng.gen_bool(0.3) { // 70% chance of a standard room 
                    matrix[new_x][new_y][new_z] = CellType::Room; 

                    let new_room = Room {
                        position: (new_x as f32, new_y as f32, new_z as f32),
                        dimensions: (1.0, 1.0, 1.0), 
                        color: [0.8, 0.8, 0.8, 1.0], 
                    };
                    room_list.insert(new_room); 
                    rooms_to_go -= 1;
                } else { // 30% chance to create a larger room
                        // Decide on horizontal or vertical extension
                    let extend_horizontal = rng.gen_bool(0.5);
                    let temp_room = Room {
                        position: (new_x as f32, new_y as f32, new_z as f32),
                        dimensions: (0.0, 0.0, 0.0),
                        color: [0.0, 0.0, 0.0, 0.0]
                    };

                    if let Some(Room) = room_list.take(&temp_room) {
                        if extend_horizontal {
                            if new_x + 1 < n && matrix[new_x + 1][new_y][new_z] == CellType::Empty {
                                matrix[new_x + 1][new_y][new_z] = CellType::Room;
                                room.dimensions.0 += 1.0;
                            }
                        } else {
                            if new_z + 1 < n && matrix[new_x][new_y][new_z + 1] == CellType::Empty {
                                matrix[new_x][new_y][new_z + 1] = CellType::Room;
                                room.dimensions.2 += 1.0;
                            }
                        }
                        room_list.insert(room);
                    } else {
                        let new_room = Room {
                            position: (new_x as f32, new_y as f32, new_z as f32),
                            dimensions: if extend_horizontal {
                                (2.0, 1.0, 1.0)
                            } else {
                                (1.0, 1.0, 2.0)
                            },
                            color: [0.8, 0.8, 0.8, 1.0],
                        };
                        room_list.insert(new_room);
                    }

                }
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
        return room_list;
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
