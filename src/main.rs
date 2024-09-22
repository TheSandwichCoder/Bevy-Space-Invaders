use bevy::{
    color::palettes::css::GOLD,
    prelude::*,
};

use player::PlayerPlugin;
use bullet::BulletPlugin;
use enemy::EnemyPlugin;
use gamestate::*;

// CRATES
mod player;
mod bullet;
mod enemy;
mod collision_shape;
mod gamestate;

#[derive(Component)]
struct scoreText;

#[derive(Component)]
struct replayText;

// MAIN CODE
fn main() {
    App::new()
    .add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window{
                    title: "Space Invaders".into(),
                    resolution: (640.0, 480.0).into(),
                    resizable:false,
                    ..default()
                }),
                ..default()
            })
            .build(),
    )
    .init_state::<GameState>()
    .add_systems(Startup, setup)
    .add_plugins(PlayerPlugin)
    .add_plugins(BulletPlugin)
    .add_plugins(EnemyPlugin)
    .add_systems(OnEnter(GameState::GameOver), show_game_over_screen_system)
    .add_systems(OnExit(GameState::GameOver), remove_game_over_screen_system)
    .add_systems(Update, update_score)
    
    
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    let camera = Camera2dBundle::default();

    commands.spawn(camera);

    let texture_handle = asset_server.load("background.png");

    // Spawn the background image as a sprite
    commands.spawn(SpriteBundle {
        texture: texture_handle,
        transform: Transform {
            scale: Vec3::new(18.8, 20.0, 1.0), // Set the scale to match your window size
            translation: Vec3::new(0.0,0.0,-1.0),
            ..default()
        },
        ..default()
    }); 

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ),
            TextSection::from_style(
                TextStyle {
                    font_size: 30.0,
                    color: GOLD.into(),
                    ..default()
                }
            ),
        ]),
        scoreText,
    ));

}

fn show_game_over_screen_system(
    mut commands: Commands,
    game_state: Res<State<GameState>>
) {
    unsafe{
        if current_score > best_score{
            best_score = current_score;
        }

        commands.spawn((
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                ("Press [Enter] to Play Again \n Best Score:".to_owned() + &best_score.to_string()),
                TextStyle {
                    font_size: 32.0,
                    ..default()
                },
            ) // Set the justification of the Text
            .with_text_justify(JustifyText::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(200.0),
                right: Val::Px(120.0),
                ..default()
            }),
            replayText
        ));
    }
    
}

fn remove_game_over_screen_system(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    replay_text: Query<Entity, With<replayText>>
){
    if replay_text.iter().count() > 0{
        let replay_text = replay_text.single();
        commands.entity(replay_text).despawn();
    }

    unsafe{
        current_score = 0;
    }
    
}

fn update_score(
    mut text: Query<&mut Text, With<scoreText>>,
){
    let mut text = text.single_mut();
    unsafe{
        text.sections[1].value = format!("{}", current_score);
    }
    
}