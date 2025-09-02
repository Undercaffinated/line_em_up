use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::Players;
use crate::TurnPlayer;

#[derive(Component)]
pub struct Piece {}

pub fn spawn_piece(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    turn_player: Res<TurnPlayer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height(), 0.0),
            texture: match turn_player.0 {
                // Note that Player1 always plays Red, P2 -> Black.
                Players::Player1 => asset_server.load("sprites/RedChecker.png"),
                Players::Player2 => asset_server.load("sprites/BlackChecker.png"),
            },
            ..default()
        },
        Piece {},
    ));
}

#[derive(Debug, Copy, Clone)]
pub enum PieceColors {
    Red,
    Black,
}
