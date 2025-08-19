use macroquad::{color::rgb_to_hsl, miniquad::log, prelude::*};

#[derive(Debug, Clone)]
enum Player {
    Player1,
    Player2,
}
enum FieldState {
    Player(Player),
    None,
}

struct GameState {
    current_player: Player,
    field_states: [[FieldState; 3]; 3],
    selected: (usize, usize),
    win: Option<((usize, usize), (usize, usize))>,
}

impl Player {
    fn player_switch(&mut self) {
        match self {
            Player::Player1 => *self = Player::Player2,
            Player::Player2 => *self = Player::Player1,
        }
    }
}

impl GameState {
    fn default() -> Self {
        GameState {
            selected: (0, 0),
            current_player: Player::Player1,
            field_states: [
                [FieldState::None, FieldState::None, FieldState::None],
                [FieldState::None, FieldState::None, FieldState::None],
                [FieldState::None, FieldState::None, FieldState::None],
            ],
            win: None,
        }
    }

    fn draw(&mut self) {
        let offset = 50.0;
        let inner_offset = 30.0;
        let line_width = 10.0;
        let rec_width = (screen_width() - offset * 2.0) / 3.0 * screen_height() / screen_width();
        let rec_height = (screen_height() - offset * 2.0) / 3.0;
        let x_offset = (screen_width() - offset - rec_width * 3.0) / 2.0;
        let y_offset = offset;
        let base_color = Color::from_rgba(36, 39, 58, 255);
        let overlay_color = Color::from_rgba(128, 135, 162, 255);
        let primary_color = Color::from_rgba(202, 211, 245, 255);
        let player1_color = Color::from_rgba(198, 160, 246, 255);
        let player2_color = Color::from_rgba(138, 173, 244, 255);
        let (selected_x, selected_y) = self.selected;

        clear_background(Color::from_rgba(36, 39, 58, 1));

        for (i, row) in self.field_states.iter().enumerate() {
            for (j, field) in row.iter().enumerate() {
                let x = (rec_width * j as f32) + x_offset;
                let y = rec_height * i as f32 + y_offset;
                let selected = selected_x == j && selected_y == i;

                //Reverse Colors For Selected Field
                let (background_color, line_color) = if selected && !self.win.is_some() {
                    (primary_color, base_color)
                } else {
                    (base_color, primary_color)
                };

                match selected && !self.win.is_some() {
                    true => draw_rectangle(x, y, rec_width, rec_height, background_color),
                    false => {
                        draw_rectangle_lines(x, y, rec_width, rec_height, line_width, line_color)
                    }
                }

                match field {
                    FieldState::Player(Player::Player2) => {
                        draw_circle_lines(
                            x + rec_width * 0.5,
                            y + rec_height * 0.5,
                            rec_width / 2.0 - inner_offset,
                            line_width * 2.0,
                            player2_color,
                        );
                    }
                    FieldState::Player(Player::Player1) => {
                        draw_line(
                            x + inner_offset,
                            y + inner_offset,
                            x + rec_width - inner_offset,
                            y + rec_height - inner_offset,
                            line_width * 2.0,
                            player1_color,
                        );
                        draw_line(
                            x + inner_offset,
                            y + rec_height - inner_offset,
                            x + rec_width - inner_offset,
                            y + inner_offset,
                            line_width * 2.0,
                            player1_color,
                        );
                    }
                    _ => (),
                }

                match self.win {
                    Some(winning_fields) => {
                        let ((x1, y1), (x2, y2)) = winning_fields;
                        draw_line(
                            x1 as f32 * rec_width + x_offset + rec_width / 2.0,
                            y1 as f32 * rec_height + y_offset + rec_height / 2.0,
                            x2 as f32 * rec_width + x_offset + rec_width / 2.0,
                            y2 as f32 * rec_height + y_offset + rec_height / 2.0,
                            10.0,
                            line_color,
                        );
                    }
                    None => (),
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
            base_color,
        );
        draw_line(
            x_offset,
            y_offset,
            rec_width * 3.0 + x_offset,
            y_offset,
            10.0,
            base_color,
        );
        draw_line(
            rec_width * 3.0 + x_offset,
            rec_height * 3.0 + y_offset,
            x_offset,
            rec_height * 3.0 + y_offset,
            10.0,
            base_color,
        );
        draw_line(
            rec_width * 3.0 + x_offset,
            rec_height * 3.0 + y_offset,
            rec_width * 3.0 + x_offset,
            y_offset,
            10.0,
            base_color,
        );
    }

    fn select_move_right(&mut self) {
        let (x, y) = self.selected;
        self.selected = if x < 2 {
            (x + 1, y)
        } else {
            self.seceted_move_pagebreake()
        };
    }

    fn select_move_left(&mut self) {
        let (x, y) = self.selected;
        self.selected = if x > 0 {
            (x - 1, y)
        } else {
            self.seceted_move_pagebreake()
        };
    }

    fn select_move_up(&mut self) {
        let (x, y) = self.selected;
        self.selected = if y > 0 { (x, y - 1) } else { (x, y) };
    }

    fn select_move_down(&mut self) {
        let (x, y) = self.selected;
        self.selected = if y < 2 { (x, y + 1) } else { (x, y) };
    }

    fn seceted_move_pagebreake(&mut self) -> (usize, usize) {
        let (x, y) = self.selected;
        if x == 2 {
            if y < 2 { (0, y + 1) } else { (x, y) }
        } else {
            if y > 0 { (2, y - 1) } else { (x, y) }
        }
    }

    fn seceted_select(&mut self) {
        let (x, y) = self.selected;

        match self.field_states[y][x] {
            FieldState::None => {
                self.field_states[y][x] = FieldState::Player(self.current_player.clone());
                self.check_for_win();
                self.current_player.player_switch();
            }
            _ => (),
        }
    }

    fn check_for_win(&mut self) {
        let (x, y) = self.selected;
        let mut count = 0;

        //vertical
        for i in 0..3 {
            match self.field_states[i][x] {
                FieldState::Player(Player::Player1) => {
                    if self.current_player.clone() as usize == Player::Player1 as usize {
                        count += 1;
                    }
                }
                FieldState::Player(Player::Player2) => {
                    if self.current_player.clone() as usize == Player::Player2 as usize {
                        count += 1;
                    }
                }
                FieldState::None => {}
            }

            if count == 3 {
                println!("win");
                self.win = Some(((x, 0), (x, 2)));
                return;
            }
        }

        count = 0;

        //horizontal
        for i in 0..3 {
            match self.field_states[y][i] {
                FieldState::Player(Player::Player1) => {
                    if self.current_player.clone() as usize == Player::Player1 as usize {
                        count += 1;
                    }
                }
                FieldState::Player(Player::Player2) => {
                    if self.current_player.clone() as usize == Player::Player2 as usize {
                        count += 1;
                    }
                }
                FieldState::None => {}
            }
            if count == 3 {
                println!("win");
                self.win = Some(((0, y), (2, y)));
                return;
            }
        }

        count = 0;

        //0-0 -> 2-2 diagonal
        if x == y {
            for i in 0..3 {
                match self.field_states[i][i] {
                    FieldState::Player(Player::Player1) => {
                        if self.current_player.clone() as usize == Player::Player1 as usize {
                            count += 1;
                        }
                    }
                    FieldState::Player(Player::Player2) => {
                        if self.current_player.clone() as usize == Player::Player2 as usize {
                            count += 1;
                        }
                    }
                    FieldState::None => {}
                }
                if count == 3 {
                    println!("win");
                    self.win = Some(((0, 0), (i, i)));
                    return;
                }
            }
        }

        count = 0;

        //0-2 -> 2-0 diagonal
        if y % 3 + x % 3 == 2 {
            for i in 0..3 {
                match self.field_states[2 - i][i] {
                    FieldState::Player(Player::Player1) => {
                        if self.current_player.clone() as usize == Player::Player1 as usize {
                            count += 1;
                        }
                    }
                    FieldState::Player(Player::Player2) => {
                        if self.current_player.clone() as usize == Player::Player2 as usize {
                            count += 1;
                        }
                    }
                    FieldState::None => {}
                }

                if count == 3 {
                    println!("win");
                    self.win = Some(((0, i), (i, 0)));
                    return;
                }
            }
        }
    }
}

#[macroquad::main("Tic Tac Rust")]
async fn main() {
    let mut game_state = GameState::default();
    let mut key_down = false;

    //Game Loop
    loop {
        game_state.draw();

        if get_keys_released().len() > 0 {
            key_down = false;
        } else if !key_down {
            for key in get_keys_down() {
                key_down = true;

                match key {
                    KeyCode::Right | KeyCode::L => game_state.select_move_right(),
                    KeyCode::Left | KeyCode::H => game_state.select_move_left(),
                    KeyCode::Up | KeyCode::K => game_state.select_move_up(),
                    KeyCode::Down | KeyCode::J => game_state.select_move_down(),
                    KeyCode::Enter | KeyCode::I => game_state.seceted_select(),
                    _ => (),
                }
            }
        }

        next_frame().await;
    }
}
