use std::{
    io::stdout,
    ptr::with_exposed_provenance,
    time::{Duration, Instant},
};

use color_eyre::{Result, owo_colors::OwoColorize};
use crossterm::{
    ExecutableCommand,
    event::{DisableMouseCapture, EnableMouseCapture, KeyEvent, KeyEventKind, KeyModifiers},
};
use itertools::Itertools;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, MouseEventKind},
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Stylize},
    symbols::Marker,
    widgets::{
        Block, Widget,
        canvas::{self, Canvas, Circle, Map, MapResolution, Points, Rectangle},
    },
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    marker: Marker,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let verticat_ratio = 2;
        let canvas_width = 99;
        let canvas_height = canvas_width / verticat_ratio;

        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
        let horizontal = Layout::horizontal([Constraint::Percentage(100)]).spacing(1);
        let [top, main] = vertical.areas(frame.area());

        frame.render_widget(self.boxes_canvas(main, canvas_width, canvas_height), main);
    }

    fn boxes_canvas(&self, area: Rect, canvas_width: u16, canvas_height: u16) -> impl Widget {
        let left = 0.0;
        let right = f64::from(area.width);
        let bottom = 0.0;
        let top = f64::from(area.height);

        Canvas::default()
            .block(Block::bordered().title("Rects"))
            .marker(self.marker)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(move |ctx| {
                let rec_width = canvas_width / 6;
                let rec_height = canvas_height / 6;

                for i in 0..3 {
                    for j in 0..3 {
                        ctx.draw(&Rectangle {
                            x: f64::from(i * rec_width),
                            y: f64::from(j * rec_height),
                            width: f64::from(rec_width),
                            height: f64::from(rec_height),
                            color: Color::White,
                        });

                        if j == 1 && i == 1 {
                            ctx.draw(&Circle {
                                x: f64::from(i * rec_width + rec_width / 2),
                                y: f64::from(j * rec_height + rec_height / 2),
                                radius: f64::from(rec_height) * 0.4,
                                color: Color::White,
                            });
                        }
                    }
                }
            })
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
