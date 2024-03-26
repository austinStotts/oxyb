
use std::path::Component;

use bevy::{math::vec3, prelude::*};
use bevy_flycam::prelude::*;
use bevy::{
    core_pipeline::{
        core_3d::graph::{Core3d, Node3d},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::{
            ComponentUniforms, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
        },
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            *,
        },
        
        camera::RenderTarget,
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::ViewTarget,
        RenderApp,
    },
    window::WindowRef,
};


use iyes_perf_ui::prelude::*;

mod map;






#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
struct PostProcessSettings {
    intensity: f32,
    sigma1: f32,
    tau: f32,
    gfact: f32,
    epsilon: f32,
    num_gvf_iterations: i32,
    enable_xdog: u32,
}










fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PostProcessPlugin, NoCameraPlayerPlugin))
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00002, // default: 0.00012
            speed: 6.0, // default: 12.0
        })
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::ControlLeft,
            ..Default::default()
        })
        .insert_resource(ActiveCamera::Primary)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, update_settings, keyboard_input, switch_cameras))
        .run();
}


#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct SecondCamera;

#[derive(Resource)]
enum ActiveCamera {
    Primary,
    Secondary
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let matrix = map::create_matrix(5);
    spawn_cubes_from_matrix(&mut commands, &mut meshes, &mut materials, &matrix);

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1_000.,
            ..default()
        },
        ..default()
    });
    
    // main camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 5.0))
                .looking_at(Vec3::default(), Vec3::Y),
            camera: Camera {
                clear_color: Color::WHITE.into(),
                // target: RenderTarget::Window(WindowRef::Primary),
                order: 1,
                is_active: true,
                ..default()
            },
            ..default()
        },
        PostProcessSettings {
            intensity: 0.02,
            sigma1: 8.0,
            tau: 0.01,
            gfact: 8.0,
            epsilon: 0.0001,
            num_gvf_iterations: 15,
            enable_xdog: 1,
        },
        FlyCam,
    )).insert(MainCamera);

    // frame times
    commands.spawn(PerfUiCompleteBundle::default());

    // second camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 5.0))
                .looking_at(Vec3::default(), Vec3::Y),
            camera: Camera {
                clear_color: Color::WHITE.into(),
                order: 0,
                is_active: false,
                ..default()

            },
            ..default()
        },
    )).insert(SecondCamera);



}





fn spawn_cubes_from_matrix(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    matrix: &Vec<Vec<Vec<map::CellType>>>,
) {
    let cube_size = 1.0; 
    let spacing = 1.2; 

    // Find the coordinates of the start cell
    let mut start_x = 0; 
    let mut start_y = 0; 
    let mut start_z = 0;
    for (z, layer) in matrix.iter().enumerate() { 
        for (y, row) in layer.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == map::CellType::Start {
                    start_x = x;
                    start_y = y; 
                    start_z = z; 
                    break; // Found the start, stop searching
                }
            }
        }
    }

    for (z, layer) in matrix.iter().enumerate() {
        for (y, row) in layer.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell != map::CellType::Empty {  // Customize the condition
                    let color = match cell {
                        map::CellType::Empty => Color::rgb(0.8, 0.8, 0.8), // Adjust for background color
                        map::CellType::Room => Color::rgb(0.2, 1.0, 0.2), // Green for rooms
                        map::CellType::Start => Color::rgb(0.0, 0.0, 1.0), // Blue for start
                        map::CellType::DeadEnd => Color::rgb(1.0, 1.0, 0.0), // Yellow for dead ends
                    };    
                    let x_pos = (x as f32 * spacing) - (start_x as f32 * spacing);
                    let y_pos = (y as f32 * spacing) - (start_y as f32 * spacing);
                    let z_pos = (z as f32 * spacing) - (start_z as f32 * spacing);

                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Cuboid::from_size(vec3(cube_size, cube_size, cube_size))), // Use Cuboid if needed
                            material: materials.add(color), // Adjust color as needed
                            transform: Transform::from_xyz(x_pos, y_pos, z_pos),
                            ..default()
                        },
                        // You might want more components like Rotates 
                        // Rotates
                    )); 
                }
            }
        }
    }
}



