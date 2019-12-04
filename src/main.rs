extern crate image;
extern crate wfc;
extern crate grid_2d;
extern crate coord_2d;

pub use coord_2d::{Coord, Size, Axis};

use grid_2d::Grid;
use std::num::NonZeroU32;
use image::{DynamicImage, Rgba, RgbaImage};
use wfc::overlapping::{OverlappingPatterns};
use wfc::{ForbidPattern, ForbidInterface, ForbidNothing, RunOwn, retry, wrap};
pub use wrap::WrapXY;
pub use wfc::orientation::{self, Orientation};

use std::error::Error;
use std::path::Path;

fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let _b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b4, b3, b2, 255]
}

struct TilePattern {
    grid: Grid<u32>,
    size: Size
}
impl TilePattern {
    fn new(map: Vec<u32>, size: Size) -> TilePattern {
        let grid = Grid::new_fn(size, |Coord { x, y }| {
            map[(y*(size.get(Axis::X) as i32) + x) as usize]
        });
        return TilePattern { size: size, grid: grid };
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
}
fn load_from_csv<P: AsRef<Path>>(path: P) -> Result<Vec<u32>, Box<dyn Error>> {
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

    return Ok(map);
}
fn main() -> Result<(), Box<dyn Error>> {
    let map = load_from_csv("small.csv")?;
    let input_size = Size::new(16, 16);
    let grid = Grid::new_fn(input_size, |Coord { x, y }| {
        map[(y*16+x) as usize]
    });
    let pattern_size = NonZeroU32::new(2).unwrap();
    let overlapping_patterns = OverlappingPatterns::new(grid, pattern_size, &[Orientation::Original]);
    let mut rng = rand::thread_rng();
    let output_size = Size::new(48, 48);
    let global_stats = overlapping_patterns.global_stats();
    let run = RunOwn::new_wrap_forbid(output_size, &global_stats, WrapXY, ForbidNothing, &mut rng);
    let result = run.collapse_retrying(retry::NumTimes(10), &mut rng);
    let wave = match result {
        Ok(wave_res) => wave_res,
        Err(s) => {
            println!("{:?}", s);
            return Ok(());
        }
    };
    let size = wave.grid().size();
    let mut rgba_image = RgbaImage::new(size.width(), size.height());
    wave.grid().enumerate().for_each(|(Coord { x, y }, cell)| {
        let colour = match cell.chosen_pattern_id() {
            Ok(pattern_id) => {
                Rgba { data: transform_u32_to_array_of_u8(*overlapping_patterns.pattern_top_left_value(pattern_id)) }
            },
            Err(_) => Rgba { data: [0, 0, 0, 0] },
        };
        rgba_image.put_pixel(x as u32, y as u32, colour);
    });
    let img = DynamicImage::ImageRgba8(rgba_image);
    img.save("outnew.png").expect("Failed to save");
    return Ok(());
}
