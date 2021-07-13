use bevy::{
    math::Rect,
    prelude::{AssetServer, Bundle, Color, Commands, Query, Res, TextBundle, With},
    text::{Text, TextSection, TextStyle},
    ui::{PositionType, Style, Val},
};

const SCOREBOARD_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const SCORE_FONT_SIZE: f32 = 40.0;
const SCORE_PADDING: Val = Val::Px(5.0);
const SCOREBOARD_FONT_PATH: &str = "fonts/FiraSans-Bold.ttf";
const SCORE_FONT_PATH: &str = "fonts/FiraSans-Bold.ttf";

#[derive(Default)]
pub struct Score(pub usize);

pub struct Scoreboard;

#[derive(Bundle)]
struct ScoreboardBundle {
    #[bundle]
    text_bundle: TextBundle,
    scoreboard: Scoreboard,
}

impl ScoreboardBundle {
    fn new(asset_server: Res<AssetServer>) -> Self {
        let text = Text {
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
        };
        let style = Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: SCORE_PADDING,
                left: SCORE_PADDING,
                ..Default::default()
            },
            ..Default::default()
        };

        Self {
            text_bundle: TextBundle {
                text,
                style,
                ..Default::default()
            },
            scoreboard: Scoreboard,
        }
    }
}

pub fn init_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn spawn_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(ScoreboardBundle::new(asset_server));
}

/// Updates the Scoreboard entity's Text based on the value of the Score resource
pub fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<Scoreboard>>) {
    let mut scoreboard_text = query.single_mut().unwrap();
    // We need to access the second section, so we need to access the sections field at the [1] index
    // (Rust is 0-indexed: https://medium.com/analytics-vidhya/array-indexing-0-based-or-1-based-dd89d631d11c)
    scoreboard_text.sections[1].value = format!("{}", score.0);
}
