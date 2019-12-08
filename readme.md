# Tiled Wave Function Collapse

This library contains helper functions to use the Wave Function Collapse algorithm provided by the `wfc` crate on tile-based maps.

You can load layer CSV files like the ones exported from Tiled, and save the result as another CSV or as a Tiled .tmx file for previewing inside the software.

As the underlying library only works on two dimensions, multiple layers are not supported.

## Getting Started

The crate includes commented example programs under the `examples/` folder.

The `simple` example takes a CSV tile map as its only argument and saves the output of wfc as a CSV as well as a .tmx file.

You can run it with:
```
cargo run --example=simple examples\input.csv examples\tiles.png
```
(Replace the slashes for non-Windows environments)

This will produce `out.tmx` and `out.csv` containing the 32x32 tile WFC output.

### Code
For convenience I provide code equivalent to the example program here: 
```Rust
let input_path = "example\\input.csv";
let tilesheet_path = "example\\tiles.png";
let attempts = 1000;
let pattern_size = 2;
let output_size = Size::new(32, 32);

// Extract patterns from input
let pattern = TilePattern::from_csv(input_path, 
    std::num::NonZeroU32::new(pattern_size).unwrap(), 
    &[Orientation::Original])
    .expect("Error while creating pattern");

// Run Wave Function Collapse
let grid = pattern.run_collapse(output_size, 
    attempts, 
    WrapXY, 
    ForbidNothing, 
    &mut rand::thread_rng())
    .expect("Error in WFC");

// Save as CSV
grid_to_csv(&grid, "out.csv")?;

// Save as Tiled .tmx file
let tset = TileSet {
    image_path: tilesheet_path,
    image_size: Size::new(256, 1450),
    columns: 8,
    tile_count: 360,
    tile_size: Size::new(32, 32)
};
grid_to_tiled(&grid, "out.tmx", tset)?;
```
This assumes you have the necessary import 
```Rust
use wfc_tiled::*;
```

### Anchoring

The `anchor` example shows how to prevent the output from wrapping around the edges of the map.

It's analogous to the example with the same name in the `wfc-image` crate.

You can run it with:

```
cargo run --example=anchor examples\input.csv examples\tiles.png
```

The program uses a `Forbid` rule which forces the right and bottom borders of the output to be equal to the bottom right corner of the input.
Since the system wraps around edges automatically, this is the same as forcing the output to be surrounded by a "wall" made of the input's bottom right corner tiles.

You can look at `src/forbid_corner.rs` to learn how to construct your own forbid rule. These rules can force `wfc` to only allow certain whitelisted tiles in the output for specific coordinates, or to only forbid certain blacklisted tiles at certain coordinates.

The `ForceBorderForbid` used in the example takes the tiles in the lower right corner and sets those as the only allowed tiles for each coordinate corresponding to the bottom and right borders of the output.