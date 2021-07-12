use bevy::{
    math::Rect,
    prelude::{AssetServer, Commands, Query, Res, TextBundle, With},
    text::{Text, TextSection, TextStyle},
    ui::{PositionType, Style},
};

use crate::config::*;

#[derive(Default)]
pub struct Score(pub usize);

pub struct Scoreboard;

pub fn spawn_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load(SCOREBOARD_FONT_PATH),
                            font_size: SCORE_FONT_SIZE,
                            color: SCOREBOARD_COLOR,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load(SCORE_FONT_PATH),
                            font_size: SCORE_FONT_SIZE,
                            color: SCORE_COLOR,
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: SCORE_PADDING,
                    left: SCORE_PADDING,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Scoreboard);
}

/// Updates the Scoreboard entity's Text based on the value of the Score resource
pub fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<Scoreboard>>) {
    let mut scoreboard_text = query.single_mut().unwrap();
    // We need to access the second section, so we need to access the sections field at the [1] index
    // (Rust is 0-indexed: https://medium.com/analytics-vidhya/array-indexing-0-based-or-1-based-dd89d631d11c)
    scoreboard_text.sections[1].value = format!("{}", score.0);
}
