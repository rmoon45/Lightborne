use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::tiles::TileTextureIndex;
use bevy_rapier2d::prelude::*;

use crate::{light::LightColor, shared::ResetLevel};

use super::LevelSystems;

/// [`Plugin`] for managing all things related to [`Crystal`]s. This plugin responds to the
/// addition and removal of [`Activated`] [`Component`]s and updates the sprite and collider of
/// each crystal entity, in addition to handling initialization and cleanup on a [`LevelSwitchEvent`].
pub struct CrystalPlugin;

impl Plugin for CrystalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CrystalToggleEvent>()
            .add_systems(Update, process_crystals.in_set(LevelSystems::Processing))
            .add_systems(Update, on_crystal_changed.in_set(LevelSystems::Simulation))
            .add_systems(FixedUpdate, reset_crystals.run_if(on_event::<ResetLevel>));

        // Current crystals
        for i in 3..=6 {
            app.register_ldtk_int_cell::<CrystalBundle>(i);
        }
    }
}

/// Marker [`Component`] used to query for crystals, currently does not contain any information.
#[derive(Default, Component)]
pub struct Crystal {
    init_active: bool,
    active: bool,
    color: LightColor,
}

/// Function to determine whether or not a cell value represents an Active Crystal. Does not use
/// the modulo operator as future crystal cell values need not necessarily follow the same pattern
/// in the future.
fn is_crystal_active(cell_value: i32) -> bool {
    match cell_value {
        3 | 5 => true,
        4 | 6 => false,
        _ => panic!("Cell value does not correspond to crystal!"),
    }
}

impl From<IntGridCell> for Crystal {
    fn from(cell: IntGridCell) -> Self {
        let init_active = is_crystal_active(cell.value);

        Crystal {
            init_active,
            active: init_active,
            color: cell.into(),
        }
    }
}

/// [`Bundle`] registered with [`LdktEntityAppExt::register_ldtk_entity`](LdtkEntityAppExt) to spawn
/// crystals directly from Ldtk.
#[derive(Default, Bundle, LdtkIntCell)]
pub struct CrystalBundle {
    #[from_int_grid_cell]
    crystal: Crystal,
    #[from_int_grid_cell]
    cell: IntGridCell,
}

fn process_crystals(
    mut commands: Commands,
    q_crystals: Query<(Entity, &IntGridCell), Added<Crystal>>,
) {
    for (entity, cell) in q_crystals.iter() {
        if is_crystal_active(cell.value) {
            commands.entity(entity).insert(Collider::cuboid(4.0, 4.0));
        }
    }
}

/// The horizontal offset between active crystals and inactive crystals in the crystal tilemap
const CRYSTAL_INDEX_OFFSET: u32 = 5;

/// Switches a crystal from inactive to active. Calling this on an already active crystal will
/// result in weird behavior.
fn activate_crystal(
    commands: &mut Commands,
    crystal_entity: Entity,
    crystal_index: &mut TileTextureIndex,
) {
    commands
        .entity(crystal_entity)
        .insert(Collider::cuboid(4.0, 4.0));
    crystal_index.0 -= CRYSTAL_INDEX_OFFSET;
}

/// Switches a crystal from active to inactive. Calling this on an already inactive crystal will
/// result in weird behavior.
fn deactivate_crystal(
    commands: &mut Commands,
    crystal_entity: Entity,
    crystal_index: &mut TileTextureIndex,
) {
    commands.entity(crystal_entity).remove::<Collider>();
    crystal_index.0 += CRYSTAL_INDEX_OFFSET;
}

/// [`System`] that listens to [`LevelSwitchEvent`]s to ensure that [`Crystal`] states are reset
/// when switching between rooms.
pub fn reset_crystals(
    mut commands: Commands,
    mut q_crystals: Query<(Entity, &mut Crystal, &mut TileTextureIndex)>,
) {
    for (entity, mut crystal, mut index) in q_crystals.iter_mut() {
        if crystal.active == crystal.init_active {
            continue;
        }
        if crystal.init_active {
            activate_crystal(&mut commands, entity, &mut index);
            crystal.active = true;
        } else {
            deactivate_crystal(&mut commands, entity, &mut index);
            crystal.active = false;
        }
    }
}

/// Event that will toggle all crystals of a certain color.
#[derive(Event)]
pub struct CrystalToggleEvent(pub LightColor);

/// [`System`] that listens to when [`Crystal`]s are activated or deactivated, updating the
/// [`Sprite`] and adding/removing [`FixedEntityBundle`] of the [`Entity`].
pub fn on_crystal_changed(
    mut commands: Commands,
    mut q_crystals: Query<(Entity, &mut Crystal, &mut TileTextureIndex)>,
    mut crystal_toggle_ev: EventReader<CrystalToggleEvent>,
) {
    if crystal_toggle_ev.is_empty() {
        return;
    }
    let mut to_toggle: HashSet<LightColor> = HashSet::new();
    for ev in crystal_toggle_ev.read() {
        dbg!("Toggling crystals for", ev.0);
        to_toggle.insert(ev.0);
    }
    for (entity, mut crystal, mut index) in q_crystals.iter_mut() {
        if !to_toggle.contains(&crystal.color) {
            continue;
        }
        if crystal.active {
            deactivate_crystal(&mut commands, entity, &mut index);
            crystal.active = false;
        } else {
            activate_crystal(&mut commands, entity, &mut index);
            crystal.active = true;
        }
    }
}
