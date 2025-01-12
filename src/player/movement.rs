use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::PlayerMarker;

const SHOULD_JUMP_TICKS: isize = 8;
const COYOTE_TIME_TICKS: isize = 5;
const JUMP_BOOST_TICKS: isize = 2;

const PLAYER_MAX_H_VEL: f32 = 1.5;
const PLAYER_MAX_Y_VEL: f32 = 5.;
const PLAYER_JUMP_VEL: f32 = 2.2;
const PLAYER_MOVE_VEL: f32 = 0.6;
const PLAYER_GRAVITY: f32 = 0.15;

#[derive(Component, Default)]
pub struct PlayerMovement {
    /// Holds information that is passed into the rapier character controller's translation
    velocity: Vec2,
    should_jump_ticks_remaining: isize,
    coyote_time_ticks_remaining: isize,
    jump_boost_ticks_remaining: isize,
}

pub fn queue_jump(mut q_player: Query<&mut PlayerMovement, With<PlayerMarker>>) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };
    player.should_jump_ticks_remaining = SHOULD_JUMP_TICKS;
}

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
    } else if output.desired_translation.y - output.effective_translation.y > 0.05 {
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
