use bevy::prelude::Component;

#[derive(Component, Clone, Copy)]
pub enum MenuButtonAction {
    Play,
    Quit,
}

#[derive(Component, Clone, Copy)]
pub enum GameButtonAction {
    Back,
}


/// Macro to spawn a button with consistent styling
#[macro_export]
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
#[macro_export]
macro_rules! simple_box {
    ($parent:expr, $width:expr, $height:expr, $text:expr) => {
        {
            let mut box_entity = $parent.spawn((
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

            if let Some(text) = $text {
                box_entity.with_children(|text_parent| {
                    text_parent.spawn((
                        Text::new(text),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),

                    ));
                });
            }

        }

    };
}
