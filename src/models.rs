use bevy::prelude::*;



pub fn load_model(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the model file
    let model_handle: Handle<Scene> = asset_server.load("path/to/your/model.gltf#Scene0");

    // Spawn the model as an entity 
    commands.spawn(SceneBundle {
        scene: model_handle,
        ..default()
    });
}