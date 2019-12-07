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

pub use coord_2d::{Coord, Size};
pub use wrap::WrapXY;
pub use wfc::orientation::{Orientation};
pub use wfc::{ForbidNothing, ForbidInterface};

mod forbid_corner;
mod tile_pattern;
use tile_pattern::*;
use forbid_corner::*;
pub struct Forbid {
    pattern_ids: HashSet<PatternId>,
    offset: i32,
}

impl ForbidPattern for Forbid {
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

fn format_tiled(map: String, size: Size, tileset: TileSet) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.1" orientation="orthogonal" renderorder="right-down" compressionlevel="-1"
     width="{width}" height="{height}" tilewidth="{twidth}" tileheight="{theight}" infinite="0" nextlayerid="3" nextobjectid="1">
     <tileset firstgid="1" name="default" tilewidth="{twidth}" tileheight="{theight}" tilecount="{tcount}" columns="{tcolumns}">
      <image source="{tpath}" width="{tsetwidth}" height="{tsetheight}"/>
     </tileset>
     <layer id="1" name="base" width="{width}" height="{height}">
      <data encoding="csv">
      {layer}
      </data>
    </layer>
    </map>"#,
        layer=map,
        width=size.width(), height=size.height(),
        twidth=tileset.tile_size.width(), theight=tileset.tile_size.height(),
        tsetwidth=tileset.image_size.width(), tsetheight=tileset.image_size.height(),
        tcount=tileset.tile_count,
        tcolumns=tileset.columns,
        tpath=tileset.image_path
    )
}
pub struct TileSet {
    image_path: String,
    image_size: Size,
    columns: u32,
    tile_size: Size,
    tile_count: u32
}
pub struct TilePattern {
    pub grid: Grid<u32>,
    pub overlapping_patterns: OverlappingPatterns<u32>
}

impl TilePattern {
    fn new(grid: Grid<u32>, pattern_size: NonZeroU32, orientation: &[Orientation]) -> TilePattern {
        let overlapping_patterns = OverlappingPatterns::new(grid.clone(), pattern_size, orientation);
        return TilePattern { grid, overlapping_patterns};
    }
    fn from_vec(map: Vec<u32>, size: Size, pattern_size: NonZeroU32, orientation: &[Orientation]) -> TilePattern {
        let grid = Grid::new_fn(size, |Coord { x, y }| {
            map[(y*(size.width() as i32) + x) as usize]
        });
        return TilePattern::new(grid, pattern_size, orientation);
    }

    fn from_csv<P: AsRef<Path>>(path: P, pattern_size: NonZeroU32, orientation: &[Orientation]) -> Result<TilePattern, Box<dyn Error>>{
        let mut map = Vec::new();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path)?;
        let mut columns: usize = 0;
        let mut rows: usize = 0;
        for result in rdr.deserialize() {
            let record: Vec<u32> = result?;
            columns = record.len();
            rows += 1;
            map.extend(record);
        }
        let size = Size::new(columns as u32, rows as u32);

        return Ok(TilePattern::from_vec(map, size, pattern_size, orientation));
    }

    fn run_collapse<W: Wrap, F: ForbidPattern, R: Rng>(&self, output_size: Size, retry_times: usize, wrap: W, forbid: F, rng: &mut R) 
    -> Result<Grid<u32>, PropagateError> {
        let global_stats = self.overlapping_patterns.global_stats();
        let run = RunOwn::new_wrap_forbid(output_size, &global_stats, wrap, forbid, rng);
        let wave = run.collapse_retrying(retry::NumTimes(retry_times), rng)?;
        let wave_grid = wave.grid();
        let grid = Grid::new_fn(wave_grid.size(), |coord| {
            *self.overlapping_patterns.pattern_top_left_value(wave_grid.get(coord).unwrap().chosen_pattern_id().unwrap())
        });
        return Ok(grid);
    }
}

fn u32conv(x:u32) -> [u8;4] {
    let _b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b4, b3, b2, 255]
}

pub fn grid_to_image(grid: &Grid<u32>) -> DynamicImage {
    let size = grid.size();
    let mut rgba_image = RgbaImage::new(size.width(), size.height());
    grid.enumerate().for_each(|(Coord { x, y }, cell)| {
        rgba_image.put_pixel(x as u32, y as u32, Rgba { data: u32conv(*cell)});
    });
    return DynamicImage::ImageRgba8(rgba_image);
}
pub fn grid_to_csv<P: AsRef<Path>>(grid: &Grid<u32>, path: P) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new().from_path(path)?;
    let size = grid.size();
    for y in 0..(size.height() as i32) {
        let mut row = Vec::new();
        for x in 0..(size.width() as i32) {
            let cell: u32 = *grid.get(Coord {x, y} ).unwrap();
            row.push(cell.to_string());

        }
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    Ok(())
}
pub fn grid_to_tiled<P: AsRef<Path>>(grid: &Grid<u32>, path: P, tileset: TileSet) -> Result<(), Box<dyn Error>> {
    let mut csv_grid = String::new();
    let size = grid.size();
    for y in 0..(size.height() as i32) {
        for x in 0..(size.width() as i32) {
            let cell: u32 = *grid.get(Coord { x, y} ).unwrap() + 1;
            csv_grid = format!("{}{}{}", csv_grid, cell, ",");
        }
        csv_grid = format!("{}{}", csv_grid, "\n");
    }
    // remove trailing newline and last comma
    csv_grid.pop();
    csv_grid.pop();
    let output = format_tiled(csv_grid, size, tileset);
    let mut file = std::fs::File::create(path)?;
    file.write_all(&output.into_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let pattern_size = 2;
    let pattern = TilePattern::from_csv("house1.csv", NonZeroU32::new(pattern_size).unwrap(), &[Orientation::Original]).expect("Pattern Err");
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
    let forbid = Forbid {
        pattern_ids: bottom_right_ids,
        offset: bottom_right_offset as i32,
    };
    let grid = pattern.run_collapse(Size::new(128, 128), 1000, WrapXY, ForbidNothing, &mut rand::thread_rng()).expect("Err");
    let img = grid_to_image(&grid);
    img.save("outhouse.png").expect("Failed to save");
    let tset = TileSet {
        image_path: "../../../Documents/magecity_1.png".to_string(),
        image_size: Size::new(256, 1450),
        columns: 8,
        tile_count: 360,
        tile_size: Size::new(32, 32)
    };
    grid_to_tiled(&grid, "outhouse.tmx", tset)?;
    return Ok(());
}
