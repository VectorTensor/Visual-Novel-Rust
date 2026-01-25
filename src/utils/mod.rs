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
