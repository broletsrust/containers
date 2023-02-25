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

    let mut game = Game::new();

    loop {
        game.update();
        terminal.draw(|f| ui(f, &game))?;

        if !event::poll(Duration::from_millis(20))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    break;
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

fn ui<B: Backend>(f: &mut Frame<B>, game: &Game) {
    for con in game.containers.iter() {
        let block = Block::default()
            .style(Style::bg(Default::default(), con.color));
        let rect = Rect::new(con.pos.0 * 7 + f.size().width / 2 - 5 * 7, con.pos.1 * 3 + con.extra_fall_height + f.size().height - 15 * 3, 7, 3);
        f.render_widget(block, rect);
    }

    let label_1 = Label::default().text(" o ");
    f.render_widget(label_1, Rect::new(game.player.pos.0 * 7 + f.size().width / 2 - 5 * 7, game.player.pos.1 * 3 + f.size().height - 15 * 3, 7, 1));
    let label_2 = Label::default().text("-|-");
    f.render_widget(label_2, Rect::new(game.player.pos.0 * 7 + f.size().width / 2 - 5 * 7, game.player.pos.1 * 3 + 1 + f.size().height - 15 * 3, 7, 1));
    let label_3 = Label::default().text("/ \\");
    f.render_widget(label_3, Rect::new(game.player.pos.0 * 7 + f.size().width / 2 - 5 * 7, game.player.pos.1 * 3 + 2 + f.size().height - 15 * 3, 7, 1));
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
