extern crate wfc_tiled;
use wfc_tiled::*;

fn main() -> Result<(), Box<dyn Error>> {
    let pattern_size = 2;
    let pattern = TilePattern::from_csv("test.csv", NonZeroU32::new(pattern_size).unwrap(), &[Orientation::Original]).expect("Pattern Err");
    let forbid = ForceBorderForbid::new(&pattern, pattern_size);
    let grid = pattern.run_collapse(Size::new(128, 128), 1000, WrapXY, ForbidNothing, &mut rand::thread_rng()).expect("Err");
    let img = grid_to_image(&grid);
    img.save("out.png").expect("Failed to save");
    let tset = TileSet {
        image_path: "tilemap.png".to_string(),
        image_size: Size::new(256, 1450),
        columns: 8,
        tile_count: 360,
        tile_size: Size::new(32, 32)
    };
    grid_to_tiled(&grid, "out.tmx", tset)?;
    return Ok(());
}
