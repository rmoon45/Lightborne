use bevy::prelude::*;
use bevy_rapier2d::{math::Real, prelude::*};

use crate::{input::CursorWorldCoords, player::PlayerMarker, shared::GroupLabel};

use super::{HitByLight, LightSegment, LightSensor, MAX_LIGHT_SEGMENTS};

pub fn shoot_light(
    mut commands: Commands,
    q_player: Query<&Transform, With<PlayerMarker>>,
    mut q_rapier: Query<&mut RapierContext>,
    q_cursor: Query<&CursorWorldCoords>,
    q_light_sensor: Query<&LightSensor>,
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

        if q_light_sensor.contains(entity) {
            commands.entity(entity).insert(HitByLight);
            break;
        };

        ray_pos = intersection.point;
        ray_dir = ray_dir.reflect(intersection.normal);
        ray_qry = ray_qry.exclude_collider(entity);
    }

    let entities: Vec<LightSegment> = pts
        .windows(2)
        .map(|segment| LightSegment {
            start: segment[0],
            end: segment[1],
        })
        .collect();

    commands.spawn_batch(entities);
}
