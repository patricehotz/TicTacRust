use macroquad::prelude::*;

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
                let selected = selected_x == j && selected_y == i;

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
                    FieldState::Player(Player::Player2) => {
                        draw_circle_lines(
                            x + rec_width * 0.5,
                            y + rec_height * 0.5,
                            rec_width / 2.0 - inner_offset,
                            line_width,
                            line_color,
                        );
                    }
                    FieldState::Player(Player::Player1) => {
                        draw_line(
                            x + inner_offset,
                            y + inner_offset,
                            x + rec_width - inner_offset,
                            y + rec_height - inner_offset,
                            line_width,
                            line_color,
                        );
                        draw_line(
                            x + inner_offset,
                            y + rec_height - inner_offset,
                            x + rec_width - inner_offset,
                            y + inner_offset,
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

    fn check_for_win(&self) {
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
        }
        if count == 3 {
            println!("win");
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
        }
        if count == 3 {
            println!("win");
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
            }
        }
        if count == 3 {
            println!("win");
        }

        count = 0;

        //0-2 -> 2-0 diagonal
        if y % 3 + x % 3 == 2 {
            for i in 0..3 {
                match self.field_states[2 - i][i] {
                    FieldState::Player(Player::Player1) => {
                        if self.current_player.clone() as usize == Player::Player1 as usize {
                            count += 1;
                            println!("{count}")
                        }
                    }
                    FieldState::Player(Player::Player2) => {
                        if self.current_player.clone() as usize == Player::Player2 as usize {
                            count += 1;
                        }
                    }
                    FieldState::None => {}
                }
            }
        }

        if count == 3 {
            println!("win");
        }
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game_state = GameState::default();

    //Game Loop
    loop {
        game_state.draw();

        if is_key_pressed(KeyCode::Right) {
            game_state.select_move_right();
        }
        if is_key_pressed(KeyCode::Left) {
            game_state.select_move_left();
        }
        if is_key_pressed(KeyCode::Up) {
            game_state.select_move_up();
        }
        if is_key_pressed(KeyCode::Down) {
            game_state.select_move_down();
        }
        if is_key_pressed(KeyCode::Enter) {
            game_state.seceted_select();
        }

        next_frame().await;
    }
}
