use bevy::prelude::*;

use super::PlayerMarker;

/// [`Component`] that offsets positions to make them match the Player's
/// pixel grid. Only offsets the position PostUpdate to change visuals, and resets it in PreUpdate of the next frame to
/// prevent interference with game logic.
#[derive(Component)]
#[require(Transform)]
pub struct MatchPlayerPixel(pub Transform);

/// [`System`] that resets entities' transforms from the player-pixel-matching transform to the actual transform.
pub fn pre_update_match_player_pixel(mut query: Query<(&mut Transform, &MatchPlayerPixel)>) {
    for (mut transform, actual_transform) in query.iter_mut() {
        *transform = actual_transform.0;
    }
}
/// [`System`] that offsets entities' positions to match the player's pixel grid,
///  and stores the actual transform in the [`MatchPlayerPixel`] component.
pub fn post_update_match_player_pixel(
    mut query: Query<(&mut Transform, &mut MatchPlayerPixel)>,
    player_transform: Query<&Transform, (With<PlayerMarker>, Without<MatchPlayerPixel>)>,
) {
    let Ok(player_transform) = player_transform.get_single() else {
        return;
    };
    for (mut transform, mut actual_transform) in query.iter_mut() {
        actual_transform.0 = *transform;
        transform.translation.x = match_pixel(
            player_transform.translation.x,
            actual_transform.0.translation.x,
        );
        transform.translation.y = match_pixel(
            player_transform.translation.y,
            actual_transform.0.translation.y,
        );
    }
}
fn match_pixel(source: f32, actual: f32) -> f32 {
    let match_pixel_1 = actual.trunc() + source.fract();
    let match_pixel_2 = actual.trunc() + 1.0 + source.fract();
    let match_pixel_3 = actual.trunc() + -1.0 + source.fract();
    if (actual - match_pixel_1).abs() < (actual - match_pixel_2).abs()
        && (actual - match_pixel_1).abs() < (actual - match_pixel_3).abs()
    {
        match_pixel_1
    } else if (actual - match_pixel_2).abs() < (actual - match_pixel_3).abs() {
        match_pixel_2
    } else {
        match_pixel_3
    }
}

/// [`Component`] that sets Entity's `z` to the player's `z` plus `offset`
#[derive(Component)]
pub struct MatchPlayerZ {
    pub offset: f32,
}

pub fn update_match_player_z(
    mut query: Query<(&mut Transform, &MatchPlayerZ)>,
    player_transform: Query<&Transform, (With<PlayerMarker>, Without<MatchPlayerZ>)>,
) {
    let Ok(player_transform) = player_transform.get_single() else {
        return;
    };
    for (mut transform, MatchPlayerZ { offset }) in query.iter_mut() {
        transform.translation.z = player_transform.translation.z + offset;
    }
}
