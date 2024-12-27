use bevy::{color::palettes::css::*, input::common_conditions::input_pressed, prelude::*};
use bevy_rapier2d::{math::Real, prelude::*};

use crate::input::{update_cursor_world_coords, CursorWorldCoords};

pub struct PlayerManagementPlugin;
impl Plugin for PlayerManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, move_player)
            .add_systems(
                Update,
                shoot_light
                    .run_if(input_pressed(MouseButton::Left))
                    .after(update_cursor_world_coords),
            );
    }
}

#[derive(Component, Default)]
#[require(
    Sprite,
    Collider,
    KinematicCharacterController,
    KinematicCharacterControllerOutput,
    RigidBody,
    Friction,
    Restitution
)]
struct Player {
    // Store my own velocity here, because KinematicCharacterController doesn't use Velocity (the
    // component) and it only supports setting translations
    velocity: Vec2,
}

fn shoot_light(
    q_player: Query<(Entity, &Transform), With<Player>>,
    mut q_rapier: Query<&mut RapierContext>,
    q_cursor: Query<&CursorWorldCoords>,
    mut gizmos: Gizmos,
) {
    let Ok((player_entity, player_transform)) = q_player.get_single() else {
        return;
    };
    let Ok(rapier_context) = q_rapier.get_single_mut() else {
        return;
    };
    let Ok(cursor_pos) = q_cursor.get_single() else {
        return;
    };

    let mut ray_pos = player_transform.translation.truncate();
    let mut ray_dir = cursor_pos.pos - ray_pos;
    let pred = move |entity: Entity| {
        return entity != player_entity;
    };
    let mut ray_qry = QueryFilter::new().predicate(&pred);

    let mut pts: Vec<Vec2> = vec![ray_pos];

    const MAX_LIGHT_SEGMENTS: usize = 50;
    for _ in 0..MAX_LIGHT_SEGMENTS {
        let Some((entity, x)) =
            rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, Real::MAX, true, ray_qry)
        else {
            break;
        };
        if x.time_of_impact < 0.01 {
            break;
        }

        pts.push(x.point);

        ray_pos = x.point;
        ray_dir = ray_dir.reflect(x.normal);
        ray_qry = ray_qry.exclude_collider(entity);
    }

    // FIXME: render with shader/something else and not gizmos
    gizmos.linestrip_gradient_2d(pts.iter().map(|pt| (*pt, RED)));
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Player::default())
        .insert(Sprite::from_image(asset_server.load("bob.png")))
        .insert(Collider::cuboid(8.0, 8.0))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Restitution {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        });
}

fn move_player(
    mut query: Query<(
        &mut KinematicCharacterController,
        &KinematicCharacterControllerOutput,
        &mut Player,
    )>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut controller, output, mut player)) = query.get_single_mut() else {
        return;
    };

    const PLAYER_MAX_H_VEL: f32 = 3.;
    const PLAYER_MAX_Y_VEL: f32 = 5.;
    const PLAYER_AIR_MOVEMENT_COEFF: f32 = 0.2;

    // Use pressed instead of just_pressed because an object on the ground is not grounded every
    // frame??
    let mut jumped = false;
    if keys.pressed(KeyCode::Space) && output.grounded {
        player.velocity.y = 4.;
        jumped = true;
    } else if !keys.pressed(KeyCode::Space) && player.velocity.y > 0. {
        // Jump cutting
        // TODO: ensure minimum length jump
        player.velocity.y = 0.;
    } else if output.grounded {
        player.velocity.y = 0.;
    }
    player.velocity.y = player.velocity.y.clamp(-PLAYER_MAX_Y_VEL, PLAYER_MAX_Y_VEL);
    if !jumped {
        // Gravity
        player.velocity.y -= 0.2;
    }

    let mut delta_v_h = 1.;
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

    controller.translation = Some(player.velocity);
}
