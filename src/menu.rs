use::bevy::prelude::*;
use bevy::text::{EditableText, TextEditChange};
use rand::random;
use crate::game::new_game;
use super::GameState;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

// https://bevy.org/examples/games/game-menu/
pub fn menu_plugin(app: &mut App) {
    app
        .insert_resource(Seed(random::<u64>().to_string()))
        .add_systems(OnEnter(GameState::Menu), main_menu_setup)
        .add_systems(Update, (click_buttons, press_enter).run_if(in_state(GameState::Menu)))
        .add_observer(seed_change)
        .add_observer(start_game);
}


#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit
}

#[derive(Resource)]
struct Seed(pub String);

#[derive(Event)]
struct StartGameEvent;

fn main_menu_setup(
    mut commands: Commands,
    seed : Res<Seed>,
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
                                margin: UiRect::px(30.0, 0.0, 50.0, 50.0),
                                width: percent(20),
                                ..default()
                            }
                        ),(
                            EditableText {
                                max_characters: Some(32),
                                visible_width: Some(32.0),
                                ..EditableText::new(&seed.0)
                            },
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                            TextLayout::no_wrap(),
                            Node {
                                margin: UiRect::px(20.0, 0.0, 50.0, 50.0),
                                width: percent(65),
                                ..default()
                            }
                        ),],
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
    mut commands: Commands,
) {
    for (interaction, menu_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                MenuButtonAction::Quit => {
                    app_exit_writer.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    commands.trigger(StartGameEvent);
                }
            }
        }
    }
}

fn seed_change(
    change: On<TextEditChange>,
    q: Query<&EditableText>,
    mut seed: ResMut<Seed>,
) {
    if let Ok(text) = q.get(change.event_target()) {
        seed.0 = text.value().to_string();
        println!("Seed set to {}", seed.0);
    }
}

fn press_enter(
    button_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if button_input.just_pressed(KeyCode::Enter) {
        commands.trigger(StartGameEvent);
    }
}

fn start_game(
    _: On<StartGameEvent>,
    mut commands: Commands,
    seed: Res<Seed>,
    mut game_state: ResMut<NextState<GameState>>
) {
    let new_grid = new_game(seed.0.to_string());
    commands.insert_resource(new_grid);
    game_state.set(GameState::Game);
}