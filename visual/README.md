# Visualise the problems

In `Cargo.toml`:

```
[dependencies]
visual = { path = "../../visual" }
```


## 2D dimensional grid

```rust
use visual::GridRenderer;

// Assuming you have a Vec<Vec<T>>

let colour_mapper = |cell: &T| match cell {
    X => 0xFFFFFF, // Path (white)
    Y => 0x000000, // Wall (black)
    _ => 0x808080, // Unknown (gray)
};

// Create renderer with the grid, cell size of 10 and the colour mapper
let width = 40;
let height = 50;
let mut renderer = GridRenderer::new(width, height, 10, colour_mapper);

// Then in a loop somewhere
renderer.render(&nodes, None);

// You can write basic text (digits only for now) as well
let number = 123;
let x = 10;
let y = 10;
let font_colour = 0xFF0000;
let font_scale = 4;

renderer.render(
    &nodes,
    Some((&format!("{}", number), x, y, font_colour, font_scale))
);
```
