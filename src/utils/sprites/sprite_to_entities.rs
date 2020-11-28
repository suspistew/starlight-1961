use crate::entities::collision::{Collider, Colliders};
use crate::utils::{Point2D, Direction};
use crate::entities::canons::{Canon, CanonKind};
use crate::utils::sprites::plasma_doors::*;
use crate::utils::sprites::TILE_SIZE;
use crate::entities::bonus::{BonusKind, Bonus};


pub fn sprite_to_colliders(sprite_nb: usize, pos_x: f32, pos_y: f32) -> Option<Colliders> {
    match sprite_nb {
        TOP_LEFT_WALL => {
            let top_collider = Collider::new(Point2D { x: pos_x, y: pos_y }, 32., -8.);
            let left_collider = Collider::new(Point2D { x: pos_x, y: pos_y }, 8., -32.);
            return Some(Colliders::from_vec(vec![top_collider, left_collider]));
        },
        TOP_WALL => {
            let top_collider = Collider::new(Point2D { x: pos_x, y: pos_y }, 32., -8.);
            return Some(Colliders::from_vec(vec![top_collider]));
        }
        TOP_RIGHT_WALL => {
            let top_collider = Collider::new(Point2D { x: pos_x, y: pos_y }, 32., -8.);
            let right_collider = Collider::new(Point2D { x: pos_x + 24., y: pos_y }, 8., -32.);
            return Some(Colliders::from_vec(vec![top_collider, right_collider]));
        }
        LEFT_WALL => {
            let left_collider = Collider::new(Point2D { x: pos_x, y: pos_y }, 8., -32.);
            return Some(Colliders::from_vec(vec![left_collider]));
        }
        RIGHT_WALL => {
            let right_collider = Collider::new(Point2D { x: pos_x + 24., y: pos_y }, 8., -32.);
            return Some(Colliders::from_vec(vec![right_collider]));
        }
        BOTTOM_LEFT_WALL => {
            let bottom_collider = Collider::new(Point2D { x: pos_x, y: pos_y - 24. }, 32., -8.);
            let left_collider = Collider::new(Point2D { x: pos_x, y: pos_y }, 8., -32.);
            return Some(Colliders::from_vec(vec![bottom_collider, left_collider]));
        }
        BOTTOM_WALL => {
            let bottom_collider = Collider::new(Point2D { x: pos_x, y: pos_y - 24. }, 32., -8.);
            return Some(Colliders::from_vec(vec![bottom_collider]));
        }
        BOTTOM_RIGHT_WALL => {
            let bottom_collider = Collider::new(Point2D { x: pos_x, y: pos_y - 24. }, 32., -8.);
            let right_collider = Collider::new(Point2D { x: pos_x + 24., y: pos_y }, 8., -32.);
            return Some(Colliders::from_vec(vec![bottom_collider, right_collider]));
        },
        LANDING_PLATFORM | STARTING_PLATFORM => {
            let platform_collider = Collider::new(Point2D { x: pos_x, y: pos_y }, 64., -32.);
            return Some(Colliders::from_vec(vec![platform_collider]));
        },
        DIAG_TOP_LEFT_TO_BOT_RIGHT_WALL => {
            return Some(Colliders::from_points(
                Point2D{ x: pos_x , y: pos_y },
                Point2D{ x: pos_x + 32. , y: pos_y -32.},
                Point2D{ x: pos_x + 24., y: pos_y -32.},
                Point2D{ x: pos_x , y: pos_y - 8. },
            ));
        },
        DIAG_TOP_RIGHT_TO_BOT_LEFT_WALL => {
            return Some(Colliders::from_points(
                Point2D{ x: pos_x + 32., y: pos_y },
                Point2D{ x: pos_x + 32. , y: pos_y -8.},
                Point2D{ x: pos_x + 8., y: pos_y -32.},
                Point2D{ x: pos_x , y: pos_y - 32. },
            ));
        },
        DIAG_BOT_LEFT_TO_TOP_RIGHT_WALL => {
            return Some(Colliders::from_points(
                Point2D{ x: pos_x , y: pos_y-24. },
                Point2D{ x: pos_x + 24. , y: pos_y},
                Point2D{ x: pos_x + 32., y: pos_y},
                Point2D{ x: pos_x , y: pos_y - 32. },
            ));
        },
        DIAG_BOT_RIGHT_TO_TOP_LEFT_WALL => {
            return Some(Colliders::from_points(
                Point2D{ x: pos_x , y: pos_y },
                Point2D{ x: pos_x + 8. , y: pos_y},
                Point2D{ x: pos_x + 32., y: pos_y -24.},
                Point2D{ x: pos_x +32., y: pos_y - 32. },
            ));
        },
        CANON_1_TO_LEFT => {
            let basement_collider = Collider::new(Point2D { x: pos_x, y: pos_y -4. }, 14., -24.);
            let canon_collider = Collider::new(Point2D { x: pos_x + 14., y: pos_y -14. }, 18., -8.);
            return Some(Colliders::from_vec(vec![basement_collider, canon_collider]));
        },
        CANON_1_TO_RIGHT => {
            let basement_collider = Collider::new(Point2D { x: pos_x+ 32., y: pos_y -4. }, -14., -24.);
            let canon_collider = Collider::new(Point2D { x: pos_x + 18., y: pos_y -14. }, -18., -8.);
            return Some(Colliders::from_vec(vec![basement_collider, canon_collider]));
        },
        CANON_1_TO_TOP => {
            let basement_collider = Collider::new(Point2D { x: pos_x + 4., y: pos_y -18.}, 24., -14.);
            let canon_collider = Collider::new(Point2D { x: pos_x + 14., y: pos_y }, 8., -18.);
            return Some(Colliders::from_vec(vec![basement_collider, canon_collider]));
        },
        CANON_1_TO_BOTTOM => {
            let basement_collider = Collider::new(Point2D { x: pos_x + 4., y: pos_y}, 24., -14.);
            let canon_collider = Collider::new(Point2D { x: pos_x + 14., y: pos_y - 14. }, 8., -18.);
            return Some(Colliders::from_vec(vec![basement_collider, canon_collider]));
        },
        CANON_2_TO_LEFT | CANON_2_TO_RIGHT => {
            let canon_collider = Collider::new(Point2D { x: pos_x, y: pos_y -8. }, 32., -16.);
            return Some(Colliders::from_vec(vec![canon_collider]));
        },
        CANON_2_TO_TOP | CANON_2_TO_BOTTOM => {
            let canon_collider = Collider::new(Point2D { x: pos_x + 8., y: pos_y }, 16., -32.);
            return Some(Colliders::from_vec(vec![canon_collider]));
        },
        CANON_3_TO_LEFT | CANON_3_TO_RIGHT => {
            let canon_collider = Collider::new(Point2D { x: pos_x, y: pos_y -8. }, 32., -16.);
            return Some(Colliders::from_vec(vec![canon_collider]));
        },
        HORIZONTAL_PLASMA_0_A| HORIZONTAL_PLASMA_0_B| HORIZONTAL_PLASMA_1_A| HORIZONTAL_PLASMA_1_B| HORIZONTAL_PLASMA_2_A| HORIZONTAL_PLASMA_2_B| HORIZONTAL_PLASMA_3_A| HORIZONTAL_PLASMA_3_B=> {
            let door_collider = Collider::new(Point2D { x: pos_x, y: pos_y -10. }, 32., -8.);
            return Some(Colliders::from_vec(vec![door_collider]));
        },
        VERTICAL_PLASMA_0_A | VERTICAL_PLASMA_0_B | VERTICAL_PLASMA_1_A  | VERTICAL_PLASMA_1_B | VERTICAL_PLASMA_2_A |  VERTICAL_PLASMA_2_B | VERTICAL_PLASMA_3_A |  VERTICAL_PLASMA_3_B => {
            let door_collider = Collider::new(Point2D { x: pos_x + 14., y: pos_y }, 8., -32.);
            return Some(Colliders::from_vec(vec![door_collider]));
        },
        ITEM_PLATFORM => {
            let platform_collider = Collider::new(Point2D { x: pos_x, y: pos_y -16. }, 32., -10.);
            return Some(Colliders::from_vec(vec![platform_collider]));
        },
        HORIZONTAL_PILLAR_BASEMENT_LEFT => {
            let basement = Collider::new(Point2D { x: pos_x + 10., y: pos_y }, 10., -32.);
            let tube = Collider::new(Point2D { x: pos_x + 20., y: pos_y - 10. }, 12., -12.);
            return Some(Colliders::from_vec(vec![basement, tube]));
        },
        HORIZONTAL_PILLAR_MIDDLE | HORIZONTAL_PILLAR_CLOSURE_CLOSED | HORIZONTAL_PILLAR_CLOSURE_OPENING=> {
            let tube = Collider::new(Point2D { x: pos_x, y: pos_y - 10. }, 32., -12.);
            return Some(Colliders::from_vec(vec![tube]));
        },
        HORIZONTAL_PILLAR_BASEMENT_RIGHT => {
            let basement = Collider::new(Point2D { x: pos_x + 16., y: pos_y }, 5., -32.);
            let tube = Collider::new(Point2D { x: pos_x, y: pos_y - 10. }, 12., -12.);
            return Some(Colliders::from_vec(vec![basement, tube]));
        },
        HORIZONTAL_PILLAR_SLIDING_LEFT => {
            let tube = Collider::new(Point2D { x: pos_x, y: pos_y - 10. }, 20., -12.);
            return Some(Colliders::from_vec(vec![tube]));
        },
        HORIZONTAL_PILLAR_SLIDING_RIGHT => {
            let tube = Collider::new(Point2D { x: pos_x + 12., y: pos_y - 10. }, 20., -12.);
            return Some(Colliders::from_vec(vec![tube]));
        },
        VERTICAL_PILLAR_BASEMENT_TOP => {
            let basement = Collider::new(Point2D { x: pos_x, y: pos_y - 10.}, 32., -10.);
            let tube = Collider::new(Point2D { x: pos_x + 10. , y: pos_y -20. }, 12., -12.);
            return Some(Colliders::from_vec(vec![basement, tube]));
        },
        VERTICAL_PILLAR_MIDDLE | VERTICAL_PILLAR_CLOSURE_CLOSED | VERTICAL_PILLAR_CLOSURE_OPENING => {
            let tube = Collider::new(Point2D { x: pos_x + 10., y: pos_y  }, 12., -32.);
            return Some(Colliders::from_vec(vec![tube]));
        },
        VERTICAL_PILLAR_BASEMENT_BOTTOM => {
            let basement = Collider::new(Point2D { x: pos_x, y: pos_y - 16.}, 32., -10.);
            let tube = Collider::new(Point2D { x: pos_x + 10. , y: pos_y}, 12., -12.);
            return Some(Colliders::from_vec(vec![basement, tube]));
        },
        VERTICAL_PILLAR_SLIDING_TOP => {
            let tube = Collider::new(Point2D { x: pos_x + 10. , y: pos_y }, 12., -20.);
            return Some(Colliders::from_vec(vec![tube]));
        },
        VERTICAL_PILLAR_SLIDING_BOTTOM => {
            let tube = Collider::new(Point2D { x: pos_x + 10., y: pos_y - 12. }, 12., -20.);
            return Some(Colliders::from_vec(vec![tube]));
        },
            _ => {}
    };

    None
}

