use std::collections::HashMap;
use rand::*;
use rand::seq::SliceRandom;

pub struct Room {
    pub template: RoomTemplate,
    pub exits: Vec<(Direction, bool)>
}

pub enum Direction {
    N,
    S,
    E,
    W
}

pub enum RoomTemplate {
    Start,
    End,
    Normal,
}

pub struct DungeonGraph {
    pub nodes: Vec<Room>,
    pub edges: Vec<(usize, usize)>,
}

impl DungeonGraph {
    pub fn print_graph(&self) {
        println!();
        println!("-- -- -- DUNGEON GRAPH LAYOUT -- -- --");
        for (i, room) in self.nodes.iter().enumerate() {
            let room_type = match room.template {
                RoomTemplate::Start => "[S]",
                RoomTemplate::End => "[E]",
                RoomTemplate::Normal => "[0]"
            };
            println!("room {}: {}", i, room_type);
        }

        println!();
        println!("Connections:");
        for(from, to) in &self.edges {
            println!("{} -> {}", from, to);
        }
    }
}

pub fn generate_simple_dungeon() -> DungeonGraph {
    let mut graph = DungeonGraph { nodes: Vec::new(), edges: Vec::new() };
    let mut rng = rand::thread_rng();

    graph.nodes.push(Room { template: RoomTemplate::Start, exits: vec![(Direction::N, true)] });
    graph.nodes.push(Room { template: RoomTemplate::End, exits: vec![(Direction::S, true)] });

    let number_of_normal_rooms = 5;
    for _ in 0..number_of_normal_rooms {
        graph.nodes.push(Room { template: RoomTemplate::Normal, exits: vec![(Direction::N, true), (Direction::S, true)] })
    }

    graph.nodes.shuffle(&mut rng);

    for i in 0..graph.nodes.len() - 1 {
        match graph.nodes[i].template {
            RoomTemplate::End => {
                if i == 1 {
                    graph.nodes.swap(i, i+1);
                }
            }
            _=>{}            
        }
        graph.edges.push((i, i+1));
    }

    graph
}