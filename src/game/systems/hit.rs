use crate::game::components::{
    MonsterHealth, MonsterType, RogueBuffs, RogueEquipped, RogueHitTarget, WeaponBuffs, WeaponType,
};
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use rand::Rng;

pub fn handle_rogue_hit(
    mut commands: Commands,
    query_rogue: Single<&RogueBuffs>,
    query_monster: Single<(Entity, &mut MonsterHealth), With<RogueHitTarget>>,
    query_weapon: Option<Single<(&WeaponType, &WeaponBuffs), With<RogueEquipped>>>,
    mut rng: GlobalEntropy<WyRand>,
) {
    let rogue_buffs = query_rogue.into_inner();
    let (monster_entity, mut monster_health) = query_monster.into_inner();
    let weapon_data = query_weapon.map(|query| query.into_inner());

    let hit_chance = hit_chance(weapon_data, rogue_buffs);
    match rng.random_ratio(hit_chance, 100) {
        true => {
            let damage = roll_damage(weapon_data, rogue_buffs, rng.as_mut());
            monster_health.hp_to_kill -= damage;
            if monster_health.hp_to_kill > 0 {
                println!("you hit");
            }
        }
        false => {
            println!("you miss");
        }
    }
    commands.entity(monster_entity).remove::<RogueHitTarget>();
}

fn hit_chance(weapon_data: Option<(&WeaponType, &WeaponBuffs)>, rogue_buffs: &RogueBuffs) -> u32 {
    let mut hit_chance = 40;
    hit_chance += 3 * weapon_hit_chance(weapon_data);
    hit_chance += 2 * rogue_buffs.xp_level_with_ring_boost();
    hit_chance -= rogue_buffs.rings_on_hands;
    hit_chance.max(0) as u32
}

fn weapon_hit_chance(weapon_data: Option<(&WeaponType, &WeaponBuffs)>) -> isize {
    if let Some((weapon_type, weapon_buffs)) = weapon_data {
        weapon_type.damage().0 + weapon_buffs.hit_enchant
    } else {
        1
    }
}

fn roll_damage(
    weapon_data: Option<(&WeaponType, &WeaponBuffs)>,
    rogue_buffs: &RogueBuffs,
    rng: &mut impl Rng,
) -> isize {
    let mut damage = 0;
    damage += weapon_damage(weapon_data, rng);
    damage += rogue_buffs.damage_for_strength();
    damage += (rogue_buffs.xp_level_with_ring_boost() - rogue_buffs.rings_on_hands + 1) / 2;
    damage.max(0)
}
fn weapon_damage(weapon_data: Option<(&WeaponType, &WeaponBuffs)>, rng: &mut impl Rng) -> isize {
    match weapon_data {
        None => -1,
        Some((weapon_type, weapon_buffs)) => {
            let mut total = 0;
            let (rolls, damage) = weapon_type.damage();
            let rolls = rolls + weapon_buffs.hit_enchant;
            let max_per_roll = damage + weapon_buffs.d_enchant;
            if max_per_roll >= 1 {
                for _ in 0..rolls {
                    total += rng.random_range(1..=max_per_roll as u32);
                }
            }
            total as isize
        }
    }
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
