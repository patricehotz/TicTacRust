use macroquad::prelude::*;

#[derive(Debug)]
enum FieldState {
    Player1,
    Player2,
    None,
}

struct GameState {
    selected: (u32, u32),
    field_states: [[FieldState; 3]; 3],
}

impl GameState {
    fn default() -> GameState {
        GameState {
            selected: (0, 0),
            field_states: [
                [FieldState::None, FieldState::None, FieldState::None],
                [FieldState::None, FieldState::None, FieldState::None],
                [FieldState::None, FieldState::None, FieldState::None],
            ],
        }
    }

    fn draw(&mut self) {
        let offset = 50.0;
        let rec_width = (screen_width() - offset * 2.0) / 3.0 * screen_height() / screen_width();
        let rec_height = (screen_height() - offset * 2.0) / 3.0;
        let x_offset = (screen_width() - offset - rec_width * 3.0) / 2.0;
        let y_offset = offset;
        let background_color = BLACK;
        let line_color = WHITE;
        let (selected_x, selected_y) = self.selected;

        //TODO make line color = background color when field selected
        //TODO make background color = background color when field selected
        //Daw Circle / X if fieldstate != none

        for (i, row) in self.field_states.iter().enumerate() {
            for (j, field) in row.iter().enumerate() {
                match field {
                    FieldState::None => {
                        draw_rectangle_lines(
                            (rec_width * j as f32) + x_offset,
                            rec_height * i as f32 + y_offset,
                            rec_width,
                            rec_height,
                            10.0,
                            line_color,
                        );
                    }
                    FieldState::Player1 => {
                        draw_rectangle_lines(
                            (rec_width * j as f32) + x_offset,
                            rec_height * i as f32 + y_offset,
                            rec_width,
                            rec_height,
                            10.0,
                            line_color,
                        );
                    }
                    FieldState::Player2 => {
                        draw_rectangle_lines(
                            (rec_width * j as f32) + x_offset,
                            rec_height * i as f32 + y_offset,
                            rec_width,
                            rec_height,
                            10.0,
                            line_color,
                        );
                    }
                }
            }
        }

        //remove outer border
        draw_line(
            x_offset,
            y_offset,
            x_offset,
            rec_height * 3.0 + y_offset,
            10.0,
            background_color,
        );
        draw_line(
            x_offset,
            y_offset,
            rec_width * 3.0 + x_offset,
            y_offset,
            10.0,
            background_color,
        );
        draw_line(
            rec_width * 3.0 + x_offset,
            rec_height * 3.0 + y_offset,
            x_offset,
            rec_height * 3.0 + y_offset,
            10.0,
            background_color,
        );
        draw_line(
            rec_width * 3.0 + x_offset,
            rec_height * 3.0 + y_offset,
            rec_width * 3.0 + x_offset,
            y_offset,
            10.0,
            background_color,
        );
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game_state = GameState::default();

    loop {
        game_state.draw();

        next_frame().await;
        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);
    }
}
