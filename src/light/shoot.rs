use bevy::{color::palettes::css::*, prelude::*};
use bevy_rapier2d::{math::Real, prelude::*};

use crate::{input::CursorWorldCoords, player::PlayerMarker, shared::GroupLabel};

use super::{HitByLight, LightSensor};

pub fn shoot_light(
    mut commands: Commands,
    q_player: Query<&Transform, With<PlayerMarker>>,
    mut q_rapier: Query<&mut RapierContext>,
    q_cursor: Query<&CursorWorldCoords>,
    q_light_material: Query<&LightSensor>,
    mut gizmos: Gizmos,
) {
    let Ok(player_transform) = q_player.get_single() else {
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
    let mut ray_qry = QueryFilter::new().groups(CollisionGroups::new(
        GroupLabel::LIGHT_RAY,
        GroupLabel::TERRAIN | GroupLabel::LIGHT_SENSOR,
    ));

    let mut pts: Vec<Vec2> = vec![ray_pos];

    const MAX_LIGHT_SEGMENTS: usize = 3;
    for _ in 0..MAX_LIGHT_SEGMENTS {
        let Some((entity, intersection)) =
            rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, Real::MAX, true, ray_qry)
        else {
            break;
        };

        if intersection.time_of_impact < 0.01 {
            break;
        }

        pts.push(intersection.point);

        if q_light_material.contains(entity) {
            commands.entity(entity).insert(HitByLight);
            break;
        };

        ray_pos = intersection.point;
        ray_dir = ray_dir.reflect(intersection.normal);
        ray_qry = ray_qry.exclude_collider(entity);
    }

    // FIXME: render with shader/something else and not gizmos
    gizmos.linestrip_gradient_2d(pts.iter().map(|pt| (*pt, RED)));
}
