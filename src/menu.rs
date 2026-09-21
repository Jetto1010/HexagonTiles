use::bevy::prelude::*;
use bevy::text::{EditableText};
use bevy::input_focus::{AutoFocus, InputFocus};

use super::GameState;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

// https://bevy.org/examples/games/game-menu/
pub fn menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Menu), main_menu_setup)
        .add_systems(Update, (click_buttons, input_seed).run_if(in_state(GameState::Menu)));
}


#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit
}

fn main_menu_setup(
    mut commands: Commands
){
    let default_node = Node {
        width: px(500),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_font = TextFont {
        font_size: FontSize::Px(33.0),
        ..default()
    };

    commands.spawn((
            DespawnOnExit(GameState::Menu),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    width: percent(100),
                    height: percent(100),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.6, 0.0, 0.0)),
                children![(
                    Text::new("Hexagon Tiling"),
                    TextFont {
                        font_size: FontSize::Px(67.0),
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(px(50)),
                        ..default()
                    },
                ),(
                    Button,
                    default_node.clone(),
                    BackgroundColor(BUTTON_COLOR),
                    MenuButtonAction::Play,
                    children![(
                        Text::new("New Game"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    )]
                ),(
                    BackgroundColor(BUTTON_COLOR),
                    default_node.clone(),
                    children![(
                        Text::new("Seed:"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::px(50.0, 0.0, 50.0, 50.0),
                            width: percent(20),
                            ..default()
                        }
                    ),(
                       Text::new("3453453435"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                        EditableText {
                            max_characters: Some(16),
                            allow_newlines: false,
                            ..default()
                        },
                        TextLayout::no_wrap(),
                        Node {
                            margin: UiRect::px(50.0, 0.0, 50.0, 50.0),
                            width: percent(80),
                            ..default()
                        },
                        AutoFocus,
                    )],
                ),(
                    Button,
                    default_node,
                    BackgroundColor(BUTTON_COLOR),
                    MenuButtonAction::Quit,
                    children![(
                        Text::new("Quit"),
                        button_text_font,
                        TextColor(TEXT_COLOR),
                    )]
                )]
            )]
    ));
}

fn click_buttons(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>
) {
    for (interaction, menu_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                MenuButtonAction::Quit => {
                    app_exit_writer.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                }
            }
        }
    }
}

fn input_seed(
    input_focus: Res<InputFocus>,
    button_input: Res<ButtonInput<KeyCode>>,
    mut input: Query<&mut EditableText>,
){
    if button_input.just_pressed(KeyCode::Enter)
        && let Some(entity) = input_focus.get()
        && let Ok(input) = input.get_mut(entity)
    {
        let text: String = input.value().to_string();
        println!("Submitted: {text}");
    }
}