pub fn init_bullet_collider(kind: &CanonKind, x: f32, y: f32 ) -> Colliders {
    match kind {
        CanonKind::Air =>  Colliders::from_vec(vec![Collider::new(Point2D { x, y }, 12., -28.)]),
        _=> Colliders::from_vec(vec![Collider::new(Point2D { x: x + 14., y: y - 16. }, 6., -4.)])
    }
}

pub fn init_bonus_collider(kind: &BonusKind, x: f32, y: f32) -> Colliders {
    Colliders::from_vec(vec![Collider::new(Point2D { x: x + 4., y: y - 4. }, 24., -24.)])
}

pub fn init_blade_saw_collider(x: f32, y: f32) -> Colliders {
    Colliders::from_vec(vec![Collider::new(Point2D { x: x + 2., y: y - 2. }, 28., -28.)])
}

pub fn is_landing_platform_start(sprite_nb: usize) -> bool{
    sprite_nb == LANDING_PLATFORM || sprite_nb == STARTING_PLATFORM
}

pub fn is_arrival(sprite_nb: usize) -> bool{
    sprite_nb == LANDING_PLATFORM
}

pub fn sprite_to_bonus_kind(sprite_nb: usize) -> Option<BonusKind> {
    match sprite_nb {
        WRENCH => Some(BonusKind::Wrench),
        FUEL => Some(BonusKind::Fuel),
        COIN => Some(BonusKind::Coin),
        _ => None
    }
}

