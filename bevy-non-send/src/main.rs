use bevy::prelude::*;
use std::cell::Cell;

#[derive(Default)]
struct UnSendable {
    cell: Cell<bool>,
}

fn main() {
    if true {
        App::new()
            .add_systems(Startup, setup_nonsend)
            .add_systems(Update, hello_world_system)
            .run();
    } else {
        App::new()
            .insert_non_send_resource(UnSendable::default())
            .add_systems(FixedUpdate, hello_world_system)
            .run();
    }
}

fn setup_nonsend(world: &mut World) {
    world.insert_non_send_resource(UnSendable::default());
}

fn hello_world_system(unsend: NonSend<UnSendable>) {
    println!("hello world {:?}", unsend.cell);
}
