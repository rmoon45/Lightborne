use bevy::prelude::*;
use bevy_ecs_ldtk::*;
use prelude::LdtkFields;

use crate::light::sensor::update_light_sensors;

use super::LevelSystems;

/// [`Plugin`] that manages all activatables in the game, e.g. [`LightSensor`](crate::light::sensor::LightSensor).
/// See [`Activatable`] for more information.
pub struct ActivatablePlugin;

impl Plugin for ActivatablePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActivatableCache>()
            .add_event::<GroupTriggeredEvent>()
            .add_systems(
                PreUpdate,
                setup_activatables.in_set(LevelSystems::Processing),
            )
            .add_systems(
                Update,
                update_activatables
                    .after(update_light_sensors)
                    .in_set(LevelSystems::Simulation),
            );
    }
}

/// [`Component`] that shuold be inserted into any entity that is activatable. The
/// [`ActivatablePlugin`] will manage each activatable's state by listening to
/// [`GroupTriggeredEvent`]s, and inserting [`Activated`] components on the entities, if they
/// should be activated. Each entity can then listen for changes with [`Added`] or
/// [`RemovedComponents`], and apply their logic in their corresponding [`System`].
#[derive(Default, Component)]
pub struct Activatable {
    /// The corresponding id of an [`Interactable`][super::interactable::Interactable::id].
    pub id: i32,
    pub init_active: bool,
}

/// [`Component`] that indicates whether or not an entity should be activated or not. See
/// [`Activatable`] for more information.
#[derive(Default, Component)]
#[component(storage = "SparseSet")]
pub struct Activated;

/// [`Resource`] used to store the entity handles of each [`Activatable`] by their id, for faster
/// performance.
#[derive(Default, Resource)]
pub struct ActivatableCache {
    // FIXME: separate activatables by room
    table: std::collections::HashMap<i32, std::collections::HashSet<Entity>>,
}

/// [`Event`] that should be sent from an [`Interactable`](crate::level::interactable::Interactable) to notify
/// this plugin to add or remove [`Activated`] markers.
#[derive(Event)]
pub struct GroupTriggeredEvent {
    pub id: i32,
}

/// The [`Update`] [`System`] responsible for responding to [`GroupTriggeredEvent`]s, and inserting
/// [`Activated`] markers.
pub fn update_activatables(
    mut commands: Commands,
    mut ev_group_triggered: EventReader<GroupTriggeredEvent>,
    mut activatable_cache: ResMut<ActivatableCache>,
    q_activated: Query<&Activated>,
) {
    for event in ev_group_triggered.read() {
        let id = event.id;
        if !activatable_cache.table.contains_key(&id) {
            continue;
        }

        let mut to_remove: Vec<Entity> = vec![];
        for &entity in activatable_cache.table[&id].iter() {
            if commands.get_entity(entity).is_none() {
                to_remove.push(entity);
                continue;
            }

            if let Ok(_) = q_activated.get(entity) {
                commands.entity(entity).remove::<Activated>();
            } else {
                commands.entity(entity).insert(Activated);
            }
        }
        for entity in to_remove.iter() {
            activatable_cache
                .table
                .get_mut(&id)
                .expect("Entry exists in hashmap if we are updating its entries")
                .remove(entity);
        }
    }
}

/// Initialization function to be used to intialize [`Activatable`] LDTK entities. See
/// [`crate::level::crystal::CrystalBundle`] for an example of how this can be done.
pub fn init_activatable(entity_instance: &EntityInstance) -> Activatable {
    let id = entity_instance
        .get_int_field("id")
        .expect("Activatable id should exist and be an integer");

    let active = entity_instance
        .get_bool_field("active")
        .expect("Activatable active status should exist and be a boolean");

    Activatable {
        id: *id,
        init_active: *active,
    }
}

/// When [`Activatable`]s are spawned, we need to insert the [`Activated`] component on entities
/// that should be intially active. This is where that is done.
pub fn setup_activatables(
    mut commands: Commands,
    mut activatable_cache: ResMut<ActivatableCache>,
    q_activatable: Query<(Entity, &Activatable), Added<Activatable>>,
) {
    for (entity, activatable) in q_activatable.iter() {
        if activatable.init_active {
            commands.entity(entity).insert(Activated);
        }

        activatable_cache
            .table
            .entry(activatable.id)
            .or_default()
            .insert(entity);
    }
}