pub fn sprite_to_canon(sprite_nb: usize, x: usize, y: usize) -> Option<Canon>{
    match sprite_nb {
        CANON_1_TO_LEFT => Some(Canon{ direction: Direction::Left, kind: CanonKind::Bullet, bullet_x_start: (x as f32  * TILE_SIZE ) - 16., bullet_y_start: (y as f32 * TILE_SIZE) - 2.  }),
        CANON_1_TO_RIGHT =>Some(Canon{ direction: Direction::Right, kind: CanonKind::Bullet, bullet_x_start: (x as f32 * TILE_SIZE ) + 16., bullet_y_start: (y as f32 * TILE_SIZE) - 2.  }),
        CANON_1_TO_TOP => Some(Canon{ direction: Direction::Top, kind: CanonKind::Bullet, bullet_x_start: (x as f32 * TILE_SIZE ), bullet_y_start: (y as f32 * TILE_SIZE)}),
        CANON_1_TO_BOTTOM => Some(Canon{ direction: Direction::Bottom, kind: CanonKind::Bullet, bullet_x_start: (x as f32 * TILE_SIZE ), bullet_y_start: (y as f32 * TILE_SIZE)}),
        CANON_2_TO_LEFT => Some(Canon{ direction: Direction::Left, kind: CanonKind::Smg, bullet_x_start: (x as f32  * TILE_SIZE ) - 16., bullet_y_start: (y as f32 * TILE_SIZE) - 1.  }),
        CANON_2_TO_RIGHT => Some(Canon{ direction: Direction::Right, kind: CanonKind::Smg, bullet_x_start: (x as f32  * TILE_SIZE ) + 16., bullet_y_start: (y as f32 * TILE_SIZE) - 1.  }),
        CANON_2_TO_TOP => Some(Canon{ direction: Direction::Top, kind: CanonKind::Smg, bullet_x_start: (x as f32  * TILE_SIZE ) , bullet_y_start: (y as f32 * TILE_SIZE) - 1.  }),
        CANON_2_TO_BOTTOM => Some(Canon{ direction: Direction::Bottom, kind: CanonKind::Smg, bullet_x_start: (x as f32  * TILE_SIZE ) , bullet_y_start: (y as f32 * TILE_SIZE) - 1.  }),
        CANON_3_TO_LEFT => Some(Canon{ direction: Direction::Left, kind: CanonKind::Air, bullet_x_start: (x as f32  * TILE_SIZE ) - 16., bullet_y_start: (y as f32 * TILE_SIZE) - 1.  }),
        CANON_3_TO_RIGHT => Some(Canon{ direction: Direction::Right, kind: CanonKind::Air, bullet_x_start: (x as f32  * TILE_SIZE ) + 16., bullet_y_start: (y as f32 * TILE_SIZE) - 1.  }),
        _ => None
    }
}

