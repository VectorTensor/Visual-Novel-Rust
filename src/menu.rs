use bevy::prelude::*;
use crate::{spawn_button, AppState};
use crate::utils::{MenuButtonAction};

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


fn setup_menu(mut commands: Commands) {
    // Camera
    commands.spawn((Camera2d::default(), MenuData));

    let buttons = [
        ("Play", MenuButtonAction::Play),
        ("Quit", MenuButtonAction::Quit),
    ];

    // UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(10.0),
                ..default()
            },
            MenuData,
        ))
        .with_children(|parent| {
            for (text, action) in buttons {
                spawn_button!(parent, Val::Px(150.0), Val::Px(65.0), Some(text), Some(action))
            }
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, action) in &interaction_query {
        match *interaction {
            Interaction::Pressed => match action {
                MenuButtonAction::Play => {
                    next_state.set(AppState::InGame);
                }
                MenuButtonAction::Quit => {
                    // TODO: Implement proper AppExit when possible
                    println!("Quit pressed");
                }
            },
            _ => {}
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuData>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
