extern crate image;
extern crate wfc;
extern crate grid_2d;
extern crate coord_2d;

use std::error::Error;
use std::path::Path;
use std::num::NonZeroU32;

use rand::Rng;
use grid_2d::Grid;
use wfc::{ForbidPattern, RunOwn, retry, Wrap, PropagateError};
use wfc::orientation::{Orientation};
use wfc::overlapping::{OverlappingPatterns};

use coord_2d::{Coord, Size};

/// Extracts a pattern from a tilemap and lets you run [Wave Function Collapse](https://github.com/mxgmn/WaveFunctionCollapse) on it.
/// 
/// This struct can be initialized from a .csv file, a vector, or a [Grid](../grid_2d/grid/struct.Grid.html).
/// 
/// It will automatically extract overapping patterns of the given size (in tiles), and respecting the orientation setting.
/// 
/// For example, if you don't want to allow your tiles to be rotated, use [Orientation::Original](enum.Orientation.html#variant.Original)
/// 
/// Afterwards call [run_collapse()](#method.run_collapse) in order to generate a new tilemap based on the input.
/// 
/// WFC is not guaranteed to succeed, even with the correct input. For this reason there is a `retry_times` parameter indicating the number of attemps that
/// will be done. If it still fails, the function will return an error.
/// 
/// # Examples
///
/// ```
/// let input_path = "example\\input.csv";
/// let tilesheet_path = "example\\tset.png";
/// let attempts = 1000;
/// let pattern_size = 2;
/// let output_size = Size::new(32, 32);

/// // Extract patterns from input
/// let pattern = TilePattern::from_csv(input_path, 
///     std::num::NonZeroU32::new(pattern_size).unwrap(), 
///     &[Orientation::Original])
///     .expect("Error while creating pattern");

/// // Run Wave Function Collapse
/// let grid = pattern.run_collapse(output_size, 
///     attempts, 
///     WrapXY, 
///     ForbidNothing, 
///     &mut rand::thread_rng())
///     .expect("Error in WFC");

/// // Save as CSV
/// grid_to_csv(&grid, "out.csv")?;

/// // Save as Tiled .tmx file
/// let tset = TileSet {
///     image_path: tilesheet_path,
///     image_size: Size::new(256, 1450),
///     columns: 8,
///     tile_count: 360,
///     tile_size: Size::new(32, 32)
/// };
/// grid_to_tiled(&grid, "out.tmx", tset)?;
/// ```
pub struct TilePattern {
    pub grid: Grid<u32>,
    pub overlapping_patterns: OverlappingPatterns<u32>
}

impl TilePattern {
    pub fn new(grid: Grid<u32>, pattern_size: NonZeroU32, orientation: &[Orientation]) -> TilePattern {
        let overlapping_patterns = OverlappingPatterns::new(grid.clone(), pattern_size, orientation);
        return TilePattern { grid, overlapping_patterns};
    }
    pub fn from_vec(map: Vec<u32>, size: Size, pattern_size: NonZeroU32, orientation: &[Orientation]) -> TilePattern {
        let grid = Grid::new_fn(size, |Coord { x, y }| {
            map[(y*(size.width() as i32) + x) as usize]
        });
        return TilePattern::new(grid, pattern_size, orientation);
    }

    pub fn from_csv<P: AsRef<Path>>(path: P, pattern_size: NonZeroU32, orientation: &[Orientation]) -> Result<TilePattern, Box<dyn Error>>{
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

    pub fn run_collapse<W: Wrap, F: ForbidPattern, R: Rng>(&self, output_size: Size, retry_times: usize, wrap: W, forbid: F, rng: &mut R) 
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