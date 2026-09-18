#![allow(clippy::unreachable)]

use bevy::prelude::*;

use crate::{constants::*, events::MovePiece, model::board::*, resources::*};

struct PieceMeshes {
    rock: Handle<Mesh>,
    paper: Handle<Mesh>,
    scissors: Handle<Mesh>,
    capture: Handle<Mesh>,
}

struct PlayerMaterials {
    piece: Handle<ColorMaterial>,
    capture: Handle<ColorMaterial>,
}

fn tile_coords_from_pos(pos: (usize, usize), board_size: usize) -> Vec2 {
    let center_offset = BOARD_TILE_SIZE * ((board_size - 1) as f32 / 2.);
    Vec2::new(
        BOARD_TILE_SIZE * pos.0 as f32 - center_offset,
        center_offset - BOARD_TILE_SIZE * pos.1 as f32, // put 0,0 in top left corner
    )
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Res<GameBoard>,
    mut piece_entities: ResMut<PieceEntities>,
) {
    let size = board.0.size();
    debug!("Rendering board size: {size}");

    let tile_border_mesh =
        meshes.add(Rectangle::new(BOARD_TILE_SIZE, BOARD_TILE_SIZE).to_ring(BOARD_LINE_THICKNESS));
    let tile_border_material = materials.add(Color::BLACK);

    let piece_meshes = PieceMeshes {
        rock: meshes.add(Circle::new(BOARD_PIECE_SIZE / 2.).to_ring(BOARD_LINE_THICKNESS)),
        paper: meshes
            .add(Rectangle::new(BOARD_PIECE_SIZE, BOARD_PIECE_SIZE).to_ring(BOARD_LINE_THICKNESS)),
        scissors: meshes
            .add(RegularPolygon::new(BOARD_PIECE_SIZE / 2., 6).to_ring(BOARD_LINE_THICKNESS)),
        capture: meshes.add(Rectangle::new(BOARD_TILE_SIZE, BOARD_TILE_SIZE)),
    };

    let player_materials: [PlayerMaterials; PLAYER_COLORS.len()] = std::array::from_fn(|i| {
        let c = PLAYER_COLORS[i];
        PlayerMaterials {
            piece: materials.add(c),
            capture: materials.add(c.with_alpha(CAPTURE_ALPHA)),
        }
    });

    for y in 0..size {
        for x in 0..size {
            let tile_coords = tile_coords_from_pos((x, y), size);
            if let Some(tile) = board.0[y][x] {
                let color_index = usize::from(tile.owner);
                match tile.piece_type {
                    PieceType::Capture => {
                        commands.spawn((
                            Mesh2d(piece_meshes.capture.clone()),
                            MeshMaterial2d(player_materials[color_index].capture.clone()),
                            Transform::from_translation(tile_coords.extend(0.0)),
                        ));
                    }
                    piece_type => {
                        let mesh = match piece_type {
                            PieceType::Rock => piece_meshes.rock.clone(),
                            PieceType::Paper => piece_meshes.paper.clone(),
                            PieceType::Scissors => piece_meshes.scissors.clone(),
                            PieceType::Capture => unreachable!(),
                        };
                        let piece_entity = commands
                            .spawn((
                                Mesh2d(mesh),
                                MeshMaterial2d(player_materials[color_index].piece.clone()),
                                Transform::from_translation(tile_coords.extend(2.0)),
                            ))
                            .id();
                        piece_entities.0.insert((x, y), piece_entity);
                    }
                }
            }
            commands.spawn((
                Mesh2d(tile_border_mesh.clone()),
                MeshMaterial2d(tile_border_material.clone()),
                Transform::from_translation(tile_coords.extend(1.0)),
            ));
        }
    }
}

pub fn animate_piece(
    move_piece: On<MovePiece>,
    mut pieces: ResMut<PieceEntities>,
    mut piece_q: Query<&mut Transform>,
    board: Res<GameBoard>,
    mut commands: Commands,
) {
    if let Some(from_entity) = pieces.0.remove(&move_piece.from) {
        if let Some(to_entity) = pieces.0.get(&move_piece.to) {
            commands.entity(*to_entity).despawn();
            // TODO: destruction animation
        }
        pieces.0.insert(move_piece.to, from_entity);

        if let Ok(mut transform) = piece_q.get_mut(from_entity) {
            let tile_coords = tile_coords_from_pos(move_piece.to, board.0.size());
            transform.translation = tile_coords.extend(2.);
            // TODO: movement animation
        }
    }
}
