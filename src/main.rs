use bevy::prelude::*;
use bevy::{DefaultPlugins, app::App};
use bevy_mod_imgui::prelude::*;
use shared_reflect::{Cube, Settings};
use wasvy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ImguiPlugin::default(),
            ModloaderPlugin::default(),
        ))
        .add_systems(Startup, (load_mods, setup))
        .add_systems(PreUpdate, update_delta)
        .add_systems(Update, (update_ui, setup_cubes))
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
    // Settings is an entity so the mod can access it
    commands.spawn(Settings {
        delta: 0.0,
        count: 1,
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

// Updates the delta time read by mods in Update
fn update_delta(mut settings: Single<&mut Settings>, time: Res<Time>) {
    // Currently wasvy does not support reading from resources so we just pass it viua settings as a temporary solution
    settings.delta = time.delta_secs();
}

fn update_ui(
    mut context: NonSendMut<ImguiContext>,
    mut settings: Single<&mut Settings>,
    mut commands: Commands,
) {
    let ui = context.ui();
    ui.window("Settings")
        .size([300.0, 400.0], Condition::FirstUseEver)
        .build(|| {
            ui.text("Count");
            ui.slider_config("##count", 1, 5).build(&mut settings.count);

            ui.text("Spin speed");
            ui.slider_config("##spin_speed", 0.0, 5.0)
                .build(&mut settings.spin_speed);

            if ui.button("Reload") {
                commands.queue(reload);
            }
        });
}

fn reload(world: &mut World) {
    info!("Reloading!");

    // Remove cubes spawned before
    let cubes: Vec<_> = world
        .query_filtered::<Entity, With<Cube>>()
        .iter(&world)
        .collect();
    for entity in cubes {
        world.despawn(entity);
    }

    // Reload mods
    let mods: Vec<_> = world
        .query::<(Entity, &Mod, Option<&Name>)>()
        .iter(&world)
        .map(|(entity, a_mod, name)| {
            let mut new_mod = Mod::new(a_mod.asset());
            for access in a_mod.accesses() {
                new_mod.enable_access(*access);
            }
            (entity, new_mod, name.cloned())
        })
        .collect();
    for (entity, new_mod, name) in mods {
        // Despawn existing mod
        world.despawn(entity);

        // Spawn a new replacement
        let mut entity = world.spawn(new_mod);
        if let Some(name) = name {
            entity.insert(name);
        }
    }
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
