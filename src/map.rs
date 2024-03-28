use bevy::prelude::*;
use bevy::transform::commands;
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
    pub dimensions: (f32, f32, f32),
    pub position: Option<(usize, usize, usize)>,
    pub color: [f32; 4],
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



// pub struct MapGenerator;

// impl Plugin for MapGenerator {
//     fn build(
//         &self, 
//         app: &mut App,
//         commands: &mut Commands,
//     ) {
        
//     }
// }



pub fn generate_map(n: usize) -> Vec<Room> {
    let rooms = populate_rooms(n);

    rooms
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
                        possible_connections.push((i, j, k));
                        // println!("{} {} {}", i, j, k);
                    }
                    None => {},
                    
                }
            }
        }
    }

    possible_connections
}

fn calculate_connections(room: &Room, cube: &mut Cube, possible_places: &mut Map, occupied_coords: Vec<(usize, usize, usize)>) {

    let (x, y, z) = room.position.unwrap();
    let size = cube.len();

    println!("room position: {} {} {}", x, y, z);
    println!("SIZE: {}", size);
    let dummy_room = Room {
        room_type: RoomType::Cube,
        dimensions: (1.0, 1.0, 1.0),
        position: Some((1, 1, 1,)),
        color: [0.2, 1.0, 0.2, 1.0],
        rotation: Rotation::None,
    };

    // let occupied_coords = get_occupied_cells(room);

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
                if !occupied_coords.contains(&(coord.0, coord.1, coord.2))
                && coord.0 > 0 && coord.1 > 0 && coord.2 > 0
                && coord.0 < size && coord.1 < size && coord.2 < size
                {
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
                if !occupied_coords.contains(&(coord.0, coord.1, coord.2))
                && coord.0 > 0 && coord.1 > 0 && coord.2 > 0
                && coord.0 < size && coord.1 < size && coord.2 < size {
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
                (x, y+1, z),
                (x, y-1, z),
                (x, y, z+1),
                (x, y, z-1),
                (x-1, y+1, z),
                (x-1, y-1, z),
                (x-1, y, z+1),
                (x-1, y, z+1),
                (x+1, y, z),
                (x-2, y, z),
            ];
            for coord in coords {
                if !occupied_coords.contains(&(coord.0, coord.1, coord.2))
                && coord.0 > 0 && coord.1 > 0 && coord.2 > 0
                && coord.0 < size && coord.1 < size && coord.2 < size {
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
                (x+1, y, z-1),
                (x-1, y, z-1),
                (x, y+1, z-1),
                (x, y-1, z-1),
                (x, y, z+1),
                (x, y, z-2),
            ];
            for coord in coords {
                if !occupied_coords.contains(&(coord.0, coord.1, coord.2))
                && coord.0 > 0 && coord.1 > 0 && coord.2 > 0 
                && coord.0 < size && coord.1 < size && coord.2 < size {
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
                (x, y+1, z),
                (x, y-1, z),
                (x, y, z+1),
                (x, y, z-1),
                (x+1, y+1, z),
                (x+1, y-1, z),
                (x+1, y, z+1),
                (x+1, y, z+1),
                (x-1, y, z),
                (x+2, y, z),
            ];
            for coord in coords {
                if !occupied_coords.contains(&(coord.0, coord.1, coord.2))
                && coord.0 > 0 && coord.1 > 0 && coord.2 > 0 
                && coord.0 < size && coord.1 < size && coord.2 < size {
                    possible_places[coord.0][coord.1][coord.2] = Some(States::Connection);
                    for occupied_coord in occupied_coords2 {
                        cube[occupied_coord.0][occupied_coord.1][occupied_coord.2] = Some(dummy_room);
                        possible_places[occupied_coord.0][occupied_coord.1][occupied_coord.2] = Some(States::Filled);
                    }
                }
            }
        }
    }



}

fn populate_rooms(n: usize) -> Vec<Room> {
    let mut possible_places: Map = Default::default();
    let mut cube: Cube = Default::default();
    let mut rooms = Vec::new();
    let mut rng = thread_rng();
    let mut rooms_to_go = n;

    let seed_room = Room {
        room_type: RoomType::Cube,
        dimensions: (1.0, 1.0, 1.0),
        position: Some((5, 5, 5)),
        color: [0.2, 1.0, 0.2, 1.0],
        rotation: Rotation::None,
    };

    let occupied_cells = get_occupied_cells(&seed_room);

    calculate_connections(&seed_room, &mut cube, &mut possible_places, occupied_cells);
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
        
        let spacing = 0.0;

        let dimensions: (f32, f32, f32) = match room_type {
            RoomType::Cube => (1.0, 1.0, 1.0),
            RoomType::R1 => (1.0, 1.0, 2.0+spacing),
            RoomType::R2 => (-2.0+spacing, 1.0, 1.0),
            RoomType::R3 => (1.0, 1.0, -2.0+spacing),
            RoomType::R4 => (2.0+spacing, 1.0, 1.0),
        };

        let color: [f32; 4] = match room_type {
            RoomType::Cube => [1.0, 0.3, 0.8, 1.0],
            RoomType::R1 => [0.0, 0.2, 1.0, 1.0],
            RoomType::R2 => [0.0, 0.2, 1.0, 1.0],
            RoomType::R3 => [0.0, 0.2, 1.0, 1.0],
            RoomType::R4 => [0.0, 0.2, 1.0, 1.0],
        };

        let new_room = Room {
            room_type,
            dimensions,
            position,
            color,
            rotation: Rotation::None,
        };

        let new_occupied_cells = get_occupied_cells(&new_room);
        let mut valid = true;
        for cell in &new_occupied_cells {
            if cube[cell.0][cell.1][cell.2].is_some() { valid = false }
            match possible_places[cell.0][cell.1][cell.2] {
                Some(state) => {
                    match state {
                        States::Connection => {},
                        States::Empty => {},
                        States::Filled => { valid = false },
                    }
                },
                None => { valid = false },
            }
        }

        if valid {
            calculate_connections(&new_room, &mut cube, &mut possible_places, new_occupied_cells);
            rooms.push(new_room);
            rooms_to_go -= 1;
        }
    }

    let possible_connections = get_possible_connections(&mut possible_places);
    let position = Some(possible_connections[rng.gen_range(0..possible_connections.len())]);

    let end_room = Room {
        room_type: RoomType::Cube,
        dimensions: (1.0, 1.0, 1.0),
        position,
        color: [1.0, 1.0, 0.1, 1.0],
        rotation: Rotation::None,
    };

    rooms.push(end_room);



    rooms
}



fn get_occupied_cells(room: &Room) -> Vec<(usize, usize, usize)> {
    // let (x, y, z) = room.position.unwrap();
    let p = room.position.unwrap();
    let x = p.0;
    let y = p.1;
    let z = p.2;
    let (mut dx, mut dy, mut dz) = room.dimensions;

    let mut cells = Vec::new();

    match room.room_type {
        RoomType::Cube => {
            cells.push((x, y, z));
        },
        RoomType::R1 => {
            cells.push((x, y, z));
            cells.push((x, y, z+1));
        },
        RoomType::R2 => {
            cells.push((x, y, z));
            cells.push((x-1, y, z));
        },
        RoomType::R3 => {
            cells.push((x, y, z));
            cells.push((x, y, z-1));
        },
        RoomType::R4 => {
            cells.push((x, y, z));
            cells.push((x+1, y, z));
        },
    }


    println!("number of cells: {}", cells.len());
    cells
}


