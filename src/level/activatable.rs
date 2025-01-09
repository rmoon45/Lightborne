use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Resource)]
pub struct ActivatableCache {
    table: std::collections::HashMap<i32, Vec<Entity>>,
}

#[derive(Default, Component)]
pub struct Activatable {
    pub id: i32,
    pub active: bool,
}

pub struct ActivatablePlugin;

impl Plugin for ActivatablePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActivatableCache>()
            .add_systems(Update, cache_activatables)
            .add_systems(Update, update_activatables.after(cache_activatables));
    }
}

#[derive(Default, Component)]
#[component(storage = "SparseSet")]
pub struct ActiveActivatableGroup {
    pub id: i32,
}

pub fn update_activatables(
    q_new_activatables: Query<Entity, Added<ActiveActivatableGroup>>,
    mut q_del_activatables: RemovedComponents<ActiveActivatableGroup>,
    q_activatable_group: Query<&ActiveActivatableGroup>,
    mut q_activatable: Query<&mut Activatable>,
    activatable_cache: Res<ActivatableCache>,
) {
    let mut set_activatable_group = |group: i32, active: bool| {
        if !activatable_cache.table.contains_key(&group) {
            return;
        }
        activatable_cache.table[&group].iter().for_each(|entity| {
            q_activatable.get_mut(*entity).unwrap().active = active;
        });
    };

    for new_activatable_group in q_new_activatables.iter() {
        let id = q_activatable_group.get(new_activatable_group).unwrap().id;
        set_activatable_group(id, true);
    }

    for del_activatable_group in q_del_activatables.read() {
        let id = q_activatable_group.get(del_activatable_group).unwrap().id;
        set_activatable_group(id, false);
    }
}

pub fn cache_activatables(
    q_activatable: Query<(Entity, &Activatable), Added<Activatable>>,
    mut activatable_cache: ResMut<ActivatableCache>,
) {
    for (entity, activatable) in q_activatable.iter() {
        activatable_cache
            .table
            .entry(activatable.id)
            .or_default()
            .push(entity);
    }
}

pub fn init_activatable(entity_instance: &EntityInstance) -> Activatable {
    let id = entity_instance
        .get_int_field("id")
        .expect("Activatable id should exist and be an integer");

    let active = entity_instance
        .get_bool_field("active")
        .expect("Activatable active status should exist and be a boolean");

    Activatable {
        id: *id,
        active: *active,
    }
}
