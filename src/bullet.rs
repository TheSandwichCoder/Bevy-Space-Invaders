use bevy::prelude::*;
use crate::player::Player;
use crate::enemy::Enemy;
use crate::collision_shape::*;
use crate::gamestate::GameState;

pub struct BulletPlugin;


impl Plugin for BulletPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_bullet_parent)
            .add_systems(Update, ((spawn_bullet, bullet_move, bullet_lifetime, bullet_collision_enemy).run_if(in_state(GameState::Playing))))
            .add_systems(OnEnter(GameState::Playing), reset)
            .register_type::<Bullet>();
    }
}

#[derive(Component, Default, Reflect)]
pub struct Bullet{
    pub lifetime: Timer,
    pub speed: f32,
    pub hitbox: CircleCollider,
}

#[derive(Component)]
pub struct BulletParent;

fn spawn_bullet_parent(mut commands: Commands){
    commands.spawn((SpatialBundle::default(), BulletParent, Name::new("Bullet Parent")));
}

fn spawn_bullet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<BulletParent>>,
){
    if !input.just_pressed(KeyCode::Space){
        return;
    }

    let player_transform = player.single();
    let parent = parent.single();

    let bullet_sprite = asset_server.load("bulletSprite.png");
    commands.entity(parent).with_children(|commands|{
        commands.spawn((
            SpriteBundle{
                texture:bullet_sprite,
                transform: *player_transform,
                ..default() 
            },
            Bullet{
                lifetime: Timer::from_seconds(2.0, TimerMode::Once),
                speed: 400.0,
                hitbox: create_circle_collider(Vec3::new(player_transform.translation.x, player_transform.translation.y, 0.0), 20.0),
            },
            Name::new("Bullet"),
        ));
    });
}

fn bullet_move(
    time: Res<Time>,
    mut bullets: Query<(&mut Transform, &mut Bullet)>,
){
    for (mut transform, mut bullet) in &mut bullets{
        bullet.speed += 10.0;
        transform.translation.y += bullet.speed * time.delta_seconds();
        update_circle_collider(&mut bullet.hitbox, Vec3::new(transform.translation.x, transform.translation.y, 1.0))
    }
}

fn remove_bullet(
    commands: &mut Commands,
    parent: Entity,
    bullet_entity: Entity, 
){
    commands.entity(parent).remove_children(&[bullet_entity]);
    commands.entity(bullet_entity).despawn();

    info!("Removed Bullet");
}

fn bullet_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Bullet)>,
    parent: Query<Entity, With<BulletParent>>,
){
    let parent = parent.single();

    for (bullet_entity, mut bullet) in &mut bullets{        
        bullet.lifetime.tick(time.delta());

        if bullet.lifetime.finished(){
            remove_bullet(&mut commands, parent, bullet_entity);
        }
        
    }
}

fn bullet_collision_enemy(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Bullet)>,
    mut enemies: Query<&mut Enemy>,
    parent: Query<Entity, With<BulletParent>>
){
    let parent = parent.single();

    for (mut bullet_entity, bullet) in &mut bullets{
        for mut enemy in &mut enemies{
            if is_circle_collision(&enemy.hitbox, &bullet.hitbox){
                info!("Bullet collided with enemy");
                remove_bullet(&mut commands, parent, bullet_entity);
                enemy.health -= 10.0;
                break;
            }
        }
    }
}

fn reset(
    mut commands: Commands,
    bullets: Query<Entity, With<Bullet>>,
    parent: Query<Entity, With<BulletParent>>,
){
    if parent.iter().count() > 0{
        let parent = parent.single();

        for bullet_entity in &bullets{
            remove_bullet(&mut commands, parent, bullet_entity);
        }
    }
}