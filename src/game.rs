use bevy::prelude::*;
use crate::AppState;
use crate::utils::GameButtonAction;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App)
    {
        app.add_systems(OnEnter(AppState::InGame), setup_game)
            .add_systems(Update, game_ui_action.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::InGame), cleanup_menu);

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

    let buttons =[
        ("Back", GameButtonAction::Back),
    ];
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::Start,
            row_gap: Val::Px(10.0),
            ..default()
        },
        GameData

    ))
        .with_children(|parent| {
            for (text, action) in buttons{
                parent.spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor::from(Color::srgb(0.15, 0.15, 0.15)),
                    BorderColor::from(Color::WHITE),
                    action,
                ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new(text),
                            TextFont {
                                font_size: 40.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });


            }
        });


}

fn game_ui_action(
    interaction_query: Query<(&Interaction, &GameButtonAction), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<AppState>>,
){
    for (interaction, action) in &interaction_query{
        match *interaction{
            Interaction::Pressed => match action{
                GameButtonAction::Back => {
                    next_state.set(AppState::MainMenu);
                }
            },
            _ => {}
        }
    }

}
fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<crate::game::GameData>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
