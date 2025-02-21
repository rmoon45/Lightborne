use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{spawn::PlayerHurtMarker, PlayerMarker};

/// The number of [`FixedUpdate`] steps the player can jump for after pressing the spacebar.
const SHOULD_JUMP_TICKS: isize = 8;
/// The number of [`FixedUpdate`] steps the player can jump for after falling off an edge.
const COYOTE_TIME_TICKS: isize = 5;
/// The number of [`FixedUpdate`] steps the player should receive upward velocity for.
const JUMP_BOOST_TICKS: isize = 2;

/// Max player horizontal velocity.
const PLAYER_MAX_H_VEL: f32 = 1.5;
/// Max player vertical velocity.
const PLAYER_MAX_Y_VEL: f32 = 5.;
/// The positive y velocity added to the player every jump boost tick.
const PLAYER_JUMP_VEL: f32 = 2.2;
/// The x velocity added to the player when A/D is held.
const PLAYER_MOVE_VEL: f32 = 0.6;
/// The y velocity subtracted from the player due to gravity.
const PLAYER_GRAVITY: f32 = 0.15;

/// [`Component`] that stores information about the player's movement state.
#[derive(Component, Default)]
pub struct PlayerMovement {
    /// Holds information that is passed into the rapier character controller's translation
    velocity: Vec2,
    pub crouching: bool,
    should_jump_ticks_remaining: isize,
    coyote_time_ticks_remaining: isize,
    jump_boost_ticks_remaining: isize,
}

#[derive(Component, Default, PartialEq, Eq)]
pub enum PlayerState {
    #[default]
    Idle,
    Jumping,
    Falling,
    Moving,
}

/// [`System`] that is run the frame the space bar is pressed. Allows the player to jump for the
/// next couple of frames.
pub fn queue_jump(mut q_player: Query<&mut PlayerMovement, With<PlayerMarker>>) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };
    player.should_jump_ticks_remaining = SHOULD_JUMP_TICKS;
}

/// [`System`] that is run on [`Update`] to crouch player
pub fn crouch_player(
    // query transform
    mut q_player: Query<(&mut PlayerMovement, &mut Transform), With<PlayerMarker>>,
    mut q_hitbox: Query<&mut Transform, (With<PlayerHurtMarker>, Without<PlayerMarker>)>,
    //ButtonInput<KeyCode> resource (access resource)
    keys: Res<ButtonInput<KeyCode>>,
) {
    // ensure only 1 candidate to match query; let Ok = pattern matching
    let Ok((mut player, mut transform)) = q_player.get_single_mut() else {
        return;
    };
    let Ok(mut hitbox_transform) = q_hitbox.get_single_mut() else {
        return;
    };
    hitbox_transform.translation = Vec3::new(0., 0., 0.);

    if keys.just_pressed(KeyCode::KeyS) && !player.crouching {
        // decrease size by half
        transform.scale.y *= 0.5;
        transform.translation.y -= 5.0;
        player.crouching = true;
    }
    if keys.just_released(KeyCode::KeyS) && player.crouching {
        transform.scale.y *= 2.0;
        transform.translation.y += 5.0;
        player.crouching = false;
    }
}

pub fn update_player_state(
    mut q_player: Query<
        (&mut PlayerState, &KinematicCharacterControllerOutput),
        With<PlayerMarker>,
    >,
) {
    let Ok((mut state, output)) = q_player.get_single_mut() else {
        return;
    };

    if output.effective_translation.y > 0.0 {
        *state = PlayerState::Jumping;
    } else if output.effective_translation.y <= 0.0 && !output.grounded {
        *state = PlayerState::Falling;
    } else if output.grounded && output.effective_translation.x.abs() > 0.1 {
        *state = PlayerState::Moving;
    } else if output.grounded {
        *state = PlayerState::Idle;
    }
}

/// [`System`] that is run on [`Update`] to move the player around.
pub fn move_player(
    mut q_player: Query<
        (
            &mut KinematicCharacterController,
            &KinematicCharacterControllerOutput,
            &mut PlayerMovement,
        ),
        With<PlayerMarker>,
    >,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut controller, output, mut player)) = q_player.get_single_mut() else {
        return;
    };

    if output.grounded {
        player.coyote_time_ticks_remaining = COYOTE_TIME_TICKS;
    }

    // Can only jump if they've pressed space within the past SHOULD_JUMP_TICKS, and they have been
    // grounded in the past COYOTE_TIME_TICKS
    if player.should_jump_ticks_remaining > 0 && player.coyote_time_ticks_remaining > 0 {
        player.jump_boost_ticks_remaining = JUMP_BOOST_TICKS;
    } else if !keys.pressed(KeyCode::Space) && player.velocity.y > 0. {
        // Jump was cut
        player.velocity.y = PLAYER_GRAVITY;
        player.jump_boost_ticks_remaining = 0;
    } else if output.desired_translation.y > 0. && output.effective_translation.y < 0.05 {
        // Bonked head onto wall
        player.velocity.y = 0.;
        player.jump_boost_ticks_remaining = 0;
    } else if output.grounded {
        player.velocity.y = 0.;
    }

    if player.jump_boost_ticks_remaining > 0 {
        player.velocity.y = PLAYER_JUMP_VEL;
    } else {
        player.velocity.y -= PLAYER_GRAVITY;
    }

    player.velocity.y = player.velocity.y.clamp(-PLAYER_MAX_Y_VEL, PLAYER_MAX_Y_VEL);

    let mut moved = false;
    if keys.pressed(KeyCode::KeyA) {
        player.velocity.x -= PLAYER_MOVE_VEL;
        moved = true;
    }
    if keys.pressed(KeyCode::KeyD) {
        player.velocity.x += PLAYER_MOVE_VEL;
        moved = true;
    }
    player.velocity.x = player.velocity.x.clamp(-PLAYER_MAX_H_VEL, PLAYER_MAX_H_VEL);
    if !moved {
        // slow player down when not moving horizontally
        // NOTE: why not using rapier friction?
        player.velocity.x *= 0.6;
        if player.velocity.x.abs() < 0.1 {
            player.velocity.x = 0.;
        }
    }

    player.should_jump_ticks_remaining -= 1;
    player.jump_boost_ticks_remaining -= 1;
    player.coyote_time_ticks_remaining -= 1;

    controller.translation = Some(player.velocity);
}
