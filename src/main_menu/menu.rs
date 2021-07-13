use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    math::Rect,
    prelude::{
        AssetServer, Bundle, Color, Commands, EventReader, KeyCode, Res, ResMut, State, TextBundle,
    },
    text::{Text, TextSection, TextStyle},
    ui::{PositionType, Style, Val},
};

use crate::AppState;

const FONT_PATH: &str = "fonts/FiraSans-Bold.ttf";
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const FONT_SIZE: f32 = 40.0;
const PADDING: Val = Val::Px(5.0);

#[derive(Bundle)]
struct MenuBundle {
    #[bundle]
    title_bundle: TextBundle,
}

impl MenuBundle {
    fn new(asset_server: Res<AssetServer>) -> Self {
        let title_text = Text {
            sections: vec![TextSection {
                value: "Arguably Better Breakout!\n>> Press Enter to play <<".to_string(),
                style: TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                },
            }],
            ..Default::default()
        };
        let title_style = Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: PADDING,
                left: PADDING,
                ..Default::default()
            },
            ..Default::default()
        };

        Self {
            title_bundle: TextBundle {
                text: title_text,
                style: title_style,
                ..Default::default()
            },
        }
    }
}

pub fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(MenuBundle::new(asset_server));
}

pub fn start_game_on_enter(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut app_state: ResMut<State<AppState>>,
) {
    for event in keyboard_input_events.iter() {
        if let Some(key_code) = event.key_code {
            if event.state == ElementState::Pressed && key_code == KeyCode::Return {
                app_state.push(AppState::InGame).unwrap();
            }
        }
    }
}
