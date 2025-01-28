use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use enum_map::EnumMap;

use super::{
    render::{LightMaterial, LightRenderData},
    sensor::{HitByLightEvent, LightSensor},
    LightColor, LightRaySource, LIGHT_SPEED,
};
use crate::shared::GroupLabel;

/// Marker [`Component`] used to query for light segments.
#[derive(Default, Component, Clone, Debug)]
pub struct LightSegmentMarker;

/// [`Bundle`] used in the initialization of the [`LightSegmentCache`] to spawn segment entities.
#[derive(Bundle, Debug, Default, Clone)]
pub struct LightSegmentBundle {
    pub marker: LightSegmentMarker,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<LightMaterial>,
    pub visibility: Visibility,
    pub transform: Transform,
}

/// [`Resource`] used to store [`Entity`] handles to the light segments so they aren't added and
/// despawned every frame. See [`simulate_light_sources`] for details.
#[derive(Resource)]
pub struct LightSegmentCache {
    table: EnumMap<LightColor, Vec<Entity>>,
}

impl FromWorld for LightSegmentCache {
    fn from_world(world: &mut World) -> Self {
        let mut cache = LightSegmentCache {
            table: EnumMap::default(),
        };
        let render_data = world.resource::<LightRenderData>();

        let mut segment_bundles: EnumMap<LightColor, LightSegmentBundle> = EnumMap::default();

        for (color, _) in cache.table.iter_mut() {
            segment_bundles[color] = LightSegmentBundle {
                marker: LightSegmentMarker,
                mesh: render_data.mesh.clone(),
                material: render_data.material_map[color].clone(),
                visibility: Visibility::Visible,
                transform: Transform::default(),
            }
        }

        for (color, segments) in cache.table.iter_mut() {
            while segments.len() < color.num_bounces() + 1 {
                let mut cmds = world.spawn(());
                cmds.insert(segment_bundles[color].clone());

                // White beams need colliders
                if color == LightColor::White {
                    cmds.insert((
                        Collider::cuboid(0.5, 0.5),
                        Sensor,
                        CollisionGroups::new(
                            GroupLabel::WHITE_RAY,
                            GroupLabel::TERRAIN | GroupLabel::LIGHT_SENSOR | GroupLabel::LIGHT_RAY,
                        ),
                    ));
                }

                segments.push(cmds.id());
            }
        }

        cache
    }
}

/// [`System`] that runs on [`Update`], calculating the [`Transform`] of light segments from the
/// corresponding [`LightRaySource`]. Note that this calculation happens every frame, so instead of
/// rapidly spawning/despawning the entities, we spawn them and cache them in the
/// [`LightSegmentCache`], then modify their [`Visibility`] and [`Transform`]s.
///
/// If needed, optimization work can be done by recalculating only segments that are currently
/// changing (segments already "stabilized" usually won't move).
///
/// Similar logic is duplicated in [`preview_light_path`](crate::player::light::preview_light_path),
/// these two systems should be merged.
pub fn simulate_light_sources(
    q_light_sources: Query<&LightRaySource>,
    mut q_rapier: Query<&mut RapierContext>,
    mut ev_hit_by_light: EventWriter<HitByLightEvent>,
    q_light_sensor: Query<&LightSensor>,
    mut q_segments: Query<(&mut Transform, &mut Visibility), With<LightSegmentMarker>>,
    segment_cache: Res<LightSegmentCache>,
) {
    let Ok(rapier_context) = q_rapier.get_single_mut() else {
        return;
    };

    for source in q_light_sources.iter() {
        let mut ray_pos = source.start_pos;
        let mut ray_dir = source.start_dir;
        let collision_groups = match source.color {
            LightColor::White => CollisionGroups::new(
                GroupLabel::WHITE_RAY,
                GroupLabel::TERRAIN | GroupLabel::LIGHT_SENSOR,
            ),
            _ => CollisionGroups::new(
                GroupLabel::LIGHT_RAY,
                GroupLabel::TERRAIN | GroupLabel::LIGHT_SENSOR | GroupLabel::WHITE_RAY,
            ),
        };

        let mut ray_qry = QueryFilter::new().groups(collision_groups);

        let mut pts: Vec<Vec2> = vec![ray_pos];
        let mut remaining_time = source.time_traveled;
        for _ in 0..source.color.num_bounces() + 1 {
            let Some((entity, intersection)) = rapier_context.cast_ray_and_get_normal(
                ray_pos,
                ray_dir,
                remaining_time,
                true,
                ray_qry,
            ) else {
                let final_point = ray_pos + ray_dir * remaining_time;
                pts.push(final_point);
                break;
            };

            if intersection.time_of_impact < 0.01 {
                break;
            }

            remaining_time -= intersection.time_of_impact;

            pts.push(intersection.point);

            if q_light_sensor.contains(entity) {
                ev_hit_by_light.send(HitByLightEvent(entity));
            };

            ray_pos = intersection.point;
            ray_dir = ray_dir.reflect(intersection.normal);
            ray_qry = ray_qry.exclude_collider(entity);
        }

        for (i, pair) in pts.windows(2).enumerate() {
            let midpoint = pair[0].midpoint(pair[1]).extend(1.0);
            let scale = Vec3::new(pair[0].distance(pair[1]), 1., 1.);
            let rotation = (pair[1] - pair[0]).to_angle();
            let transform = Transform::from_translation(midpoint)
                .with_scale(scale)
                .with_rotation(Quat::from_rotation_z(rotation));

            let Ok((mut c_transform, mut c_visibility)) =
                q_segments.get_mut(segment_cache.table[source.color][i])
            else {
                panic!("Segment did not have visibility or transform");
            };

            *c_transform = transform;
            *c_visibility = Visibility::Visible;
        }
    }
}

/// [`System`] that runs on [`FixedUpdate`], advancing the distance the light beam can travel.
pub fn tick_light_sources(mut q_light_sources: Query<&mut LightRaySource>) {
    for mut source in q_light_sources.iter_mut() {
        source.time_traveled += LIGHT_SPEED;
    }
}

/// [`System`] that is responsible for hiding all of the [`LightSegment`](LightSegmentBundle)s
/// and despawning [`LightRaySource`]s when the level changes.
pub fn cleanup_light_sources(
    mut commands: Commands,
    q_light_sources: Query<Entity, With<LightRaySource>>,
    segment_cache: Res<LightSegmentCache>,
    mut q_segments: Query<(&mut Transform, &mut Visibility), With<LightSegmentMarker>>,
) {
    // FIXME: should make these entities children of the level so that they are despawned
    // automagically (?)

    for entity in q_light_sources.iter() {
        commands.entity(entity).despawn_recursive();
    }

    segment_cache.table.iter().for_each(|(_, items)| {
        for &entity in items.iter() {
            let (mut transform, mut visibility) = q_segments
                .get_mut(entity)
                .expect("Segment should have visibility");

            *transform = Transform::default();
            *visibility = Visibility::Hidden;
        }
    });
}