fn switch_cameras(
    mut active_camera: ResMut<ActiveCamera>,
    mut main_camera: Query<&mut Camera, With<MainCamera>>, 
    mut secondary_camera: Query<&mut Camera, Without<MainCamera>>, 
) {

    // set main camera
    for mut camera in main_camera.iter_mut() {
        match *active_camera {
            ActiveCamera::Primary => {
                camera.is_active = true;
            },
            ActiveCamera::Secondary => {
                camera.is_active = false;
            },
        }
    }

    // set other cameras
    for mut camera in secondary_camera.iter_mut() {
        match *active_camera {
            ActiveCamera::Primary => {
                camera.is_active = false;
            },
            ActiveCamera::Secondary => {
                camera.is_active = true;
            },
        }
    }

}




#[derive(Component)]
struct Rotates; // ROTATES

fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        transform.rotate_x(0.55 * time.delta_seconds());
        transform.rotate_z(0.15 * time.delta_seconds());
    }
}


#[derive(Component)]
struct KeyboardInput;

fn keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    mut active_camera: ResMut<ActiveCamera>,
) {
    if input.pressed(KeyCode::KeyW) {
        info!("'W' currently pressed");
    }
    if input.just_pressed(KeyCode::KeyA) {
        info!("'A' just pressed");
    }
    if input.just_released(KeyCode::KeyS) {
        info!("'S' just released");
    }
    if input.just_released(KeyCode::KeyD) {
        info!("'D' just released");
    }
    if input.just_released(KeyCode::Space) {
        info!("'Space' just released");
    }
    if input.just_released(KeyCode::ControlLeft) {
        info!("'Left CTRL' just released");
    }
    if input.just_released(KeyCode::ShiftLeft) {
        info!("'Left SHIFT' just released");
    }
    if input.just_released(KeyCode::KeyF) {
        info!("'F' just released");
    }
    if input.just_released(KeyCode::KeyE) {
        info!("'E' just released");
    }
    if input.just_released(KeyCode::KeyQ) {
        info!("'Q' just released");
    }
    if input.just_pressed(KeyCode::Tab) {
        *active_camera = ActiveCamera::Secondary;
    }
    if input.just_released(KeyCode::Tab) {
        *active_camera = ActiveCamera::Primary;
    }
}


fn update_settings(mut settings: Query<&mut PostProcessSettings>, time: Res<Time>) {
    for mut setting in &mut settings {
        let mut intensity = time.elapsed_seconds().sin();
        intensity = intensity.sin();
        intensity = intensity * 0.5 + 0.5;
        intensity *= 0.005;

        setting.intensity = intensity;
    }
}





struct PostProcessPlugin; // POST-PROCESS PLUGIN

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<PostProcessSettings>::default(),
            UniformComponentPlugin::<PostProcessSettings>::default(),
        ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<PostProcessNode>>(
                Core3d,
                PostProcessLabel,
            )
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    PostProcessLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<PostProcessPipeline>();
    }
}



#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct PostProcessLabel; // POST-PROCESS LABEL



#[derive(Resource)]
struct PostProcessPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for PostProcessPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let layout = render_device.create_bind_group_layout(
            "post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<PostProcessSettings>(false),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let shader = world
            .resource::<AssetServer>()
            .load("shaders\\post_processing.wgsl");

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("post_process_pipeline".into()),
                layout: vec![layout.clone()],
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}



#[derive(Default)]
struct PostProcessNode;


impl ViewNode for PostProcessNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static PostProcessSettings,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, _post_process_settings): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let post_process_pipeline = world.resource::<PostProcessPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let settings_uniforms = world.resource::<ComponentUniforms<PostProcessSettings>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        // This will start a new "post process write", obtaining two texture
        // views from the view target - a `source` and a `destination`.
        // `source` is the "current" main texture and you _must_ write into
        // `destination` because calling `post_process_write()` on the
        // [`ViewTarget`] will internally flip the [`ViewTarget`]'s main
        // texture to the `destination` texture. Failing to do so will cause
        // the current main texture information to be lost.
        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            "post_process_bind_group",
            &post_process_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &post_process_pipeline.sampler,
                settings_binding.clone(),
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}







