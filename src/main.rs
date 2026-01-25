use bevy::prelude::*;

mod menu;
mod game;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(menu::MenuPlugin)
        .add_plugins(game::GamePlugin)
        .run();
}

