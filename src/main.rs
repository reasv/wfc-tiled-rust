extern crate image;
extern crate wfc;
extern crate grid_2d;
extern crate coord_2d;

pub use coord_2d::{Coord, Size};

use grid_2d::Grid;
use std::num::NonZeroU32;
use image::{DynamicImage, Rgba, RgbaImage};
use wfc::overlapping::{OverlappingPatterns};
use wfc::{ForbidPattern, ForbidInterface, ForbidNothing, RunOwn, retry, wrap, Wrap, PropagateError, Wave};
pub use wrap::WrapXY;
pub use wfc::orientation::{self, Orientation};

use std::error::Error;
use std::path::Path;
use rand::Rng;

fn u32conv(x:u32) -> [u8;4] {
    let _b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b4, b3, b2, 255]
}

struct TilePattern {
    grid: Grid<u32>,
}
impl TilePattern {
    fn new(map: Vec<u32>, size: Size) -> TilePattern {
        let grid = Grid::new_fn(size, |Coord { x, y }| {
            map[(y*(size.width() as i32) + x) as usize]
        });
        return TilePattern { grid: grid };
    }

    fn from_csv<P: AsRef<Path>>(path: P) -> Result<TilePattern, Box<dyn Error>>{
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

        return Ok(TilePattern::new(map, size));
    }
    fn run_collapse<W: Wrap, F: ForbidPattern, R: Rng>(&self, output_size: Size, pattern_size: NonZeroU32, retry_times: usize, orientation: &[Orientation], wrap: W, forbid: F, rng: &mut R) 
    -> Result<Grid<u32>, PropagateError> {
        let overlapping_patterns = OverlappingPatterns::new(self.grid.clone(), pattern_size, orientation);
        let global_stats = overlapping_patterns.global_stats();
        let run = RunOwn::new_wrap_forbid(output_size, &global_stats, wrap, forbid, rng);
        let wave = run.collapse_retrying(retry::NumTimes(retry_times), rng)?;
        let wave_grid = wave.grid();
        let grid = Grid::new_fn(wave_grid.size(), |coord| {
            *overlapping_patterns.pattern_top_left_value(wave_grid.get(coord).unwrap().chosen_pattern_id().unwrap())
        });
        return Ok(grid);
    }
}
fn grid_to_image(grid: &Grid<u32>) -> DynamicImage {
    let size = grid.size();
    let mut rgba_image = RgbaImage::new(size.width(), size.height());
    grid.enumerate().for_each(|(Coord { x, y }, cell)| {
        rgba_image.put_pixel(x as u32, y as u32, Rgba { data: u32conv(*cell)});
    });
    return DynamicImage::ImageRgba8(rgba_image);
}
fn main() -> Result<(), Box<dyn Error>> {
    let grid = (TilePattern::from_csv("small.csv").expect("err")).run_collapse(Size::new(48,48), NonZeroU32::new(2).unwrap(), 10, &[Orientation::Original], WrapXY, ForbidNothing, &mut rand::thread_rng()).expect("Err");
    let img = grid_to_image(&grid);
    img.save("outnew.png").expect("Failed to save");
    return Ok(());
}
