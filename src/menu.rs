use bevy::prelude::*;
use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_menu)
           .add_systems(Update, menu_action.run_if(in_state(AppState::MainMenu)))
           .add_systems(OnExit(AppState::MainMenu), cleanup_menu);
    }
}

#[derive(Component)]
struct MenuData;

#[derive(Component)]
struct PlayButton;

fn setup_menu(mut commands: Commands) {
    // Camera
    commands.spawn((Camera2d::default(), MenuData));

    // UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            MenuData,
        ))
        .with_children(|parent| {
            parent
                .spawn((
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
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &BackgroundColor),
        (Changed<Interaction>, With<Button>, With<PlayButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, _color) in &interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(AppState::InGame);
            }
            _ => {}
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuData>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
