use crate::entities::collision::{Collider, Colliders};
use crate::utils::Point2D;

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
        }
        _ => {}
    };

    None
}

pub fn is_landing_platform_start(sprite_nb: usize) -> bool{
    sprite_nb == LANDING_PLATFORM || sprite_nb == STARTING_PLATFORM
}


const TOP_LEFT_WALL: usize = 30;
const TOP_WALL: usize = 31;
const TOP_RIGHT_WALL: usize = 32;
const LEFT_WALL: usize = 40;
const RIGHT_WALL: usize = 42;
const BOTTOM_LEFT_WALL: usize = 50;
const BOTTOM_WALL: usize = 51;
const BOTTOM_RIGHT_WALL: usize = 52;

const LANDING_PLATFORM: usize = 90;
const STARTING_PLATFORM: usize = 92;