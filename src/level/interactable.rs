use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component)]
pub struct Interactable {
    pub id: i32,
}

pub fn init_interactable(entity_instance: &EntityInstance) -> Interactable {
    let id = entity_instance
        .get_int_field("id")
        .expect("Interactable id should exist and be an integer");

    Interactable { id: *id }
}
