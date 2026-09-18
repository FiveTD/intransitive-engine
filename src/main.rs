mod components;
mod constants;
mod events;
mod model;
mod resources;
mod systems;

use bevy::log;
use bevy::prelude::*;

use crate::constants::*;
use crate::events::*;
use crate::resources::*;
use crate::systems::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Intransitive".into(),
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            })
            .set(log::LogPlugin {
                level: log::Level::WARN,
                filter: "intransitive_engine=debug".to_string(),
                ..default()
            }),
    )
    .insert_resource(ClearColor(BG_COLOR))
    .insert_resource(GameBoard::default())
    .insert_resource(PieceEntities::default())
    .insert_resource(SelectedTile::default());

    app.add_systems(
        Startup,
        (
            render::spawn_camera,
            game::setup_board.before(render::spawn_board),
            render::spawn_board,
        ),
    )
    .add_systems(Update, (input::handle_mouse))
    .add_observer(render::animate_piece)
    .add_observer(game::handle_move_piece);

    app.run();
}
