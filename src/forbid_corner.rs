extern crate image;
extern crate wfc;
extern crate grid_2d;
extern crate coord_2d;

use std::error::Error;
use std::path::Path;
use std::num::NonZeroU32;
use std::io::prelude::*;
use std::collections::HashSet;

use rand::Rng;
use grid_2d::Grid;
use image::{DynamicImage, Rgba, RgbaImage};
use wfc::overlapping::{OverlappingPatterns};
use wfc::{ForbidPattern, RunOwn, retry, wrap, Wrap, PropagateError, PatternId};

use coord_2d::{Coord, Size};
use wrap::WrapXY;
use wfc::orientation::{Orientation};
use wfc::{ForbidNothing, ForbidInterface};

use crate::tile_pattern::TilePattern;
pub struct ForceBorderForbid {
    pattern_ids: HashSet<PatternId>,
    offset: i32,
}

impl ForbidPattern for ForceBorderForbid {
    fn forbid<W: Wrap, R: Rng>(&mut self, fi: &mut ForbidInterface<W>, rng: &mut R) {
        let output_size = fi.wave_size();
        (0..(output_size.width() as i32))
            .map(|x| Coord::new(x, output_size.height() as i32 - self.offset as i32))
            .chain(
                (0..(output_size.width() as i32)).map(|y| {
                    Coord::new(output_size.width() as i32 - self.offset as i32, y)
                }),
            )
            .for_each(|coord| {
                self.pattern_ids.iter().for_each(|&pattern_id| {
                    fi.forbid_all_patterns_except(coord, pattern_id, rng)
                        .unwrap();
                });
            });
    }
}
impl ForceBorderForbid {
    pub fn new(pattern: TilePattern, pattern_size: u32) -> ForceBorderForbid{
        let input_size = pattern.overlapping_patterns.grid().size();
        let bottom_right_offset = pattern_size - (pattern_size / 2);
        let id_grid = pattern.overlapping_patterns.id_grid();
        let bottom_right_coord = Coord::new(
            input_size.width() as i32 - bottom_right_offset as i32,
            input_size.height() as i32 - bottom_right_offset as i32,
        );
        let bottom_right_ids = id_grid
            .get_checked(bottom_right_coord)
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        return ForceBorderForbid {
            pattern_ids: bottom_right_ids,
            offset: bottom_right_offset as i32,
        };
    }
}