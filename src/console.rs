use std::f32::consts::PI;
use std::collections::HashMap;
use bevy::{core_pipeline::core_3d::graph::input, prelude::*, render::render_asset::RenderAssetUsages, transform::commands};
use bevy_rapier3d::parry::simba::scalar::SupersetOf;
// use bevy_rapier3d::rapier::dynamics::RigidBody;
use bevy_ui::prelude::*;
use bevy::math::vec3;
use meshtext::{MeshGenerator, MeshText, TextSection};
use bevy_rapier3d::{parry::query::Ray, prelude::*};
use bevy::input::mouse::MouseWheel;
use bevy::input::mouse::MouseScrollUnit;
use crate::game;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;


#[derive(Component)]
pub struct ActiveTerminal {
    pub id: String
}

#[derive(Component)]
pub struct ConsoleTerminal;

#[derive(Resource)]
pub struct Terminal {
    pub text: Vec<String>,
    pub upper: usize,
    pub lower: usize,
}

#[derive(Resource)]
pub struct CurrentCommand {
    pub text: String
}

#[derive(Component)]
pub struct TerminalScreen;

#[derive(Component)]
pub struct ConsoleText;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum ConsoleState {
    #[default]
    IsNotUsingConsole,
    IsUsingConsole,
}



pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        // app.insert_resource(Terminal{ text: vec![String::from("hello world")] });
        // app.add_systems(Update, update_terminal);
    }
}

fn access_terminal(
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyF) { // and check if in front of console
        // move camera to be looking right at console screen
    }
}

pub fn update_terminal(
    terminal: ResMut<Terminal>,
    current_command: ResMut<CurrentCommand>,
    mut terminal_child_query: Query<(Entity, &mut Handle<Mesh>), With<ConsoleText>>,
    mut terminal_screen_query: Query<Entity, (With<TerminalScreen>, Without<ConsoleText>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let mut text_list: Vec<&str> = vec![];
    for line in terminal.text.iter() {
        text_list.push(&line);
    }

    if text_list.len() >= terminal.upper {
        let l = text_list.len();
        text_list = text_list[(l-terminal.upper)..((l-terminal.upper)+13)].to_vec();
        let l2 = text_list.len();
    } else {
        let leftover = 14 - text_list.len();

        for x in 0..leftover {
            text_list.push(".");
        }
    }


    let mut command_line_string = String::from("> ");
    command_line_string.push_str(&current_command.text);
    text_list.insert(13, &command_line_string);

    for (i, (mut entity, mut mesh) ) in terminal_child_query.iter_mut().enumerate() {
        *mesh = meshes.add(get_text_mesh(text_list[i]));
    }
}


pub fn spawn_console(
    transform: Transform,
    id: String,
    asset_server: & Res<AssetServer>,
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
) {

    let console_scene = asset_server.load("objects/console.gltf#Scene0");
    // let console_mesh: Handle<Mesh> = asset_server.load("objects/consolewithcollider.gltf#Mesh0/Primitive0");
    // let cmesh = meshes.get(console_mesh).expect("could not open mesh");

    let console = commands.spawn((
        SceneBundle {
            scene: console_scene,
            transform,
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            RigidBody::Fixed,
            Collider::cuboid(0.7, 0.9, 0.6),
            TransformBundle::from_transform(Transform {
                translation: vec3(-0.9, 1.0, -1.0),
                ..default()
            }),
            game::Interactable,
            game::InteractionType::Console
        ));
    })
    .insert(ConsoleTerminal)
    .id();
    // .insert(RigidBody::Dynamic)
    // .insert(Collider::from_bevy_mesh(cmesh, &ComputedColliderShape::ConvexHull))
    // .insert((ConsoleTerminal, game::Interactable, game::InteractionType::Console))
    // .insert(ActiveTerminal{id}).id();

    // let console_mesh: Handle<Mesh> = asset_server.load("objects/consolewithcollider.gltf#Mesh0/Primitive0");
    // let m = meshes.get(console_mesh).expect("could not unwrap console mesh");
    // println!("M UNWRAPPED");

    // let console_collider = commands.spawn
    // (RigidBody::Dynamic)
    // .insert(Collider::from_bevy_mesh(m, &ComputedColliderShape::TriMesh).unwrap())
    // .insert(TransformBundle::from_transform(transform))
    // .id();
    
    let mesh = get_text_mesh("");
    let scale = vec3(0.05, 0.05, 0.05);

    // create the screen element to attatch the text children to
    let screen = commands.spawn(PbrBundle {
        transform: Transform {
            translation: vec3(
                -0.5,
                0.15,
                -1.7,
            ),
            rotation: Quat::from_axis_angle(vec3(1.0, 0.0, 0.0), PI / 18.0),
            ..default()
        },
        ..Default::default()
    }).insert(TerminalScreen)
    .id();

    // create the default bundle for the text
    let bundle = PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.1, 1.0, 0.1)),
        transform: Transform {
            translation: vec3(0.0, 1.6, 0.0), 
            scale,
            rotation: Quat::from_axis_angle(vec3(0.0, 1.0, 0.0), PI),
            ..Default::default()
        },
        ..Default::default()
    };

    for i in 0..13 {
        let mut b = bundle.clone();
        let new_bundle = get_text_pos(&mut b, i as usize);
        let line = commands.spawn(new_bundle).insert(ConsoleText).id();
        commands.entity(screen).add_child(line);
    }

    let command_bundle = get_text_pos(&mut bundle.clone(), 13);
    let command_line = commands.spawn(command_bundle).insert(ConsoleText).id();
    commands.entity(screen).add_child(command_line);

    commands.entity(console).add_child(screen);

}

