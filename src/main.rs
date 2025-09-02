use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod game;
mod main_menu;

use game::board::*;
use game::game_logic::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(TurnPlayer(Players::Player1))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_board)
        .add_systems(Startup, whose_turn_is_it)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

#[derive(Debug, Clone, Copy, Resource)]
pub struct TurnPlayer(game::game_logic::Players);

pub fn whose_turn_is_it(tp: Res<TurnPlayer>) {
    println!("{:?}", tp);
}
