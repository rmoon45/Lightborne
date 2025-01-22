use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::shared::GameState;

use super::{
    activatable::{init_activatable, update_activatables, Activatable, Activated},
    entity::FixedEntityBundle,
    LevelSystems,
};

/// [`Plugin`] for managing all things related to [`Crystal`]s. This plugin responds to the
/// addition and removal of [`Activated`] [`Component`]s and updates the sprite and collider of
/// each crystal entity, in addition to handling initialization and cleanup on a [`LevelSwitchEvent`].
pub struct CrystalPlugin;

impl Plugin for CrystalPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<CrystalBundle>("RedCrystal")
            .register_ldtk_entity::<CrystalBundle>("GreenCrystal")
            .add_systems(PreUpdate, on_crystal_added.in_set(LevelSystems::Processing))
            .add_systems(
                Update,
                on_crystal_changed
                    .after(update_activatables)
                    .in_set(LevelSystems::Simulation),
            )
            .add_systems(OnEnter(GameState::Playing), reset_crystals);
    }
}

/// Marker [`Component`] used to query for crystals, currently does not contain any information.
#[derive(Default, Component)]
pub struct Crystal;

/// [`Bundle`] registered with [`LdktEntityAppExt::register_ldtk_entity`](LdtkEntityAppExt) to spawn
/// crystals directly from Ldtk.
#[derive(Default, Bundle, LdtkEntity)]
pub struct CrystalBundle {
    marker: Crystal,
    #[from_entity_instance]
    collider: FixedEntityBundle,
    #[sprite_sheet]
    sprite: Sprite,
    #[from_entity_instance]
    instance: EntityInstance,
    #[with(init_activatable)]
    activatable: Activatable,
}

/// [`System`] to ensure that the sprite color and collider match the initially activated state of
/// the [`Crystal`].
pub fn on_crystal_added(
    mut commands: Commands,
    mut q_new: Query<(Entity, &mut Sprite, &Activatable), Added<Crystal>>,
) {
    // Fix crystals that start inactive
    for (entity, mut sprite, activatable) in q_new.iter_mut() {
        sprite.color = Color::srgba(1.3, 1.3, 1.3, 1.0);
        if !activatable.init_active {
            sprite.color = Color::srgba(1.0, 1.0, 1.0, 0.1);
            commands.entity(entity).remove::<FixedEntityBundle>();
        }
    }
}

/// [`System`] that listens to [`LevelSwitchEvent`]s to ensure that [`Crystal`] states are reset
/// when switching between rooms.
pub fn reset_crystals(
    mut commands: Commands,
    mut q_crystals: Query<(Entity, &Activatable), With<Crystal>>,
) {
    for (entity, activatable) in q_crystals.iter_mut() {
        if activatable.init_active {
            commands.entity(entity).insert(Activated);
        } else {
            commands.entity(entity).remove::<Activated>();
        }
    }
}

/// [`System`] that listens to when [`Crystal`]s are activated or deactivated, updating the
/// [`Sprite`] and adding/removing [`FixedEntityBundle`] of the [`Entity`].
///
/// We should instead consider inserting a [`Sensor`](bevy_rapier2d::prelude::Sensor) component and updating the entity's
/// [`CollisionGroups`](bevy_rapier2d::prelude::CollisionGroups) instead, if we are worried about the entity changing its
/// Archetype and causing performance issues (not likely, [`Crystal`]s do not change state often).
pub fn on_crystal_changed(
    mut commands: Commands,
    mut q_activated: Query<Entity, (With<Crystal>, Added<Activated>)>,
    mut q_deactivated: RemovedComponents<Activated>,

    mut q_crystals: Query<(Entity, &mut Sprite, &EntityInstance), With<Crystal>>,
) {
    for entity in q_activated.iter_mut() {
        let Ok((entity, mut sprite, instance)) = q_crystals.get_mut(entity) else {
            continue;
        };
        sprite.color = Color::srgba(1.3, 1.3, 1.3, 1.0);
        commands
            .entity(entity)
            .insert(FixedEntityBundle::from(instance));
    }
    for entity in q_deactivated.read() {
        let Ok((entity, mut sprite, _)) = q_crystals.get_mut(entity) else {
            continue;
        };
        sprite.color = Color::srgba(1.0, 1.0, 1.0, 0.1);
        commands.entity(entity).remove::<FixedEntityBundle>();
    }
}
