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
                [FieldState::None, FieldState::Player1, FieldState::None],
                [FieldState::Player2, FieldState::None, FieldState::None],
            ],
        }
    }

    fn draw(&mut self) {
        let offset = 50.0;
        let inner_offset = 20.0;
        let line_width = 5.0;
        let rec_width = (screen_width() - offset * 2.0) / 3.0 * screen_height() / screen_width();
        let rec_height = (screen_height() - offset * 2.0) / 3.0;
        let x_offset = (screen_width() - offset - rec_width * 3.0) / 2.0;
        let y_offset = offset;
        let background_color = BLACK;
        let line_color = WHITE;
        let (selected_x, selected_y) = self.selected;

        for (i, row) in self.field_states.iter().enumerate() {
            for (j, field) in row.iter().enumerate() {
                let x = (rec_width * j as f32) + x_offset;
                let y = rec_height * i as f32 + y_offset;
                let selected = selected_x == j as u32 && selected_y == i as u32;

                //Reverse Colors For Selected Field
                let (background_color, line_color) = if selected {
                    (line_color, background_color)
                } else {
                    (background_color, line_color)
                };

                match selected {
                    true => draw_rectangle(x, y, rec_width, rec_height, background_color),
                    false => {
                        draw_rectangle_lines(x, y, rec_width, rec_height, line_width, line_color)
                    }
                }

                match field {
                    FieldState::Player1 => {
                        draw_circle_lines(
                            x + rec_width * 0.5,
                            y + rec_height * 0.5,
                            rec_width / 2.0 - inner_offset,
                            line_width,
                            line_color,
                        );
                    }
                    FieldState::Player2 => {
                        draw_line(
                            x + inner_offset / 2.0,
                            y + inner_offset / 2.0,
                            x + rec_width - inner_offset / 2.0,
                            y + rec_height - inner_offset / 2.0,
                            line_width,
                            line_color,
                        );
                        draw_line(
                            x + inner_offset / 2.0,
                            y + rec_height - inner_offset / 2.0,
                            x + rec_width - inner_offset / 2.0,
                            y + inner_offset / 2.0,
                            line_width,
                            line_color,
                        );
                    }
                    _ => (),
                }
            }
        }

        //Remove Outer Border
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

    fn select_right(&mut self) {
        let (x, y) = self.selected;
        self.selected = if x < 2 { (x + 1, y) } else { (x, y) };
    }

    fn select_left(&mut self) {
        let (x, y) = self.selected;
        self.selected = if x > 0 { (x - 1, y) } else { (x, y) };
    }

    fn select_up(&mut self) {
        let (x, y) = self.selected;
        self.selected = if y > 0 { (x, y - 1) } else { (x, y) };
    }

    fn select_down(&mut self) {
        let (x, y) = self.selected;
        self.selected = if y < 2 { (x, y + 1) } else { (x, y) };
    }

    //TODO make auto brake
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game_state = GameState::default();

    loop {
        game_state.draw();

        if is_key_pressed(KeyCode::Right) {
            game_state.select_right();
        }
        if is_key_pressed(KeyCode::Left) {
            game_state.select_left();
        }
        if is_key_pressed(KeyCode::Up) {
            game_state.select_up();
        }
        if is_key_pressed(KeyCode::Down) {
            game_state.select_down();
        }

        next_frame().await;
    }
}
