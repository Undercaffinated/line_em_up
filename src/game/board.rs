use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::pieces::PieceColors;

#[derive(Component)]
pub struct Board {
    play_area: [Option<PieceColors>; 42],
}

pub fn spawn_board(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/Board.png"), // Escape backslash bc Windows is trash.
            ..default()
        },
        Board {
            play_area: [None; 42],
        },
    ));
}
