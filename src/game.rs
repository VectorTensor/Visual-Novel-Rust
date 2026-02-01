use bevy::prelude::*;
use crate::AppState;
use crate::utils::GameButtonAction;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App)
    {
        app.add_systems(OnEnter(AppState::InGame), (setup_game_menu_ui, setup_gameplay_ui))
            .add_systems(Update, game_ui_action.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::InGame), cleanup_menu);

    }
}

#[derive(Component)]
struct GameData;

/// Macro to spawn a button with consistent styling
macro_rules! spawn_button {
    ($parent:expr, $width:expr, $height:expr, $text:expr, $action:expr) => {
        {
            let mut button_entity = $parent.spawn((
                Button,
                Node {
                    width: $width,
                    height: $height,
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor::from(Color::srgb(0.15, 0.15, 0.15)),
                BorderColor::from(Color::WHITE),
            ));
            
            // Add action if provided
            if let Some(action) = $action {
                button_entity.insert(action);
            }
            
            // Add text if provided
            if let Some(text) = $text {
                button_entity.with_children(|text_parent| {
                    text_parent.spawn((
                        Text::new(text),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            }
        }
    };
}

fn setup_game_menu_ui(mut commands: Commands) {
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
                spawn_button!(parent, Val::Px(150.0), Val::Px(65.0), Some(text), Some(action));
            }
        });


}

fn setup_gameplay_ui(mut commands: Commands){


    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(10.0),
            ..default()
        },
        GameData

    ))
        .with_children(|parent| {
            spawn_button!(parent, Val::Percent(90.0), Val::Percent(25.0), None::<&str>, None::<GameButtonAction>);
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
