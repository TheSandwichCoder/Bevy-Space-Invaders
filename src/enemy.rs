use bevy::prelude::*;
use rand::Rng;
use crate::collision_shape::*;
use crate::gamestate::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app:&mut App){
        app.add_systems(Startup,spawn_enemy_parent)
            .add_systems(OnEnter(GameState::Playing), reset)
            .add_systems(Update, ((spawn_enemy, update_enemy_pos, remove_dead_enemies, end_game).run_if(in_state(GameState::Playing))))
            .register_type::<Enemy>();
    }
}

#[derive(Component, Default, Reflect)]
pub struct EnemyParent{
    pub spawn_timer: Timer,
}

#[derive(Component, Default, Reflect)]
pub struct Enemy{
    pub health: f32,
    pub speed: f32,
    pub movement_direction: bool,
    pub hitbox: CircleCollider,
}

fn spawn_enemy_parent(mut commands:Commands){
    commands.spawn((
        SpatialBundle::default(), 
        EnemyParent{
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }, 
        Name::new("Enemy Parent")
    ));
}

fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut parent: Query<(Entity, &mut EnemyParent)>,
    time: Res<Time>,
){
    let (parent, mut parentStruct) = parent.single_mut();
    parentStruct.spawn_timer.tick(time.delta());

    if parentStruct.spawn_timer.finished(){
        let enemy_sprite = asset_server.load("enemySprite.png");

        let random_size:f32 = (rand::thread_rng().gen_range(1..=5) as f32) / 5.0 + 1.0;
        let speed_relation_component = 1.0 - (1.0-random_size)*(1.0-random_size);
        let random_speed:f32 = speed_relation_component * 3.0 + 1.0;
        let direction: bool = rand::thread_rng().gen_range(0..=1) == 1;
        // let random_speed:f32 = rand::thread_rng().gen_range(1..3=) as f32;

        commands.entity(parent).with_children(|commands|{
            commands.spawn((
                SpriteBundle{
                    texture: enemy_sprite,
                    transform: Transform {
                        scale: Vec3::new(2.5*random_size, 2.5*random_size, 1.0),  // Resize the image by scaling it
                        translation: Vec3::new(0.0,180.0,0.0),
                        ..default()
                    },
                    ..default()
                },
                Enemy{
                    health: 50.0 * random_size * random_size,
                    speed: 200.0 * random_speed,
                    movement_direction: direction,
                    hitbox: create_circle_collider(Vec3::new(0.0, 180.0, 0.0), 70.0*random_size),
                },
                Name::new("Enemy"),
            ));
        });
        info!("Spawned New Enemy");
    }
}

fn update_enemy_pos(
    mut enemies: Query<(&mut Transform, &mut Enemy)>,
    time: Res<Time>,
){
    for (mut transform, mut enemy) in &mut enemies{
        let movement_amount;
        if enemy.movement_direction{
            movement_amount = enemy.speed * time.delta_seconds();
        }
        else{
            movement_amount = -enemy.speed * time.delta_seconds();
        }

        transform.translation.x += movement_amount;

        if transform.translation.x > 320.0{
            enemy.movement_direction = false;
            transform.translation.y -= 50.0;
        }
        else if transform.translation.x < -320.0{
            enemy.movement_direction = true;
            transform.translation.y -= 50.0;
        }

        update_circle_collider(&mut enemy.hitbox, Vec3::new(transform.translation.x, transform.translation.y, 1.0));
    }
}

fn remove_enemy(
    commands: &mut Commands,
    parent: Entity,
    enemy_entity: Entity, 
){
    commands.entity(parent).remove_children(&[enemy_entity]);
    commands.entity(enemy_entity).despawn();

    info!("Removed Enemy");
}

fn remove_dead_enemies(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Enemy)>,
    parent: Query<Entity, With<EnemyParent>>,
){
    let parent = parent.single();

    for (enemy_entity,mut enemy) in &mut enemies{
        if enemy.health <= 0.0{
            remove_enemy(&mut commands, parent, enemy_entity);
            unsafe{
                current_score += 100;
            }
        }
    }
}

fn end_game(
    enemies: Query<&Transform, With<Enemy>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>
){
    for transform in &enemies{
        if transform.translation.y < -150.0{
            next_state.set(GameState::GameOver);
            break;
        }
    } 
}

fn reset(
    mut commands: Commands,
    enemies: Query<Entity, With<Enemy>>,
    parent: Query<Entity, With<EnemyParent>>,
){
    if parent.iter().count() > 0{
        let parent = parent.single();

        for enemy_entity in &enemies{
            remove_enemy(&mut commands, parent, enemy_entity);
        }
    }
}