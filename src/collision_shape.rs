use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct CircleCollider{
    pub pos: Vec3,
    pub radius: f32,
}

pub fn create_circle_collider(
    pos: Vec3,
    radius: f32
)-> CircleCollider{
    return CircleCollider{pos:pos, radius:radius};
}

pub fn update_circle_collider(colliderObject: &mut CircleCollider, newPos: Vec3){
    colliderObject.pos = newPos;
}

pub fn is_circle_collision(
    colliderObject1: &CircleCollider, 
    colliderObject2: &CircleCollider
) ->bool {
    let distanceVec:Vec3 = colliderObject1.pos - colliderObject2.pos;
    let radiusSum: f32 = colliderObject1.radius+colliderObject2.radius;
    
    return distanceVec.length_squared() <= radiusSum*radiusSum;
}