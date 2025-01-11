use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::{
    activatable::{init_activatable, update_activatables, Activatable, Activated},
    entity::FixedEntityBundle,
};

pub struct CrystalPlugin;

impl Plugin for CrystalPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<CrystalBundle>("RedCrystal")
            .register_ldtk_entity::<CrystalBundle>("GreenCrystal")
            .add_systems(
                Update,
                (on_crystal_added, on_crystal_changed).after(update_activatables),
            );
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
        if !activatable.init_active {
            sprite.color.set_alpha(0.1);
            commands.entity(entity).remove::<FixedEntityBundle>();
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
        sprite.color.set_alpha(1.0);
        commands
            .entity(entity)
            .insert(FixedEntityBundle::from(instance));
    }
    for entity in q_deactivated.read() {
        let Ok((entity, mut sprite, _)) = q_crystals.get_mut(entity) else {
            continue;
        };
        sprite.color.set_alpha(0.1);
        commands.entity(entity).remove::<FixedEntityBundle>();
    }
}
