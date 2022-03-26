use bevy::prelude::*;
use std::collections::HashMap;

use super::TILE_SIZE;

#[derive(Debug)]
pub struct Obstacle {
    pub pos: Point<i32>,
    pub is_one_way: bool,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Point<T: Copy>(pub T, pub T);

#[derive(Debug, PartialEq)]
pub struct BBox<T: Copy> {
    left: T,
    bottom: T,
    right: T,
    top: T,
}

impl<T: Copy> BBox<T> {
    pub fn new(left: T, bottom: T, right: T, top: T) -> BBox<T> {
        BBox {
            left,
            bottom,
            right,
            top,
        }
    }
}

/// Map screen-space bbox to tile-space bbox
pub fn get_tile_space_bbox(bbox: &BBox<f32>) -> BBox<i32> {
    BBox::new(
        (bbox.left / TILE_SIZE).floor() as i32,
        (bbox.bottom / TILE_SIZE).floor() as i32,
        (bbox.right / TILE_SIZE).floor() as i32,
        (bbox.top / TILE_SIZE).floor() as i32,
    )
}

/// Map tile-space bbox to list of tile coords
pub fn get_tile_list(bbox: BBox<i32>) -> Vec<Point<i32>> {
    let mut tiles = Vec::new();

    for y in bbox.bottom..(bbox.top + 1) {
        for x in bbox.left..(bbox.right + 1) {
            tiles.push(Point(x, y));
        }
    }

    tiles
}

/// Convert screen-space Point to tile-space
pub fn to_tile_space(pos: &Vec2) -> Point<i32> {
    Point(
        (pos.x / TILE_SIZE).floor() as i32,
        (pos.y / TILE_SIZE).floor() as i32,
    )
}

pub fn get_first_obstacle_pos_downward(
    obstacles: &HashMap<Point<i32>, Obstacle>,
    pos: Point<i32>,
) -> Option<Point<i32>> {
    for y in (0..pos.1).rev() {
        let obs = obstacles.get(&Point(pos.0, y));

        if obs.is_some() {
            return Some(obs.unwrap().pos.clone());
        }
    }

    None
}

/// Given a tile list, get the ones with obstacles
pub fn get_obstacle_list(
    tiles: Vec<Point<i32>>,
    obstacles: &HashMap<Point<i32>, Obstacle>,
    ignore_one_way: bool,
) -> Vec<&Obstacle> {
    tiles
        .iter()
        .filter_map(|p| obstacles.get(p))
        .filter(|o| {
            if ignore_one_way {
                return !o.is_one_way;
            }

            true
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_tile_space_bbox() {
        assert_eq!(
            get_tile_space_bbox(&BBox::new(1.0, 1.0, 24.0, 38.0)),
            BBox::new(0, 0, 1, 2)
        );

        assert_eq!(
            get_tile_space_bbox(&BBox::new(0.0, 0.0, 16.0, 16.0)),
            BBox::new(0, 0, 1, 1)
        );

        assert_eq!(
            get_tile_space_bbox(&BBox::new(0.0, 0.0, 15.0, 15.0)),
            BBox::new(0, 0, 0, 0)
        );

        assert_eq!(
            get_tile_space_bbox(&BBox::new(0.0, 0.0, 70.0, 8.0)),
            BBox::new(0, 0, 4, 0)
        );
    }

    #[test]
    fn test_get_tile_list() {
        assert_eq!(
            get_tile_list(BBox::new(0, 0, 1, 2)),
            vec![
                Point(0, 0),
                Point(1, 0),
                Point(0, 1),
                Point(1, 1),
                Point(0, 2),
                Point(1, 2)
            ]
        );

        assert_eq!(get_tile_list(BBox::new(0, 0, 0, 0)), vec![Point(0, 0)]);
    }

    #[test]
    fn test_get_obstacle_list() {
        let mut obstacles = HashMap::new();

        obstacles.insert(
            Point(0, 1),
            Obstacle {
                pos: Point(0, 1),
                is_one_way: false,
            },
        );

        obstacles.insert(
            Point(1, 1),
            Obstacle {
                pos: Point(1, 1),
                is_one_way: true,
            },
        );

        let list = get_obstacle_list(get_tile_list(BBox::new(0, 0, 1, 2)), &obstacles, false);

        assert_eq!(list.len(), 2);
        assert_eq!(list[0].pos, Point(0, 1));
        assert_eq!(list[1].pos, Point(1, 1));

        let list = get_obstacle_list(get_tile_list(BBox::new(0, 0, 1, 2)), &obstacles, true);

        assert_eq!(list.len(), 1);
        assert_eq!(list[0].pos, Point(0, 1));
    }
}