fn get_text_mesh(
    text: &str
) -> Mesh {
    let font_data = include_bytes!("../assets/fonts/KodeMono-Regular.ttf");
    let mut generator = MeshGenerator::new(font_data);
    let transform = Mat4::from_scale(Vec3::new(1f32, 1f32, 0.2f32)).to_cols_array();
    let text_mesh: MeshText = generator
        .generate_section(&text.to_string(), false, Some(&transform))
        .unwrap();

    let vertices = text_mesh.vertices;
    let positions: Vec<[f32; 3]> = vertices.chunks(3).map(|c| [c[0], c[1], c[2]]).collect();
    let uvs = vec![[0f32, 0f32]; positions.len()];

    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.compute_flat_normals();

    return mesh;
}

fn get_text_pos(bundle: &mut PbrBundle, index: usize) -> PbrBundle {
    let y = 1.6 - (0.05*index as f32);
    // println!("{}", y);
    let mut new_bundle = bundle.clone();
    new_bundle.transform.translation = vec3(0.0, y, 0.0);

    return new_bundle;
}






//            TERMINAL STRUCTURE AND NAVIGATION


#[derive(Resource)]
pub struct GameDirectory {
    pub root: Directory
}

#[derive(Resource)]
pub struct CurrentDirectory(pub Directory);

#[derive(Clone)]
pub enum Node {
    Directory(Directory),
    Program(Program),
    File(File)
}

#[derive(Clone)]
pub struct Program {
    name: String
}

#[derive(Clone)]
pub struct File {
    name: String
}

// A node representing a directory
#[derive(Clone)]
pub struct Directory {
    pub name: String,
    pub children: HashMap<String, Node>, // Map of child directory names to Directory structs
}

impl Directory {
    pub fn new(name: String) -> Self {
        Directory {
            name,
            children: HashMap::new(),
        }
    }

    pub fn add_dir(&mut self, name: String) -> Result<(), String> {
        let n = name.clone();
        if self.children.contains_key(&name) {
            Err(format!("Directory with name '{}' already exists", &name))
        } else {
            let mut new_dir = Directory::new(n);
            self.children.insert(name, Node::Directory(new_dir));
            Ok(())
        }
    }

    pub fn add_program(&mut self, name: String) -> Result<(), String> {
        let n = name.clone();
        if self.children.contains_key(&name) {
            Err(format!("name '{}' already exists", &name))
        } else {
            let mut new_program = Program { name: name.clone() };
            self.children.insert(name, Node::Program(new_program));
            Ok(())
        }
    }

    pub fn add_file(&mut self, name: String) -> Result<(), String> {
        let n = name.clone();
        if self.children.contains_key(&name) {
            Err(format!("name '{}' already exists", &name))
        } else {
            let mut new_program = File { name: name.clone() };
            self.children.insert(name, Node::File(new_program));
            Ok(())
        }
    }

