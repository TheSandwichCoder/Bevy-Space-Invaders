use bevy::prelude::*;
use crate::gamestate::GameState;
pub struct PlayerPlugin;


impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, pause_game))
            .register_type::<Player>();
    }
}

#[derive(Component, Default, Reflect)]
pub struct Player{
    pub speed: f32,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>){
    let texture = asset_server.load("playerSprite.png");

    commands.spawn((
        SpriteBundle{
            texture,
            transform: Transform {
                scale: Vec3::new(5.0, 5.0, 1.0),  // Resize the image by scaling it
                translation: Vec3::new(0.0,-180.0,0.0),
                ..default()
            },
            ..default()
        },
        Player {speed:400.0},
        Name::new("Player"),
    ));
}

fn player_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game_state: Res<State<GameState>>
){
    match game_state.get() {
        GameState::Playing => {
            for (mut transform, player) in &mut characters{
                let movement_amount = player.speed * time.delta_seconds();
        
                if input.pressed(KeyCode::KeyD){
                    transform.translation.x += movement_amount;
                }
                if input.pressed(KeyCode::KeyA){
                    transform.translation.x -= movement_amount;
                }
            }
        },
        _ => {},
    } 
    
}

fn pause_game(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
){
    if input.just_pressed(KeyCode::Enter) {
        next_state.set(match current_state.get() {
            GameState::Playing => GameState::GameOver,
            GameState::GameOver => GameState::Playing,
        });
    }
}