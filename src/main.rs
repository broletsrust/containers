use std::{io, panic};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, Event, self, KeyCode}};
use game::Game;
use tui::{backend::{CrosstermBackend, Backend}, Terminal, Frame, layout::Rect, widgets::Block, style::{Style, Color}};

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
            .style(Style::bg(Default::default(), Color::Red));
        let rect = Rect::new(con.pos.0, con.pos.1, 7, 3);
        f.render_widget(block, rect);
    }
}