    pub fn get_child(&mut self, name: &str) -> Option<&mut Node> {
        let child = self.children.get_mut(name);
        return child;
    }

    pub fn get_dir(&mut self, name: &str) -> Option<&mut Directory> {
        let child = self.children.get_mut(name);
        match child {
            Some(node) => {
                match node {
                    Node::Directory(dir) => {
                        return Some(dir);
                    }
                    _ => { None }
                }
            },
            None => { None },
        }
        // return child;
    }

    pub fn find_child_dir(&mut self, name: &str) -> Option<Directory> {
        if self.children.clone().contains_key(name) {
            match self.clone().get_child(name) {
                Some(node) => {
                    match node {
                        Node::Directory(dir) => {
                            return Some(dir.clone());
                        }
                        _ => {}
                    }
                },
                None => {},
            }
            
        }

        for (_, child) in &mut self.children {
            match child {
                Node::Directory(dir) => {
                    return dir.find_child_dir(name);
                }
                _ => {

                }
            }
        }

        None
    }

    pub fn delete_child(&mut self, name: &str) -> Result<(), String> {
        if let Some(_child) = self.children.remove(name) {
            Ok(())
        } else {
            Err(format!("Directory with name '{}' not found", name))
        }
    }

    pub fn cd(&mut self, path: &str) -> Result<&Directory, String> {
        let mut current_dir = self;
        let mut dir_name = format!("{}{}/", current_dir.name, path);
        if dir_name.ends_with("//") { dir_name.pop(); }
        println!("{dir_name}");
        let cd = current_dir.get_child(&dir_name).ok_or_else(|| "could not find dir")?;

        match cd {
            Node::Directory(dir) => {
                Ok(dir)
            }
            _ => {
                Err(String::from("! not a valid directory"))
            }
        }

        
    }

    pub fn ls(&self) -> (Vec<String>, Vec<String>) {
        let mut names = vec![];
        let mut lengths = vec![];
        if self.children.is_empty() {
            names.push(String::from("~ empty"))
        } else {
            for child in self.children.clone() {
                match child.1 {
                    Node::Directory(dir) => {
                        let mut n = child.0.clone();
                        _=n.pop();
                        let v = n.split("/").last().unwrap();
        
                        names.push(String::from(v));
                        lengths.push(dir.children.len().to_string());
                    }
                    Node::Program(program) => {
                        let mut n = program.name.clone();
                        // _=n.pop();
                        let v = n.split("/").last().unwrap();
                        names.push(String::from(v));
                        lengths.push(String::from("!"));
                    }
                    Node::File(file) => {
                        let mut n = file.name.clone();
                        // _=n.pop();
                        let v = n.split("/").last().unwrap();
                        names.push(String::from(v));
                        lengths.push(String::from("#"));
                    }
                    _ => {}
                }

            }
        }
        return (names, lengths);
    }
}

fn start_matchbox_socket() -> MatchboxSocket<SingleChannel> {
    let room_url = "ws://stevelovesgames.com/oxyb?next=2";
    info!("connecting to matchbox server: {room_url}");
    MatchboxSocket::new_unreliable(room_url)
}

fn terminal_list(dir: Directory) -> (Vec<String>, Vec<String>) {
    return dir.ls();
} 

fn make_hello() -> String {
    return String::from("hello, my name is sumi :3");
}

// very happy with this
// need to add the root to the world resources
// need to format the outputs
// need to make it happen!




//                   HANDLE TERMINAL INPUTS AND UPDATE SCREEN


