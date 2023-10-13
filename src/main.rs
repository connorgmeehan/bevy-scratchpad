//! This example illustrates loading scenes from files.
use bevy::{ecs::system::SystemState, math::vec4, prelude::*, utils::Uuid, winit::WinitSettings};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .register_type::<Uuid>()
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(SceneStoreResource::default())
        .add_systems(Startup, (sys_setup_ui, sys_setup_scene))
        .run();
}

// Unique entity ID that persists serialisation/deserialisation. 
#[derive(Component, Reflect, Default)]
#[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
struct AppId(Uuid);

#[derive(Resource, Default)]
enum SceneStoreResource {
    #[default]
    Empty,
    ReadyToSave {
        target_appid: AppId,
    },
    Stored {
        /// Parent of the stored scene object
        parent_appid: AppId,
        // App Id of the stored scene object
        target_appid: AppId,
        // The dynamic scene to add/remove from scene
        scene: DynamicScene,
    },
}

const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.2, -0.2, 0.4, 0.0),
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.3, -0.3, 0.5, 0.0),
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.3, 0.2, -0.3, 0.0),
        ..matl.to_owned()
    })),
};

fn sys_setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Auto,
                padding: UiRect::all(Val::Px(50.)),
                justify_content: JustifyContent::End,
                align_items: AlignItems::Start,
                column_gap: Val::Px(10.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    background_color: Color::rgb_u8(50, 50, 50).into(),
                    style: Style {
                        padding: UiRect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(On::<Pointer<Click>>::run(sys_on_to_dynamic_scene))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "To Dynamic Scene",
                        TextStyle::default(),
                    ));
                });
            parent
                .spawn(ButtonBundle {
                    background_color: Color::rgb_u8(50, 50, 50).into(),
                    style: Style {
                        padding: UiRect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "From Dynamic Scene",
                        TextStyle::default(),
                    ));
                });
        });
}

fn sys_setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RaycastPickCamera::default(),
    ));
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(5.0).into()),
            material: materials.add(StandardMaterial::from(Color::rgb(0.3, 0.5, 0.3))),
            ..default()
        },
        AppId::default(),
        PickableBundle::default(),
        HIGHLIGHT_TINT,
    ));
    // cube
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(StandardMaterial::from(Color::rgb(0.8, 0.7, 0.6))),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
            AppId::default(),
            PickableBundle::default(),
            HIGHLIGHT_TINT,
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Torus {
                        radius: 0.1,
                        ring_radius: 0.5,
                        ..Default::default()
                    })),
                    material: materials.add(StandardMaterial::from(Color::rgb(0.8, 0.7, 0.6))),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..Default::default()
                },
                AppId::default(),
                PickableBundle::default(),
                HIGHLIGHT_TINT,
            ));
        });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn sys_on_to_dynamic_scene(world: &mut World) {
    let mut sys_state: SystemState<(ResMut<SceneStoreResource>, Query<(Entity, &AppId)>)> =
        SystemState::new(world);

    let (mut scene_sture, q_app_id) = sys_state.get_mut(world);

    println!("TODO: Move the picked object into a dynamic scene and store in resource.");
}
fn sys_on_from_dynamic_scene(world: &mut World) {
    let mut sys_state: SystemState<(ResMut<SceneStoreResource>, Query<(Entity, &AppId)>)> =
        SystemState::new(world);

    let (mut scene_sture, q_app_id) = sys_state.get_mut(world);

    println!("TODO: Insert the dynamic scene back into the world, as a child of the picked object (or root if no object selected).");
}
