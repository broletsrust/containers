use std::{io, panic, time::Duration};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, Event, self, KeyCode}};
use game::Game;
use tui::{backend::{CrosstermBackend, Backend}, Terminal, Frame, layout::Rect, widgets::{Block, Widget}, style::Style, buffer::Buffer};

mod game;

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let original_hook = panic::take_hook();

    panic::set_hook(Box::new(move |info| {
        // restore terminal
        disable_raw_mode().unwrap();
        execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture
        ).unwrap();

        original_hook(info);
    }));

    if terminal.size()?.width < 70 || terminal.size()?.height < 46 {
        panic!("terminal not big enough");
    }

    let state = State::Playing;
    let mut game = Game::new();

    loop {
        if state == State::Playing {
            game.update();
        }

        terminal.draw(|f| ui(f, &state, &game))?;

        if !event::poll(Duration::from_millis(20))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Left => {
                    game.player.move_left();
                }
                KeyCode::Right => {
                    game.player.move_right();
                }
                KeyCode::Up => {
                    game.player.jump();
                }
                _ => {}
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, state: &State, game: &Game) {
    match state {
        State::Playing => {
            for con in game.containers.iter() {
                let block = Block::default()
                    .style(Style::bg(Default::default(), con.color));
                let rect = Rect::new(con.pos.0 * 7 + f.size().width / 2 - 5 * 7, con.pos.1 * 3 + con.extra_fall_height + f.size().height - 15 * 3, 7, 3);
                f.render_widget(block, rect);
            }

            let label_1 = Label::default().text(" o ");
            f.render_widget(label_1, Rect::new(game.player.pos.0 * 7 + game.player.extra.0 + f.size().width / 2 - 5 * 7, game.player.pos.1 * 3 + game.player.extra.1 + f.size().height - 15 * 3, 7, 1));
            let label_2 = Label::default().text("-|-");
            f.render_widget(label_2, Rect::new(game.player.pos.0 * 7 + game.player.extra.0 + f.size().width / 2 - 5 * 7, game.player.pos.1 * 3 + 1 + game.player.extra.1 + f.size().height - 15 * 3, 7, 1));
            let label_3 = Label::default().text("/ \\");
            f.render_widget(label_3, Rect::new(game.player.pos.0 * 7 + game.player.extra.0 + f.size().width / 2 - 5 * 7, game.player.pos.1 * 3 + 2 + game.player.extra.1 + f.size().height - 15 * 3, 7, 1));

            let points_text = format!("Points: {}", game.points);
            let point_label = RightToLeftLabel::default().text(&*points_text);
            f.render_widget(point_label, Rect::new(f.size().width / 2 + 24, f.size().height - 15 * 3 - 1, 11, 1));

            if game.over {
                let label = Label::default().text("Game Over!");
                f.render_widget(label, Rect::new(f.size().width / 2 - 5, f.size().height - 15 * 3 - 1, 10, 1));
            }
        }
        State::Menu => {

        }
    }
}

#[derive(PartialEq)]
enum State {
    Menu,
    Playing,
}

#[derive(Default)]
struct Label<'a> {
    text: &'a str,
}

impl<'a> Widget for Label<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), self.text, Style::default());
    }
}

impl<'a> Label<'a> {
    fn text(mut self, text: &'a str) -> Label<'a> {
        self.text = text;
        self
    }
}

#[derive(Default)]
struct RightToLeftLabel<'a> {
    text: &'a str,
}

impl<'a> Widget for RightToLeftLabel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.right() - self.text.len() as u16, area.top(), self.text, Style::default());
    }
}

impl<'a> RightToLeftLabel<'a> {
    fn text(mut self, text: &'a str) -> RightToLeftLabel<'a> {
        self.text = text;
        self
    }
}
