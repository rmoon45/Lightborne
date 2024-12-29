use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{PlayerMarker, PlayerMovement};

pub fn queue_jump(mut q_player: Query<&mut PlayerMovement, With<PlayerMarker>>) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };
    player.jump_queued.reset();
}

pub fn move_player(
    time: Res<Time>,
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

    const PLAYER_MAX_H_VEL: f32 = 1.5;
    const PLAYER_MAX_Y_VEL: f32 = 5.;
    const PLAYER_AIR_MOVEMENT_COEFF: f32 = 0.7;
    const PLAYER_JUMP_VEL: f32 = 2.5;

    if output.grounded {
        player.coyote_time.reset();
    }

    let mut jumped = false;
    // Can only jump if they've pressed space within the past 100 millis, and they have been
    // grounded in the past 100 millis
    if !player.jump_queued.finished() && !player.coyote_time.finished() {
        player.velocity.y = PLAYER_JUMP_VEL;
        player.prevent_jump_cut.reset();
        jumped = true;
    } else if !keys.pressed(KeyCode::Space)
        && player.velocity.y > 0.
        && player.prevent_jump_cut.finished()
    {
        // Jump was cut
        player.velocity.y = 0.;
    } else if output.desired_translation.y - output.effective_translation.y > 0.05 {
        // Bonked head onto wall
        player.velocity.y = 0.;
    } else if output.grounded {
        player.velocity.y = 0.;
    }
    player.velocity.y = player.velocity.y.clamp(-PLAYER_MAX_Y_VEL, PLAYER_MAX_Y_VEL);
    if !jumped {
        // Gravity
        player.velocity.y -= 0.2;
    }

    let mut delta_v_h = 0.3;
    if !output.grounded {
        delta_v_h *= PLAYER_AIR_MOVEMENT_COEFF
    };
    let mut moved = false;
    if keys.pressed(KeyCode::KeyA) {
        // Change dirs instantly
        if player.velocity.x > 0. {
            player.velocity.x = 0.;
        }
        player.velocity.x -= delta_v_h;
        moved = true;
    }
    if keys.pressed(KeyCode::KeyD) {
        // Change dirs instantly
        if player.velocity.x < 0. {
            player.velocity.x = 0.;
        }
        player.velocity.x += delta_v_h;
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

    player.prevent_jump_cut.tick(time.delta());
    player.jump_queued.tick(time.delta());
    player.coyote_time.tick(time.delta());

    controller.translation = Some(player.velocity);
}
