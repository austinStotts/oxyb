// ...data structures from above

fn generate_rooms(weights: &HashMap<RoomType, usize>, num_rooms: usize) -> Vec<Room> {
    // ... logic to generate rooms based on weights
}

fn place_room(room: &Room, cube: &mut Cube) -> bool {
    // ... logic to check valid orientations and place the room in the cube
}

fn main() {
    let mut cube: Cube = Default::default();
    let room_weights = HashMap::from([(RoomType::Cube, 3), (RoomType::RectPrism, 2)]);
    let rooms = generate_rooms(&room_weights, 20); // Example

    for room in rooms {
        if !place_room(&room, &mut cube) {
            println!("Unable to place all rooms");
            break;
        }
    }
}






use rand::prelude::*; // Bring in a random number generator

// ... other code remains the same

fn generate_rooms(weights: &HashMap<RoomType, usize>, num_rooms: usize) -> Vec<Room> {
    let mut rooms = Vec::new();
    let mut rng = thread_rng(); // Create a thread-local random number generator

    let total_weight: usize = weights.values().sum(); // Sum of all weights

    for _ in 0..num_rooms {
        let rand_val = rng.gen_range(0..total_weight); // Value for weighted selection
        let mut cumulative_weight = 0;

        // Find matching type based on weight
        for (room_type, weight) in weights {
            cumulative_weight += weight;
            if rand_val < cumulative_weight {
                let dimensions = match room_type {
                    RoomType::Cube => (1, 1, 1),
                    RoomType::RectPrism => (1, 1, 2),
                };
                rooms.push(Room {
                    room_type: room_type.clone(),
                    dimensions,
                    position: None,
                });
                break;
            }
        }
    }

    rooms
}



fn place_room(room: &Room, cube: &mut Cube) -> bool {
    let (dim_x, dim_y, dim_z) = room.dimensions;

    for x in 0..cube.len() - dim_x + 1 {
        for y in 0..cube[0].len() - dim_y + 1 {
            for z in 0..cube[0][0].len() - dim_z + 1 {
                if check_valid_placement(&room, cube, x, y, z) {
                    // Place the room
                    for i in x..x + dim_x {
                        for j in y..y + dim_y {
                            for k in z..z + dim_z {
                                cube[i][j][k] = Some(room.clone()); 
                            }
                        }
                    }
                    return true; 
                }
            }
        }
    }

    false // No valid placement found
}

fn check_valid_placement(room: &Room, cube: &Cube, x: usize, y: usize, z: usize) -> bool {
    let (dim_x, dim_y, dim_z) = room.dimensions;

    // Check bounds and if cells are occupied
    for i in x..x + dim_x {
        for j in y..y + dim_y {
            for k in z..z + dim_z {
                if i >= cube.len() || j >= cube[0].len() || k >= cube[0][0].len() || cube[i][j][k].is_some() {
                    return false; 
                }
            }
        }
    }

    true 
}











use bevy::prelude::*;
use bevy_ui::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Setup your 3D camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Setup your UI camera
    commands.spawn(Camera2dBundle::default());

    // Create UI text within 3D Space 
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(10.0),
                bottom: Val::Px(10.0),
                ..default() 
            },
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section("Hello from 3D!", TextStyle {
                font: asset_server.load("fonts/my_font.ttf"),
                font_size: 40.0,
                color: Color::ORANGE,
            }),
            ..default()
        });
    });
}









