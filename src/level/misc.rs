use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{
    activatable::{init_activatable, Activatable},
    entity::FixedEntityBundle,
};
use crate::light::LightSensorBundle;

#[derive(Default, Component)]
pub struct DoorMarker;

#[derive(Default, Bundle, LdtkEntity)]
pub struct DoorBundle {
    marker: DoorMarker,
    #[with(init_activatable)]
    activatable: Activatable,
    #[sprite_sheet]
    sprite: Sprite,
    #[from_entity_instance]
    entity_info: FixedEntityBundle,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

pub fn on_door_changed(
    mut commands: Commands,
    mut q_door: Query<
        (
            Entity,
            &DoorMarker,
            &Activatable,
            &mut Sprite,
            &EntityInstance,
        ),
        Changed<Activatable>,
    >,
) {
    for (entity, _, affectable, mut sprite, entity_instance) in q_door.iter_mut() {
        let active = affectable.active;
        if active {
            commands.entity(entity).remove::<FixedEntityBundle>();
            sprite.color.set_alpha(0.1);
        } else {
            commands
                .entity(entity)
                .insert(FixedEntityBundle::from(entity_instance));
            sprite.color.set_alpha(1.0);
        }
    }
}

#[derive(Default, Component)]
pub struct ButtonMarker;

#[derive(Default, Bundle, LdtkEntity)]
pub struct ButtonBundle {
    marker: ButtonMarker,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[from_entity_instance]
    light_sensor: LightSensorBundle,
}
