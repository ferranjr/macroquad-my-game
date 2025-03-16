use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    loop {
        clear_background(DARKBLUE);
        next_frame().await
    }
}
