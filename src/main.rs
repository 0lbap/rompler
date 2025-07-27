mod app;
mod ui;

use app::App;
use ratatui::Terminal;
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use ratatui::crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use ratatui::prelude::Backend;
use ratatui::prelude::CrosstermBackend;
use rodio::OutputStream;
use rodio::Source;
use std::error::Error;
use std::io;
use ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Left => {
                        app.prev_instrument();
                    }
                    KeyCode::Right => {
                        app.next_instrument();
                    }
                    KeyCode::Char(c) => {
                        if let Some(note) = app.char_to_note.get(&c) {
                            let current_instrument =
                                app.instruments.get(app.current_instrument_index).unwrap();
                            let source_for_speed = current_instrument.source.clone();
                            let note_map_value = app.note_map.get(note).unwrap().clone();
                            app.press_note(note.to_string());
                            let s = source_for_speed.speed(note_map_value as f32);
                            match stream_handle.play_raw(s.convert_samples()) {
                                Ok(_) => {}
                                Err(e) => return Err(Box::new(e)),
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