const TOP_LEFT_WALL: usize = 30;
const TOP_WALL: usize = 31;
const TOP_RIGHT_WALL: usize = 32;
const LEFT_WALL: usize = 40;
const RIGHT_WALL: usize = 42;
const BOTTOM_LEFT_WALL: usize = 50;
const BOTTOM_WALL: usize = 51;
const BOTTOM_RIGHT_WALL: usize = 52;
const DIAG_TOP_LEFT_TO_BOT_RIGHT_WALL: usize = 34;
const DIAG_TOP_RIGHT_TO_BOT_LEFT_WALL: usize = 33;
const DIAG_BOT_LEFT_TO_TOP_RIGHT_WALL: usize = 44;
const DIAG_BOT_RIGHT_TO_TOP_LEFT_WALL: usize = 43;

const ITEM_PLATFORM: usize = 96;

const CANON_1_TO_LEFT: usize = 18;
const CANON_1_TO_RIGHT: usize = 8;
const CANON_1_TO_TOP: usize = 17;
const CANON_1_TO_BOTTOM: usize = 7;

const CANON_2_TO_LEFT: usize = 28;
const CANON_2_TO_RIGHT: usize = 27;
const CANON_2_TO_TOP: usize = 38;
const CANON_2_TO_BOTTOM: usize = 37;


const CANON_3_TO_LEFT: usize = 48;
const CANON_3_TO_RIGHT: usize = 47;

const LANDING_PLATFORM: usize = 90;
const STARTING_PLATFORM: usize = 92;
pub const BLADE_SAW_SPRITE: usize = 16;

const WRENCH: usize = 99;
const FUEL: usize = 98;
pub const COIN: usize = 97;


const HORIZONTAL_PILLAR_BASEMENT_LEFT: usize = 110;
const HORIZONTAL_PILLAR_MIDDLE: usize = 111;
const HORIZONTAL_PILLAR_CLOSURE_CLOSED: usize = 112;
const HORIZONTAL_PILLAR_CLOSURE_OPENING: usize = 122;
const HORIZONTAL_PILLAR_BASEMENT_RIGHT: usize = 113;
const HORIZONTAL_PILLAR_SLIDING_LEFT: usize = 101;
const HORIZONTAL_PILLAR_SLIDING_RIGHT: usize = 102;

const VERTICAL_PILLAR_BASEMENT_TOP: usize = 132;
const VERTICAL_PILLAR_MIDDLE: usize = 142;
const VERTICAL_PILLAR_CLOSURE_CLOSED: usize = 152;
const VERTICAL_PILLAR_CLOSURE_OPENING: usize = 151;
const VERTICAL_PILLAR_BASEMENT_BOTTOM: usize = 162;
const VERTICAL_PILLAR_SLIDING_TOP: usize = 140;
const VERTICAL_PILLAR_SLIDING_BOTTOM: usize = 150;
