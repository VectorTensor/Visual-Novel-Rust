use bevy::prelude::*;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_game);
    }
}

#[derive(Component)]
struct GameData;

fn setup_game(mut commands: Commands) {
    commands.spawn((Camera2d::default(), GameData));
    
    commands.spawn((
        Text::new("Game Scene"),
        TextFont {
            font_size: 60.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        GameData,
    ));
}
