use bevy::prelude::*;
use bevy::{DefaultPlugins, app::App};
use shared_reflect::{Cube, Settings};
use wasvy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ModloaderPlugin::default()))
        .add_systems(Startup, (load_mods, setup))
        .add_systems(PreUpdate, pre_update)
        .add_systems(Update, setup_cubes)
        .register_type::<Settings>()
        .register_type::<Cube>()
        .init_resource::<Handles>()
        .run();
}

fn load_mods(mut mods: Mods) {
    mods.load("mods/cubes.wasm");
}

#[derive(Resource)]
struct Handles {
    cube: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl FromWorld for Handles {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let cube = meshes.add(Cuboid::default());

        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let material = materials.add(Color::srgb_u8(144, 255, 124));

        Self { cube, material }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Settings {
        delta: 0.0,
        spin_speed: 1.0,
    });

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 3.5, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn pre_update(mut settings: Single<&mut Settings>, time: Res<Time>) {
    settings.delta = time.delta_secs();
}

/// Add a mesh and a material to new cubes
fn setup_cubes(cubes: Query<Entity, Added<Cube>>, mut commands: Commands, handles: Res<Handles>) {
    for cube in cubes {
        commands.entity(cube).insert((
            Mesh3d(handles.cube.clone()),
            MeshMaterial3d(handles.material.clone()),
        ));
    }
}
