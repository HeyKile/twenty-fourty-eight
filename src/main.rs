use macroquad::prelude::*;

const TILE_SIZE: i32 = 50;
const SQUARES: i16 = 16;

type Point = (i16, i16);
struct Square {
    value: u32,
    pos: Point,
    color: Color
}

#[macroquad::main("BasicShapes")]
async fn main() {

    let game_over = false;

    loop {
        if !game_over {
            clear_background(Color::new(0.20, 0.20, 0.20, 1.00));
            
            let game_size = screen_width().min(screen_height());
            let offset_x = (screen_width() - game_size) / 2. + 10.;
            let offset_y = (screen_height() - game_size) / 2. + 10.;
            let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

            draw_rectangle(offset_x - 4., offset_y - 4., game_size - 12., game_size - 12., Color::new(0.16, 0.16, 0.16, 1.00));
            draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., DARKGRAY);

            // for i in 1..SQUARES {
            //     draw_line(
            //         offset_x,
            //         offset_y + sq_size * i as f32,
            //         screen_width() - offset_x,
            //         offset_y + sq_size * i as f32,
            //         2.,
            //         LIGHTGRAY
            //     );
            // }

            // for i in 1..SQUARES {
            //     draw_line(
            //         offset_x + sq_size * i as f32,
            //         offset_y,
            //         offset_x + sq_size * i as f32,
            //         screen_height() - offset_y,
            //         2.,
            //         LIGHTGRAY,
            //     );
            // }

            // clear_background(DARKGRAY);
            // draw_text("We Mov!", 20.0, 20.0, 30.0, DARKGREEN);
        }
        next_frame().await;
    }
}