pub fn use_console(
    mut terminal: ResMut<Terminal>,
    mut current_command: ResMut<CurrentCommand>,
    mut root: ResMut<GameDirectory>,
    mut current_directory: ResMut<CurrentDirectory>,
    mut console_state: Res<State<ConsoleState>>,
    input: Res<ButtonInput<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut next_console_state: ResMut<NextState<ConsoleState>>,
    mut commands: Commands,
) {
    match console_state.get() {
        ConsoleState::IsUsingConsole => {

            // upper cannot be lower that 13
            // nor can it be above length of list
            // list.len() = 15? 15-13
            for event in scroll_evr.read() {
                if event.y > 0.0 {
                    println!("UP");
                    let size = terminal.text.len();
                    terminal.upper += 1;
                    if terminal.upper >= size { terminal.upper = size }
                } else {
                    println!("DOWN");
                    terminal.upper -= 1;
                    if terminal.upper < 13 { terminal.upper = 13 }
                }
            }



            for key in input.get_just_pressed() {
                println!("{}", current_command.text);
                if input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
                    // shift is held
                    for key in input.get_just_pressed() {
                        match key {
                            KeyCode::Digit0 => { current_command.text.push_str("!") }
                            KeyCode::Digit1 => { current_command.text.push_str("@") }
                            KeyCode::Digit2 => { current_command.text.push_str("#") }
                            KeyCode::Digit3 => { current_command.text.push_str("$") }
                            KeyCode::Digit4 => { current_command.text.push_str("%") }
                            KeyCode::Digit5 => { current_command.text.push_str("^") }
                            KeyCode::Digit6 => { current_command.text.push_str("&") }
                            KeyCode::Digit7 => { current_command.text.push_str("*") }
                            KeyCode::Digit8 => { current_command.text.push_str("(") }
                            KeyCode::Digit9 => { current_command.text.push_str(")") }
                            KeyCode::Slash => { current_command.text.push_str("?") }
                            KeyCode::Comma => { current_command.text.push_str("<") }
                            KeyCode::Period => { current_command.text.push_str(">") }
                            KeyCode::Semicolon => { current_command.text.push_str(":") }
                            KeyCode::Quote => { current_command.text.push_str("\"") }
                            KeyCode::BracketLeft => { current_command.text.push_str("{") }
                            KeyCode::BracketRight => { current_command.text.push_str("}") }
                            KeyCode::Backslash => { current_command.text.push_str("|") }
                            KeyCode::Minus => { current_command.text.push_str("_") }
                            KeyCode::Equal => { current_command.text.push_str("+") }
                            _ => {}
                        }
                    }
                } else {
                    for key in input.get_just_pressed() {

                        match key {
                            KeyCode::KeyA => { current_command.text.push_str("a") }
                            KeyCode::KeyB => { current_command.text.push_str("b") }
                            KeyCode::KeyC => { current_command.text.push_str("c") }
                            KeyCode::KeyD => { current_command.text.push_str("d") }
                            KeyCode::KeyE => { current_command.text.push_str("e") }
                            KeyCode::KeyF => { current_command.text.push_str("f") }
                            KeyCode::KeyG => { current_command.text.push_str("g") }
                            KeyCode::KeyH => { current_command.text.push_str("h") }
                            KeyCode::KeyI => { current_command.text.push_str("i") }
                            KeyCode::KeyJ => { current_command.text.push_str("j") }
                            KeyCode::KeyK => { current_command.text.push_str("k") }
                            KeyCode::KeyL => { current_command.text.push_str("l") }
                            KeyCode::KeyM => { current_command.text.push_str("m") }
                            KeyCode::KeyN => { current_command.text.push_str("n") }
                            KeyCode::KeyO => { current_command.text.push_str("o") }
                            KeyCode::KeyP => { current_command.text.push_str("p") }
                            KeyCode::KeyQ => { current_command.text.push_str("q") }
                            KeyCode::KeyR => { current_command.text.push_str("r") }
                            KeyCode::KeyS => { current_command.text.push_str("s") }
                            KeyCode::KeyT => { current_command.text.push_str("t") }
                            KeyCode::KeyU => { current_command.text.push_str("u") }
                            KeyCode::KeyV => { current_command.text.push_str("v") }
                            KeyCode::KeyW => { current_command.text.push_str("w") }
                            KeyCode::KeyX => { current_command.text.push_str("x") }
                            KeyCode::KeyY => { current_command.text.push_str("y") }
                            KeyCode::KeyZ => { current_command.text.push_str("z") }
                            KeyCode::Digit0 => { current_command.text.push_str("0") }
                            KeyCode::Digit1 => { current_command.text.push_str("1") }
                            KeyCode::Digit2 => { current_command.text.push_str("2") }
                            KeyCode::Digit3 => { current_command.text.push_str("3") }
                            KeyCode::Digit4 => { current_command.text.push_str("4") }
                            KeyCode::Digit5 => { current_command.text.push_str("5") }
                            KeyCode::Digit6 => { current_command.text.push_str("6") }
                            KeyCode::Digit7 => { current_command.text.push_str("7") }
                            KeyCode::Digit8 => { current_command.text.push_str("8") }
                            KeyCode::Digit9 => { current_command.text.push_str("9") }
                            KeyCode::Slash => { current_command.text.push_str("/") }
                            KeyCode::Comma => { current_command.text.push_str(",") }
                            KeyCode::Period => { current_command.text.push_str(".") }
                            KeyCode::Semicolon => { current_command.text.push_str(";") }
                            KeyCode::Quote => { current_command.text.push_str("'") }
                            KeyCode::BracketLeft => { current_command.text.push_str("[") }
                            KeyCode::BracketRight => { current_command.text.push_str("]") }
                            KeyCode::Backslash => { current_command.text.push_str("\\") }
                            KeyCode::Minus => { current_command.text.push_str("-") }
                            KeyCode::Equal => { current_command.text.push_str("=") }
                            KeyCode::Space => { current_command.text.push_str(" ") }
                            KeyCode::Backspace => { current_command.text.pop(); }
                            KeyCode::Enter => { 
            
                                let command = current_command.text.clone();
                                terminal.text.push("$ ".to_owned() + &current_command.text.clone());
                                current_command.text = String::from("");
        
                                if command.to_lowercase().eq("clear") { // >> CLEAR
                                    println!("CLEAR COMMAND");
                                    terminal.text = Vec::new();
                                }
                                else if command.to_lowercase().eq("exit") { // >> EXIT
                                    next_console_state.set(ConsoleState::IsNotUsingConsole);
                                }
                                if command.to_lowercase().eq("hello") { // >> HELLO
                                    println!("HELLO COMMAND");
                                    terminal.text.push(make_hello());
                                }
                                if command.to_lowercase().eq("ls") { // >> LIST
                                    println!("list directory COMMAND");
                                    let cd = current_directory.0.clone();
                                    terminal.text.push(cd.name.clone());
                                    let list_values = terminal_list(cd);
                                    let list = list_values.0;
                                    if list.len() > 0 && list_values.1.len() > 0 {
                                        for (i, line) in list.iter().enumerate() {
                                            let length = format!("[{}]", list_values.1[i]);
                                            terminal.text.push(length + &line);
                                        }
                                    } else {
                                        terminal.text.push(String::from("~ empty"));
                                    }

                                }
                                if command.to_lowercase().starts_with("cd ") { // >> CURRENT DIRECTORY
                                    println!("move COMMAND");

                                    let new_dir = command.split(" ").last().unwrap();
                                    println!("{new_dir}");
                                    if new_dir == ".." {
                                        let cd = current_directory.0.clone();
                                        println!("{}", cd.name);
                                        let mut parent: Vec<&str> = cd.name.split("/").collect();
                                        if parent.len() <= 2 {
                                            println!("! current directory is root");
                                        } else {
                                            _=parent.pop();
                                            _=parent.pop();
                                            let mut new_dir = String::from("");
                                            for item in parent {
                                                new_dir = format!("{}{}/", new_dir, item);
                                            }
                                            if new_dir.eq("root/") {
                                                current_directory.0 = root.root.clone();
                                                terminal.text.push(String::from("root/"));
                                            } else {
                                                println!("moveing to: {}", new_dir);
                                                match root.root.find_child_dir(&new_dir) {
                                                    Some(dir) => {
                                                        let name = dir.name.clone();
                                                        current_directory.0 = dir.to_owned();
                                                        terminal.text.push(name);
                                                    },
                                                    None => {
                                                        terminal.text.push(String::from("! error moving to new directory"))
                                                    },
                                                }
                                            }
                                        }

                                    }
                                    else {
                                        match current_directory.0.cd(new_dir) {
                                            Ok(cd) => {
                                                let name = cd.name.clone();
                                                current_directory.0 = cd.to_owned();
                                                terminal.text.push(name);
                                            },
                                            Err(_) => {
                                                terminal.text.push(String::from("! could not find directory"))
                                            },
                                        };
                                    }
                                }

                                if command.to_lowercase().eq("start-server") {
                                    let socket = start_matchbox_socket();
                                    commands.insert_resource(socket)
                                }

                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

