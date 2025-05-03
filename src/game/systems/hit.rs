use crate::game::components::{RogueHitTarget, MonsterHealth, MonsterType, RogueEquipped, WeaponType};
use bevy::prelude::*;

pub fn handle_rogue_hit(
    mut commands: Commands,
    query_monster: Single<(Entity, &mut MonsterHealth), With<RogueHitTarget>>,
    query_weapon: Option<Single<&WeaponType, With<RogueEquipped>>>,
) {
    let (monster_entity, mut monster_health) = query_monster.into_inner();
    let weapon_type = query_weapon.map(|query| query.into_inner());
    let damage = weapon_damage(weapon_type);

    monster_health.hp_to_kill -= damage;
    println!("you hit");
    commands.entity(monster_entity).remove::<RogueHitTarget>();
}

pub fn handle_defeated_monsters(
    mut commands: Commands,
    query_monsters: Query<(Entity, &MonsterType, &MonsterHealth)>,
) {
    for (entity, monster_type, monster_health) in query_monsters.iter() {
        if monster_health.hp_to_kill <= 0 {
            println!("defeated the {}", monster_type.name());
            commands.entity(entity).despawn();
        }
    }
}

fn weapon_damage(weapon_type: Option<&WeaponType>) -> isize {
    match weapon_type {
        None => -1,
        Some(weapon_type) => {
            let (number, damage) = weapon_type.damage();
            (number * damage) as isize
        }
    }
}
