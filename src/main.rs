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
// Registered components must implement the `Reflect` and `FromWorld` traits.
// The `Reflect` trait enables serialization, deserialization, and dynamic property access.
// `Reflect` enable a bunch of cool behaviors, so its worth checking out the dedicated `reflect.rs`
// example. The `FromWorld` trait determines how your component is constructed when it loads.
// For simple use cases you can just implement the `Default` trait (which automatically implements
// `FromWorld`). The simplest registered component just needs these three derives:
#[derive(Component, Reflect, Default)]
#[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
struct AppId(Uuid);

// Resources can be serialized in scenes as well, with the same requirements `Component`s have.
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
        target_appId: AppId,
        // The dynamic scene to add/remove from scene
        scene: DynamicScene,
    },
}

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

    // let mut builder = DynamicSceneBuilder::from_world(&world);
    // builder.extract_entity(entity);
    // let dynamic_scene = builder.build();
}
//
// fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
//     // "Spawning" a scene bundle creates a new entity and spawns new instances
//     // of the given scene's entities as children of that entity.
//     commands.spawn(DynamicSceneBundle {
//         // Scenes are loaded just like any other asset.
//         scene: asset_server.load(SCENE_FILE_PATH),
//         ..default()
//     });
// }
//
// // This system logs all ComponentA components in our world. Try making a change to a ComponentA in
// // load_scene_example.scn. You should immediately see the changes appear in the console.
// fn log_system(
//     query: Query<(Entity, &Uuid), Changed<Uuid>>,
//     res: Option<Res<SceneSavedResource>>,
// ) {
//     for (entity, component_a) in &query {
//         info!("  Entity({})", entity.index());
//         info!(
//             "    ComponentA: {{ x: {} y: {} }}\n",
//             component_a.x, component_a.y
//         );
//     }
//     if let Some(res) = res {
//         if res.is_added() {
//             info!("  New ResourceA: {{ score: {} }}\n", res.score);
//         }
//     }
// }
//
// fn save_scene_system(world: &mut World) {
//     // Scenes can be created from any ECS World.
//     // You can either create a new one for the scene or use the current World.
//     // For demonstration purposes, we'll create a new one.
//     let mut scene_world = World::new();
//
//     // The `TypeRegistry` resource contains information about all registered types (including components).
//     // This is used to construct scenes, so we'll want to ensure that our previous type registrations
//     // exist in this new scene world as well.
//     // To do this, we can simply clone the `AppTypeRegistry` resource.
//     let type_registry = world.resource::<AppTypeRegistry>().clone();
//     scene_world.insert_resource(type_registry);
//
//     let mut component_b = ComponentB::from_world(world);
//     component_b.value = "hello".to_string();
//     scene_world.spawn((
//         component_b,
//         Uuid { x: 1.0, y: 2.0 },
//         Transform::IDENTITY,
//     ));
//     scene_world.spawn(Uuid { x: 3.0, y: 4.0 });
//     scene_world.insert_resource(SceneSavedResource { score: 1 });
//
//     // With our sample world ready to go, we can now create our scene:
//     let scene = DynamicScene::from_world(&scene_world);
//
//     // Scenes can be serialized like this:
//     let type_registry = world.resource::<AppTypeRegistry>();
//     let serialized_scene = scene.serialize_ron(type_registry).unwrap();
//
//     // Showing the scene in the console
//     info!("{}", serialized_scene);
//
//     // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
//     // as they are blocking
//     // This can't work in WASM as there is no filesystem access
//     #[cfg(not(target_arch = "wasm32"))]
//     IoTaskPool::get()
//         .spawn(async move {
//             // Write the scene RON data to file
//             File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
//                 .and_then(|mut file| file.write(serialized_scene.as_bytes()))
//                 .expect("Error while writing scene to file");
//         })
//         .detach();
// }
//
// // This is only necessary for the info message in the UI. See examples/ui/text.rs for a standalone
// // text example.
// fn infotext_system(mut commands: Commands) {
//     commands.spawn(Camera2dBundle::default());
//     commands.spawn(
//         TextBundle::from_section(
//             "Nothing to see in this window! Check the console output!",
//             TextStyle {
//                 font_size: 50.0,
//                 color: Color::WHITE,
//                 ..default()
//             },
//         )
//         .with_style(Style {
//             align_self: AlignSelf::FlexEnd,
//             ..default()
//         }),
//     );
// }
