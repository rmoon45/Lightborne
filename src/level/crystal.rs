use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::{
    activatable::{init_activatable, update_activatables, Activatable, Activated},
    entity::FixedEntityBundle,
    LevelSwitchEvent,
};

pub struct CrystalPlugin;

impl Plugin for CrystalPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<CrystalBundle>("RedCrystal")
            .register_ldtk_entity::<CrystalBundle>("GreenCrystal")
            .add_systems(
                Update,
                (on_crystal_added, on_crystal_changed).after(update_activatables),
            )
            .add_systems(Update, reset_crystals);
    }
}

#[derive(Default, Component)]
pub struct Crystal;

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

pub fn reset_crystals(
    mut commands: Commands,
    mut ev_level_switch: EventReader<LevelSwitchEvent>,
    mut q_crystals: Query<(Entity, &Activatable), With<Crystal>>,
) {
    if ev_level_switch.is_empty() {
        return;
    }
    ev_level_switch.clear();

    for (entity, activatable) in q_crystals.iter_mut() {
        if activatable.init_active {
            commands.entity(entity).insert(Activated);
        } else {
            commands.entity(entity).remove::<Activated>();
        }
    }
}

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
