use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

/// [`Component`] inserted into entities that are interactable.
#[derive(Default, Component)]
pub struct Interactable {
    /// Corresponds to the id field on an [`Activatable`][super::activatable::Activatable::id] that
    /// should toggle its [`Activated`][super::activatable::Activated] state when this interactable is
    /// triggered.
    pub id: i32,
}

#[derive(Component)]
pub struct InteractableSFX {
    pub on_triggered: Option<Handle<AudioSource>>,
}

/// Used with the proc macro `#[with(init_interactable)]` to inialize the [`Interactable`]
/// [`Component`] on a given [`Bundle`] registered with Ldtk as an [`LdtkEntity`].
pub fn init_interactable(entity_instance: &EntityInstance) -> Interactable {
    let id = entity_instance
        .get_int_field("id")
        .expect("Interactable id should exist and be an integer");

    Interactable { id: *id }
}
