use macroquad::prelude::*;

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
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game_state = GameState::default();
    let screen_width_to_height = screen_height() / screen_width();

    set_fullscreen(true);

    loop {
        draw_fields(&game_state);
        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);
        draw_rectangle_lines(
            1.0,
            1.0,
            (screen_width() - 3.0) / 3.0 * screen_width_to_height,
            (screen_height() - 3.0) / 3.0,
            2.0,
            GREEN,
        );

        next_frame().await
    }
}

fn draw_fields(game_state: &GameState) {}
