use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{constants::BOARD_TILE_SIZE, events::MovePiece, resources::*};

pub fn handle_mouse(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut selected_tile: ResMut<SelectedTile>,
    board: Res<GameBoard>,
) {
    if !buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        return;
    }

    // Right click to cancel
    if buttons.just_pressed(MouseButton::Right) {
        selected_tile.0 = None;
        return;
    }

    let Ok(window) = windows.single() else { return };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    // Convert to board position
    let world_position = camera
        .viewport_to_world_2d(camera_transform, cursor_position)
        .unwrap();
    let pos: (usize, usize) = (world_position / BOARD_TILE_SIZE)
        .round()
        .as_ivec2()
        .saturating_mul(ivec2(1, -1))
        .saturating_add(ivec2(4, 4))
        .as_usizevec2()
        .into();

    if let Some(selected) = selected_tile.0 {
        if board.0.is_valid_move(selected, pos) {
            selected_tile.0 = None;
            commands.trigger(MovePiece {
                from: selected,
                to: pos,
            });
            return;
        } else if pos == selected {
            selected_tile.0 = None;
            return;
        }
    }
    if board.0[pos].is_some_and(|p| p.owner == board.0.active_turn()) {
        selected_tile.0 = Some(pos);
    }
}
