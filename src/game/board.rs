use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::game_logic::*;
use crate::TurnPlayer;
use crate::game::board;
use crate::game::pieces::PieceColors;

#[derive(Component)]
pub struct Board {
    play_area: [Option<PieceColors>; 42],
}

pub fn print_board(board_query: Query<&mut Board>) {
    let board: &Board = board_query.get_single().unwrap();
    let mut output_array = ["N"; 42];

    for cell in 0..board.play_area.len() {
        output_array[cell] = match board.play_area[cell] {
            Some(PieceColors::Red) => "R",
            Some(PieceColors::Black) => "B",
            None => "N",
        };
    }

    let mut left_bound: usize = 35;
    let mut right_bound: usize = 41;

    for i in 0..6 {
        for j in left_bound..=right_bound {
            print!("{}", output_array[j]);
        }
        print!("\n");

        // Since the print function prints from the top row down,
        // these definitions ensure we stay in range since i is at most 5.
        left_bound = 35 - 7 * i;
        right_bound = 41 - 7 * i;
    }
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

pub enum UpdateCellError {
    IndexOutofRange,
    CellAlreadyOccupied,
}

pub fn update_board_cell(
    mut board_query: Query<&mut Board>,
    turn_player: Res<TurnPlayer>,
    index: usize,
) -> Result<(), UpdateCellError> {
    // The highest index on the board is 41.
    if index > 41 {
        return Err(UpdateCellError::IndexOutofRange);
    }

    let board: &mut Board = &mut board_query.get_single_mut().unwrap();

    if board.play_area[index] != None {
        return Err(UpdateCellError::CellAlreadyOccupied);
    }

    match turn_player.0 {
        Players::Player1 => {
            board.play_area[index] = Some(PieceColors::Red);
            Ok(())
        }

        Players::Player2 => {
            board.play_area[index] = Some(PieceColors::Black);
            Ok(())
        }
    }
}
