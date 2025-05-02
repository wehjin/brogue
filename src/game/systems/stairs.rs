use crate::game::components::{Pack, Rogue, TileType};
use crate::game::values::grid::GridOffset;
use crate::game::values::pack_item::PackItem;
use bevy::prelude::*;
use std::process::exit;

pub fn handle_keyboard_ascent(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    rogue: Single<(&GridOffset, &Pack), With<Rogue>>,
    query_tiles: Query<(&GridOffset, &TileType)>,
) {
    if just_pressed_angle_bracket_right(keyboard_input) {
        let (rogue_position, rogue_pack) = rogue.into_inner();
        let tile_type_at_rogue_position = tile_type_at_position(query_tiles, rogue_position);
        if TileType::Stairs == tile_type_at_rogue_position {
            match rogue_pack.find_item(PackItem::Amulet) {
                None => {
                    println!("Your way is magically blocked");
                }
                Some(_) => {
                    println!("you feel a wrenching sensation in your gut");
                    win();
                }
            }
        } else {
            println!("I see no way up");
        }
    }
}

fn win() {
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!("          @   @  @@@   @   @      @  @  @   @@@   @   @   @");
    println!("           @ @  @   @  @   @      @  @  @  @   @  @@  @   @");
    println!("            @   @   @  @   @      @  @  @  @   @  @ @ @   @");
    println!("            @   @   @  @   @      @  @  @  @   @  @  @@    ");
    println!("            @    @@@    @@@        @@ @@    @@@   @   @   @");
    println!();
    println!();
    println!("          Congratulations,  you have  been admitted  to  the");
    println!("          Fighters' Guild.   You return home,  sell all your");
    println!("          treasures at great profit and retire into comfort.");
    println!();
    println!();
    exit(0);
}

fn tile_type_at_position(
    query_tiles: Query<(&GridOffset, &TileType)>,
    grid_position: &GridOffset,
) -> TileType {
    let tile_type = query_tiles
        .iter()
        .find_map(|(position, tile_type)| {
            if *position == *grid_position {
                Some(tile_type)
            } else {
                None
            }
        })
        .expect("rogue should be on a tile");
    *tile_type
}

fn just_pressed_angle_bracket_right(keyboard_input: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard_input.just_pressed(KeyCode::Period)
        && keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight])
}
