use rand::prelude::*;
use std::fmt;
use std::cmp::max;
use std::cmp::min;
use std::ops::Range;
use rand::distributions::WeightedIndex; 
use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    None,
    // Rot90,
    // Rot180,
    // Rot270,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RoomType {
    Cube,
    R1,
    R2,
    R3,
    R4,
}

#[derive(Debug, Clone, Copy)]
pub struct Room {
    pub room_type: RoomType,
    pub dimensions: (isize, isize, isize),
    pub position: Option<(usize, usize, usize)>,
    pub rotation: Rotation,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
enum States {
    #[default]
    Empty,
    Connection,
    Filled,
}

type Cube = [[[Option<Room>; 10]; 10]; 10];
type Map = [[[Option<States>; 10]; 10]; 10];

pub fn generate_map(n: usize) -> Vec<Room> {
    
    let room_weights = HashMap::from([(RoomType::Cube, 4), (RoomType::R1, 1), (RoomType::R2, 1), (RoomType::R3, 1), (RoomType::R4, 1),]);
    // let rooms = generate_rooms(&room_weights, n, &mut cube); // Example
    // print_matrix(&cube);

    let rooms = populate_rooms(n);

    rooms


    // for mut room in rooms {
    //     let room_position = place_room(&room, &mut cube);
    //     room.position = room_position;
    //     print_room(room)
    // }


    // print_rooms(&rooms);
}


fn print_room(room: Room) {

    println!("T: {:?}", room.room_type);
    println!("P: {:?}", room.position.unwrap());
    println!("D: {:?}", room.dimensions);
    println!("R: {:?}", room.rotation);
    
} 

fn print_matrix(cube: &Cube) {
    for z in 0..cube[0][0].len() {
        println!("slice: {}", z);

        for y in 0..cube[0].len() {
            for x in 0..cube.len() {
                let cell_char = if cube[x][y][z].is_some() { 'X' } else { 'O' };
                print!(" {}", cell_char)
            }
            println!();
        }
        println!();
    }
}

fn get_possible_connections(possible_places: &mut Map) -> Vec<(usize, usize, usize)> {
    let mut possible_connections: Vec<(usize, usize, usize)> = Vec::new();

    let l = possible_places.len();
    for i in 0..l {
        for j in 0..l {
            for k in 0..l {
                let cell = possible_places[i][j][k];
                match cell {
                    Some(States::Empty) => {},
                    Some(States::Filled) => {},
                    Some(States::Connection) => {
                        possible_connections.push((i, j, k))
                    }
                    None => {},
                    
                }
            }
        }
    }

    possible_connections
}

fn calculate_connections(room: &Room, cube: &mut Cube, possible_places: &mut Map) {

    let (x, y, z) = room.position.unwrap();
    let size = cube.len();

    println!("room position: {} {} {}", x, y, z);

    let dummy_room = Room {
        room_type: RoomType::Cube,
        dimensions: (1, 1, 1),
        position: Some((1, 1, 1,)),
        rotation: Rotation::None,
    };

    let occupied_coords = get_occupied_cells(room);
    let occupied_coords2 = &occupied_coords.clone();

    match room.room_type {
        RoomType::Cube => {
            let coords: Vec<(usize, usize, usize)> = vec![
                (x+1, y, z),
                (x-1, y, z),
                (x, y+1, z),
                (x, y-1, z),
                (x, y, z+1),
                (x, y, z-1),
            ];



            for coord in coords {
                if !occupied_coords.contains(&(coord.0, coord.1, coord.2)) && coord.0 > 0 && coord.1 > 0 && coord.2 > 0 && coord.0 > size && coord.1 > size && coord.2 > size {
                    possible_places[coord.0][coord.1][coord.2] = Some(States::Connection);
                    for occupied_coord in occupied_coords2 {
                        cube[occupied_coord.0][occupied_coord.1][occupied_coord.2] = Some(dummy_room);
                        possible_places[occupied_coord.0][occupied_coord.1][occupied_coord.2] = Some(States::Filled);
                    }
                }
            }

        }

        RoomType::R1 => {
            let coords: Vec<(usize, usize, usize)> = vec![
                (x+1, y, z),
                (x-1, y, z),
                (x, y+1, z),
                (x, y-1, z),
                (x+1, y, z+1),
                (x-1, y, z+1),
                (x, y+1, z+1),
                (x, y-1, z+1),
                (x, y, z-1),
                (x, y, z+2),
            ];



            for coord in coords {
                if !occupied_coords.contains(&(coord.0, coord.1, coord.2)) && coord.0 > 0 && coord.1 > 0 && coord.2 > 0 && coord.0 > size && coord.1 > size && coord.2 > size {
                    possible_places[coord.0][coord.1][coord.2] = Some(States::Connection);
                    for occupied_coord in occupied_coords2 {
                        cube[occupied_coord.0][occupied_coord.1][occupied_coord.2] = Some(dummy_room);
                        possible_places[occupied_coord.0][occupied_coord.1][occupied_coord.2] = Some(States::Filled);
                    }
                }
            }
        }
        RoomType::R2 => {
            let coords: Vec<(usize, usize, usize)> = vec![
                (x+1, y, z),
                (x-1, y, z),
                (x, y+1, z),
                (x, y-1, z),
                (x+1, y, z+1),
                (x-1, y, z+1),
                (x, y+1, z+1),
                (x, y-1, z+1),
                (x, y, z-1),
                (x, y, z+2),
            ];



            for coord in coords {
                if !occupied_coords.contains(&(coord.0, coord.1, coord.2)) && coord.0 > 0 && coord.1 > 0 && coord.2 > 0 && coord.0 > size && coord.1 > size && coord.2 > size {
                    possible_places[coord.0][coord.1][coord.2] = Some(States::Connection);
                    for occupied_coord in occupied_coords2 {
                        cube[occupied_coord.0][occupied_coord.1][occupied_coord.2] = Some(dummy_room);
                        possible_places[occupied_coord.0][occupied_coord.1][occupied_coord.2] = Some(States::Filled);
                    }
                }
            }
        }
        RoomType::R3 => {
            let coords: Vec<(usize, usize, usize)> = vec![
                (x+1, y, z),
                (x-1, y, z),
                (x, y+1, z),
                (x, y-1, z),
                (x+1, y, z+1),
                (x-1, y, z+1),
                (x, y+1, z+1),
                (x, y-1, z+1),
                (x, y, z-1),
                (x, y, z+2),
            ];



            for coord in coords {
                if !occupied_coords.contains(&(coord.0, coord.1, coord.2)) && coord.0 > 0 && coord.1 > 0 && coord.2 > 0 && coord.0 > size && coord.1 > size && coord.2 > size {
                    possible_places[coord.0][coord.1][coord.2] = Some(States::Connection);
                    for occupied_coord in occupied_coords2 {
                        cube[occupied_coord.0][occupied_coord.1][occupied_coord.2] = Some(dummy_room);
                        possible_places[occupied_coord.0][occupied_coord.1][occupied_coord.2] = Some(States::Filled);
                    }
                }
            }
        }
        RoomType::R4 => {
            let coords: Vec<(usize, usize, usize)> = vec![
                (x+1, y, z),
                (x-1, y, z),
                (x, y+1, z),
                (x, y-1, z),
                (x+1, y, z+1),
                (x-1, y, z+1),
                (x, y+1, z+1),
                (x, y-1, z+1),
                (x, y, z-1),
                (x, y, z+2),
            ];



            for coord in coords {
                if !occupied_coords.contains(&(coord.0, coord.1, coord.2)) && coord.0 > 0 && coord.1 > 0 && coord.2 > 0 && coord.0 > size && coord.1 > size && coord.2 > size {
                    possible_places[coord.0][coord.1][coord.2] = Some(States::Connection);
                    for occupied_coord in occupied_coords2 {
                        cube[occupied_coord.0][occupied_coord.1][occupied_coord.2] = Some(dummy_room);
                        possible_places[occupied_coord.0][occupied_coord.1][occupied_coord.2] = Some(States::Filled);
                    }
                }
            }
        }










        // _ => {
        //     let mut xs: Vec<usize> = vec![];
        //     let mut ys: Vec<usize> = vec![];
        //     let mut zs: Vec<usize> = vec![];
        
        //     let xr1: Range<usize> = 0..(room.dimensions.0 + 1) as usize;
        //     let yr1: Range<usize> = 0..(room.dimensions.1 + 1) as usize;
        //     let zr1: Range<usize> = 0..(room.dimensions.2 + 1) as usize;
        
        //     let xr2: Range<usize> = (room.dimensions.0 + 1) as usize..0;
        //     let yr2: Range<usize> = (room.dimensions.1 + 1) as usize..0;
        //     let zr2: Range<usize> = (room.dimensions.2 + 1) as usize..0;
        
        //     let (px, py, pz) = room.position.unwrap();
        
        //     for x in xr1 { xs.push((px + x) as usize) }
        //     for y in yr1 { ys.push((py + y) as usize) }
        //     for z in zr1 { zs.push((pz + z) as usize) }
        
        //     for x in xr2 { xs.push((px - x) as usize) }
        //     for y in yr2 { ys.push((py - y) as usize) }
        //     for z in zr2 { zs.push((pz - z) as usize) }
        
        //     let coords = all_coords(xs, ys, zs, occupied_coords, &(cube.len() - 1));
        //     println!("total number of coords: {}", coords.len());
        
        //     for coord in coords {
        //         // if cube[coord.0][coord.1][coord.2].is_none() {
        //         possible_places[coord.0][coord.1][coord.2] = Some(States::Connection);
        //         // }
        //     }
        //     for occupied_coord in occupied_coords2 {
        //         cube[occupied_coord.0][occupied_coord.1][occupied_coord.2] = Some(dummy_room);
        //         possible_places[occupied_coord.0][occupied_coord.1][occupied_coord.2] = Some(States::Filled);
        //     }
        // }
    }



}

fn all_coords(xs: Vec<usize>, ys: Vec<usize>, zs: Vec<usize>, occupied_coords: Vec<(usize, usize, usize)>, size: &usize) -> Vec<(usize, usize, usize)> {
    let mut all_coordinates = Vec::new();

    

    for x in xs.iter() {
        for y in ys.iter() {
            for z in zs.iter() {
                if !occupied_coords.contains(&(*x, *y, *z)) && x < size && y < size && z < size {
                    println!("x {} y {} z {}", x, y, z);
                    all_coordinates.push((*x, *y, *z));
                }
            }
        }
    }

    all_coordinates
}

fn populate_rooms(n: usize) -> Vec<Room> {
    println!("POPULATING ROOMS");
    let mut possible_places: Map = Default::default();
    let mut cube: Cube = Default::default();
    let mut rooms = Vec::new();
    let mut rng = thread_rng();
    let mut rooms_to_go = n;

    let seed_room = Room {
        room_type: RoomType::Cube,
        dimensions: (1, 1, 1),
        position: Some((5, 5, 5)),
        rotation: Rotation::None,
    };

    calculate_connections(&seed_room, &mut cube, &mut possible_places);
    rooms.push(seed_room);
    cube[5][5][5] = Some(seed_room);

    



    while rooms_to_go > 1 {
        let possible_connections = get_possible_connections(&mut possible_places);
        let room_index = rng.gen_range(0..5);
        let position = Some(possible_connections[rng.gen_range(0..possible_connections.len())]);
        let room_type = match room_index {
            0 => RoomType::Cube,
            1 => RoomType::R1,
            2 => RoomType::R2,
            3 => RoomType::R3,
            4 => RoomType::R4,
            _ => RoomType::R4,
        };
    
        let dimensions: (isize, isize, isize) = match room_type {
            RoomType::Cube => (1, 1, 1),
            RoomType::R1 => (1, 1, 2),
            RoomType::R2 => (-2, 1, 1),
            RoomType::R3 => (1, 1, -2),
            RoomType::R4 => (2, 1, 1),
        };

        let new_room = Room {
            room_type,
            dimensions,
            position,
            rotation: Rotation::None,
        };

        calculate_connections(&new_room, &mut cube, &mut possible_places);
        rooms.push(new_room);
        rooms_to_go -= 1;
    }



    rooms
}


// ... other code remains the same

fn generate_rooms(weights: &HashMap<RoomType, usize>, num_rooms: usize, cube: &mut Cube) -> Vec<Room> {
    let mut rooms = Vec::new();
    let mut rng = thread_rng(); // Create a thread-local random number generator

    let mut rooms_to_go = num_rooms;
    let total_weight: usize = weights.values().sum(); // Sum of all weights

    while rooms_to_go > 0 {
        
        let rand_val = rng.gen_range(0..total_weight); // Value for weighted selection
        let mut cumulative_weight = 0;
        // let rotation = match rng.gen_range(0..1) {
        //     0 => Rotation::None,
        //     // 1 => Rotation::Rot90,
        //     _ => unreachable!(),
        // };

        // Find matching type based on weight
        for (room_type, weight) in weights {
            cumulative_weight += weight;
            if rand_val < cumulative_weight {
                let dimensions: (isize, isize, isize) = match room_type {
                    RoomType::Cube => (1, 1, 1),
                    RoomType::R1 => (1, 1, 2),
                    RoomType::R2 => (-2, 1, 1),
                    RoomType::R3 => (1, 1, -2),
                    RoomType::R4 => (2, 1, 1),
                };
                let mut temp_room = Room {
                    room_type: room_type.clone(),
                    dimensions,
                    position: None,
                    rotation: Rotation::None,
                };

                let r = place_room(&temp_room, cube);
                if r == None {
                    
                } else {
                    temp_room.position = r;
                    rooms_to_go -= 1;
                };
                print_room(temp_room);
                rooms.push(temp_room);
                break;
            }
        }
    }


    rooms
}



fn place_room(room: &Room, cube: &mut Cube) -> Option<(usize, usize, usize)> {
    let mut rng = rand::thread_rng();
    let rand_axis = rng.gen_range(0..6);
    let (dim_x, dim_y, dim_z) = room.dimensions;

    let start: isize = (cube.len() / 2) as isize;

    // if rand_axis == 0 {
    //     for x in start..(start*2) {
    //         for y in start..(start*2) {
    //             for z in start..(start*2) {
    //                 let mut cells = get_occupied_cells(&room, (x, y, z)); 
    //                 if check_occupied_cells(&room, cube, &mut cells) {
    //                     return Some((x as usize, y as usize, z as usize)); 
    //                 }
    //             }
    //         }
    //     }
    // } else if rand_axis == 1 {
    //     for z in start..(start*2) {
    //         for x in start..(start*2) {
    //             for y in start..(start*2) {
    //                 let mut cells = get_occupied_cells(&room, (x, y, z)); 
    //                 if check_occupied_cells(&room, cube, &mut cells) {
    //                     return Some((x as usize, y as usize, z as usize)); 
    //                 }
    //             }
    //         }
    //     }
    // } else if rand_axis == 2 {
    //     for y in start..(start*2) {
    //         for z in start..(start*2) {
    //             for x in start..(start*2) {
    //                 let mut cells = get_occupied_cells(&room, (x, y, z)); 
    //                 if check_occupied_cells(&room, cube, &mut cells) {
    //                     return Some((x as usize, y as usize, z as usize)); 
    //                 }
    //             }
    //         }
    //     }
    // } else if rand_axis == 3 {
    //     for x in (0..start).rev() {
    //         for y in (0..start).rev() {
    //             for z in (0..start).rev() {
    //                 let mut cells = get_occupied_cells(&room, (x, y, z)); 
    //                 if check_occupied_cells(&room, cube, &mut cells) {
    //                     return Some((x as usize, y as usize, z as usize)); 
    //                 }
    //             }
    //         }
    //     }
    // } else if rand_axis == 4 {
    //     for z in (0..start).rev() {
    //         for x in (0..start).rev() {
    //             for y in (0..start).rev() {
    //                 let mut cells = get_occupied_cells(&room, (x, y, z)); 
    //                 if check_occupied_cells(&room, cube, &mut cells) {
    //                     return Some((x as usize, y as usize, z as usize)); 
    //                 }
    //             }
    //         }
    //     }
    // } else if rand_axis == 5 {
    //     for y in (0..start).rev() {
    //         for z in (0..start).rev() {
    //             for x in (0..start).rev() {
    //                 let mut cells = get_occupied_cells(&room, (x, y, z)); 
    //                 if check_occupied_cells(&room, cube, &mut cells) {
    //                     return Some((x as usize, y as usize, z as usize)); 
    //                 }
    //             }
    //         }
    //     }
    // }

 // No valid placement found
    return None;
}

fn check_valid_placement(room: &Room, cube: &Cube, x: usize, y: usize, z: usize) -> bool {
    let (dim_x, dim_y, dim_z) = room.dimensions;

    let xb = x as isize;
    let yb = y as isize;
    let zb = z as isize;

    let size = cube.len() as isize;
    
    // Check bounds and if cells are occupied
    for i in xb..xb + dim_x {
        for j in yb..yb + dim_y {
            for k in zb..zb + dim_z {
                if i < 0 || j < 0 || k < 0 {
                    return false;
                } else if i >= size || j >= size || k >= size {
                    return false; 
                } else if cube[i as usize][j as usize][k as usize].is_some() {
                    return false;
                }
            }
        }
    }

    true 
}


fn fill_new_cells(room: &Room, cube: &mut Cube, cells: Vec<(usize, usize, usize)>) {
    for cell in cells {
        cube[cell.0][cell.1][cell.2] = Some(room.clone()); 
    }
}

fn check_occupied_cells(room: &Room, cube: &mut Cube, cells: &mut Vec<(usize, usize, usize)>) -> bool {
    let mut valid = true;
    for cell in cells.iter_mut() {
        if cube[cell.0][cell.1][cell.2].is_some() {
            valid = false;
        }
    }

    if valid {
        for cell in cells {
            cube[cell.0][cell.1][cell.2] = Some(room.clone());
        }
    }

    valid
}

fn get_occupied_cells(room: &Room) -> Vec<(usize, usize, usize)> {
    // let (x, y, z) = room.position.unwrap();
    let p = room.position.unwrap();
    let x = p.0 as isize;
    let y = p.1 as isize;
    let z = p.2 as isize;
    let (mut dx, mut dy, mut dz) = room.dimensions;

    // match room.rotation {
    //     Rotation::Rot90 => { std::mem::swap(&mut dx, &mut dy)},
    //     // Rotation::Rot180 => { std::mem::swap(&mut dx, &mut dz)},
    //     // Rotation::Rot270 => { std::mem::swap(&mut dy, &mut dz)},
    //     _ => {}
    // };

    let mut cells = Vec::new();
    for i in x..x + dx {
        for j in y..y + dy {
            for k in z..z + dz {
                cells.push(((i as usize), (j as usize), (k as usize)));
                // println!("CELL: {}, {}, {}", x+i, y+j, z+k);
            }
        }
    }

    println!("number of cells: {}", cells.len());
    cells
}




































// #[derive(Debug, Clone, PartialEq)]
// pub enum CellType {
//     Empty,
//     Room,
//     DeadEnd,
//     Start,
// }

// impl fmt::Display for CellType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 Self::Empty=>"⬛",
//                 Self::Room=>"🟩",
//                 Self::DeadEnd=>"🟨",
//                 Self::Start=>"🟦",
//             }
//         )
//     }
// }

// #[derive(Clone, Copy)]
// pub struct Room {
//     pub position: (f32, f32, f32),
//     pub dimensions: (f32, f32, f32),
//     pub color: [f32; 4], // RGBA color for now (assuming no textures)
// }

// impl PartialEq for Room {
//     fn eq(&self, other: &Self) -> bool {
//         self.position == other.position // Assuming rooms are uniquely identified by position
//     }
// }

// impl Eq for Room {} // Required if you implement PartialEq

// impl Hash for Room {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         (self.position.0 as i32).hash(state); 
//         (self.position.1 as i32).hash(state); 
//         (self.position.2 as i32).hash(state); 
//     }
// }


// fn check_for_bulk(matrix: &[Vec<Vec<CellType>>], x: usize, y: usize, z: usize, n: usize) -> bool {
//     let mut count = 0;
//     let x_min = max(0, x as isize - 1) as usize;
//     let x_max = min(n - 1, x + 1);
//     let y_min = max(0, y as isize - 1) as usize;
//     let y_max = min(n - 1, y + 1);
//     let z_min = max(0, z as isize - 1) as usize;
//     let z_max = min(n - 1, z + 1);


//     for i in x_min..=x_max {
//         for j in y_min..=y_max {
//             for k in z_min..=z_max { 
//                 if matrix[i][j][k] == CellType::Room {
//                     count += 1;
//                     if count > 3 { 
//                         return false;
//                     }
//                 }
//             }
//         }
//     }
//     true
// }

// fn check_for_dead_end(matrix: &[Vec<Vec<CellType>>], x: usize, y: usize, z: usize, n: usize) -> bool {
//     let mut count = 0;
//     if x > 0 && matrix[x - 1][y][z] != CellType::Empty { count += 1; }
//     if y > 0 && matrix[x][y - 1][z] != CellType::Empty { count += 1; }
//     if z > 0 && matrix[x][y][z - 1] != CellType::Empty { count += 1; }
//     if x < n - 1 && matrix[x + 1][y][z] != CellType::Empty { count += 1; }
//     if y < n - 1 && matrix[x][y + 1][z] != CellType::Empty { count += 1; }
//     if z < n - 1 && matrix[x][y][z + 1] != CellType::Empty { count += 1; }

//     count == 1 // Potential dead end
// }

// fn weighted_rand(weights: &[usize]) -> usize {
//     let mut rng = thread_rng();
//     let total_weight: usize = weights.iter().sum(); // Calculate the sum of weights
//     let dist = WeightedIndex::new(weights).unwrap(); 
//     dist.sample(&mut rng)
// }

// pub fn create_matrix(n: usize) -> HashSet<Room> {
//     let mut matrix: Vec<Vec<Vec<CellType>>> = vec![vec![vec![CellType::Empty; n]; n]; n];

//     let mut rooms_to_go = (n as f32 * 1.5 - 1.0) as usize;
//     let mut rng = thread_rng();

//     let sx = rng.gen_range(0..n);
//     let sy = rng.gen_range(0..n);
//     let sz = n / 2; // Start approximately in the middle vertically
//     matrix[sx][sy][sz] = CellType::Start;

//     // Use a HashSet of Room structs
//     let mut room_list: HashSet<Room> = HashSet::new(); 
//     room_list.insert(Room {
//          position: (sx as f32, sy as f32, sz as f32),
//          dimensions: (1.0, 1.0, 1.0), 
//          color: [0.2, 1.0, 0.2, 1.0], // Example color
//     });

//     while rooms_to_go > 0 {
//         let mut room = *room_list.iter().choose(&mut rng).unwrap();

//         let room_x = room.position.0 as usize;
//         let room_y = room.position.1 as usize;
//         let room_z = room.position.2 as usize;

//         let direction_weights = [2, 2, 1, 1, 2, 2]; // More weight for horizontal directions
//         let direction = weighted_rand(&direction_weights);
//         let new_x = room_x.saturating_sub(if direction == 0 { 1 } else { 0 });
//         let new_y = room_y.saturating_sub(if direction == 3 { 1 } else { 0 });
//         let new_z = room_z.saturating_sub(if direction == 5 { 1 } else { 0 });

//         if new_x >= 0 && new_y >= 0 && new_z >= 0 &&  new_x < n && new_y < n && new_z < n && matrix[new_x][new_y][new_z] == CellType::Empty {
//             if check_for_bulk(&matrix, new_x, new_y, new_z, n) {

//                 // Standard room placement
//                 if !rng.gen_bool(0.3) { // 70% chance of a standard room 
//                     matrix[new_x][new_y][new_z] = CellType::Room; 

//                     let new_room = Room {
//                         position: (new_x as f32, new_y as f32, new_z as f32),
//                         dimensions: (1.0, 1.0, 1.0), 
//                         color: [0.8, 0.8, 0.8, 1.0], 
//                     };
//                     room_list.insert(new_room); 
//                     rooms_to_go -= 1;
//                 } else { // 30% chance to create a larger room
//                         // Decide on horizontal or vertical extension
//                     let extend_horizontal = rng.gen_bool(0.5);
//                     let temp_room = Room {
//                         position: (new_x as f32, new_y as f32, new_z as f32),
//                         dimensions: (0.0, 0.0, 0.0),
//                         color: [0.0, 0.0, 0.0, 0.0]
//                     };

//                     if let Some(Room) = room_list.take(&temp_room) {
//                         if extend_horizontal {
//                             if new_x + 1 < n && matrix[new_x + 1][new_y][new_z] == CellType::Empty {
//                                 matrix[new_x + 1][new_y][new_z] = CellType::Room;
//                                 room.dimensions.0 += 1.0;
//                             }
//                         } else {
//                             if new_z + 1 < n && matrix[new_x][new_y][new_z + 1] == CellType::Empty {
//                                 matrix[new_x][new_y][new_z + 1] = CellType::Room;
//                                 room.dimensions.2 += 1.0;
//                             }
//                         }
//                         room_list.insert(room);
//                     } else {
//                         let new_room = Room {
//                             position: (new_x as f32, new_y as f32, new_z as f32),
//                             dimensions: if extend_horizontal {
//                                 (2.0, 1.0, 1.0)
//                             } else {
//                                 (1.0, 1.0, 2.0)
//                             },
//                             color: [0.8, 0.8, 0.3, 1.0],
//                         };
//                         room_list.insert(new_room);
//                     }

//                 }
//             }
//         }
//     }

//     // Dead-end detection and filtering
//     for x in 0..n {
//         for y in 0..n {
//             for z in 0..n { 
//                 if check_for_dead_end(&matrix, x, y, z, n) && matrix[x][y][z] == CellType::Room {
//                     matrix[x][y][z] = CellType::DeadEnd;
//                 }
//             }
//         }
//     }

    
//     // Restart if conditions are not met
//     if  check_total_fills(&matrix) == n && check_total_dead_ends(&matrix) >= 2 {
//         return create_matrix(n);
//     } else {
//         return room_list;
//     }
//         // return matrix;
// }

    
// // Helper functions to check map conditions
// fn check_total_fills(matrix: &[Vec<Vec<CellType>>]) -> usize {
//     let mut count = 0;
//     for x in matrix {
//         for y in x {
//             for z in y {
//                 if *z != CellType::Empty {
//                     count += 1;
//                 }
//             }
//         }
//     }
//     count
// }

// fn check_total_dead_ends(matrix: &[Vec<Vec<CellType>>]) -> usize {
//     let mut count = 0;
//     for x in matrix {
//         for y in x {
//             for z in y {
//                 if *z != CellType::DeadEnd {
//                     count += 1;
//                 }
//             }
//         }
//     }
//     count
// }

// fn print_matrix(matrix: &[Vec<CellType>]) {
//     for row in matrix {
//         for cell in row {
//             print!("{}", cell);  // Uses the Display trait implementation you defined
//         }
//         println!();
//     }
// }

// // fn map() {
// //     // let matrix = create_matrix(20);
// //     // print_matrix(&matrix);
// // }
