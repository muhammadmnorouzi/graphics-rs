# A simple, extendable, CPU based 2D graphics library. Also supports CloudPoints and rotation!

## Draw simple shapes with the least effort needed.
```rust
const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() -> Result<(), String> {
    let antialiasing = false;
    let antialiasing_resolution = 1;
    let fill_color = Some(color::BLACK);
    
    let mut canvas =
        SimpleCanvas::new(
            WIDTH, 
            HEIGHT, 
            fill_color, 
            antialiasing, 
            antialiasing_resolution);

    canvas.change_color(color::BLACK);
    canvas.fill();

    canvas.change_color(color::GREEN);
    canvas.draw_shape(&mut Circle::new(WIDTH /2 , HEIGHT / 2, 100));

    canvas
        .save("canvas.ppm")
        .map_err(|error| error.to_string())?;

    Ok(())
}
```