mod bindings {
    wit_bindgen::generate!({
        path: ["./wit"],
        world: "component:cubes/cubes",
        with: {
            "wasvy:ecs/app": generate,
        }
    });
}
use bevy_math::Quat;
use bevy_transform::components::Transform;
use bindings::{
    wasvy::ecs::app::{App, Query, QueryFor, Schedule, System},
    *,
};
use serde::{Deserialize, Serialize};
use shared::{Cube, Settings};

use crate::bindings::wasvy::ecs::app::TypePath;

struct CubesMod;

impl Guest for CubesMod {
    fn setup() {
        let spawn_cubes = System::new("spawn-cubes");
        spawn_cubes.add_commands();

        let spin_cubes = System::new("spin-cubes");
        spin_cubes.add_query(&[
            QueryFor::Mut(type_name::<Transform>()),
            QueryFor::With(type_name::<Cube>()),
        ]);
        spin_cubes.add_query(&[QueryFor::Ref(type_name::<Settings>())]);

        let app = App::new();
        app.add_systems(&Schedule::ModStartup, vec![spawn_cubes]);
        app.add_systems(&Schedule::Update, vec![spin_cubes]);
    }

    fn spawn_cubes(commands: Commands) {
        println!("Spawning a cube");

        commands.spawn(&[
            component(&Cube::default()),
            component(&Transform::default()),
        ]);
    }

    fn spin_cubes(cubes: Query, settings: Query) {
        let settings = settings.iter().expect("Single component")[0].get();
        let settings: Settings = from_json(&settings);

        while let Some(components) = cubes.iter() {
            // Get and deserialize the first component
            let mut transform: Transform = from_json(&components[0].get());

            // Spin the cube
            transform.rotate(Quat::from_rotation_y(settings.delta * settings.spin_speed));

            // Set the new component value
            components[0].set(&to_json(&transform));
        }
    }
}

export!(CubesMod);

fn type_name<T>() -> TypePath {
    String::from(std::any::type_name::<T>())
}

fn component<T>(value: &T) -> (TypePath, String)
where
    T: Serialize,
{
    (type_name::<T>(), to_json(value))
}

fn from_json<'a, T>(component: &'a str) -> T
where
    T: Deserialize<'a>,
{
    serde_json::from_str(component).expect("serializable component")
}

fn to_json<T>(component: &T) -> String
where
    T: Serialize,
{
    serde_json::to_string(&component).expect("serializable component")
}
