use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            },
            Health { current: 10, max: 10},
            FieldOfView::new(80),
        )
    );
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push(
        (Item, AmuletOfYala, pos, Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
            Name("Amulet of Yala".to_string())
        )
    );
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, view_radius, glyph) = match rng.roll_dice(1,10) {
        1..=8 => goblin(),
        _ => orc()
    };

    ecs.push(
        (
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph,
            },
            ChasingPlayer,
            Health{current: hp, max: hp},
            Name(name),
            FieldOfView::new(view_radius),
        )
    );
}

fn goblin() -> (i32, String, i32, FontCharType) {
    (1, "Goblin".to_string(), 6, to_cp437('g'))
}

fn orc() -> (i32, String, i32, FontCharType) {
    (2, "Orc".to_string(), 4, to_cp437('o'))
}