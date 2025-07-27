use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Sampling \"piano.wav\". Press <ESC> to quit.",
        Style::default(),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let piano_key_c = piano_key("c".to_string(), app, PianoKeyType::White);
    let piano_key_cs = piano_key("c#".to_string(), app, PianoKeyType::Black);
    let piano_key_cs_spacer = piano_key("c#".to_string(), app, PianoKeyType::Spacer);
    let piano_key_d = piano_key("d".to_string(), app, PianoKeyType::White);
    let piano_key_ds = piano_key("d#".to_string(), app, PianoKeyType::Black);
    let piano_key_ds_spacer = piano_key("c#".to_string(), app, PianoKeyType::Spacer);
    let piano_key_e = piano_key("e".to_string(), app, PianoKeyType::White);
    let piano_key_f = piano_key("f".to_string(), app, PianoKeyType::White);
    let piano_key_fs = piano_key("f#".to_string(), app, PianoKeyType::Black);
    let piano_key_fs_spacer = piano_key("c#".to_string(), app, PianoKeyType::Spacer);
    let piano_key_g = piano_key("g".to_string(), app, PianoKeyType::White);
    let piano_key_gs = piano_key("g#".to_string(), app, PianoKeyType::Black);
    let piano_key_gs_spacer = piano_key("c#".to_string(), app, PianoKeyType::Spacer);
    let piano_key_a = piano_key("a".to_string(), app, PianoKeyType::White);
    let piano_key_as = piano_key("a#".to_string(), app, PianoKeyType::Black);
    let piano_key_as_spacer = piano_key("c#".to_string(), app, PianoKeyType::Spacer);
    let piano_key_b = piano_key("b".to_string(), app, PianoKeyType::White);

    let piano_block = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 12),
            Constraint::Ratio(1, 12),
            Constraint::Ratio(1, 12),
            Constraint::Ratio(1, 12),
            Constraint::Ratio(1, 12),
            Constraint::Ratio(1, 12),
            Constraint::Ratio(1, 12),
            Constraint::Ratio(1, 12),
            Constraint::Ratio(1, 12),
            Constraint::Ratio(1, 12),
            Constraint::Ratio(1, 12),
            Constraint::Ratio(1, 12),
        ])
        .split(chunks[1]);

    let piano_key_cs_block = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(piano_block[1]);

    let piano_key_ds_block = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(piano_block[3]);

    let piano_key_fs_block = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(piano_block[6]);

    let piano_key_gs_block = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(piano_block[8]);

    let piano_key_as_block = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(piano_block[10]);

    frame.render_widget(piano_key_c, piano_block[0]);
    frame.render_widget(piano_key_cs, piano_key_cs_block[0]);
    frame.render_widget(piano_key_cs_spacer, piano_key_cs_block[1]);
    frame.render_widget(piano_key_d, piano_block[2]);
    frame.render_widget(piano_key_ds, piano_key_ds_block[0]);
    frame.render_widget(piano_key_ds_spacer, piano_key_ds_block[1]);
    frame.render_widget(piano_key_e, piano_block[4]);
    frame.render_widget(piano_key_f, piano_block[5]);
    frame.render_widget(piano_key_fs, piano_key_fs_block[0]);
    frame.render_widget(piano_key_fs_spacer, piano_key_fs_block[1]);
    frame.render_widget(piano_key_g, piano_block[7]);
    frame.render_widget(piano_key_gs, piano_key_gs_block[0]);
    frame.render_widget(piano_key_gs_spacer, piano_key_gs_block[1]);
    frame.render_widget(piano_key_a, piano_block[9]);
    frame.render_widget(piano_key_as, piano_key_as_block[0]);
    frame.render_widget(piano_key_as_spacer, piano_key_as_block[1]);
    frame.render_widget(piano_key_b, piano_block[11]);
}

fn piano_key(key: String, app: &App, key_type: PianoKeyType) -> Paragraph<'_> {
    let notes_pressed = app.shared_notes_pressed.lock().unwrap();
    let style = match key_type {
        PianoKeyType::White => match notes_pressed.get(&key).unwrap() {
            true => Style::default().bg(Color::DarkGray).fg(Color::White),
            false => Style::default().bg(Color::White).fg(Color::DarkGray),
        },
        PianoKeyType::Black => match notes_pressed.get(&key).unwrap() {
            true => Style::default().bg(Color::DarkGray).fg(Color::Black),
            false => Style::default().bg(Color::Black).fg(Color::DarkGray),
        },
        PianoKeyType::Spacer => Style::default().bg(Color::White).fg(Color::White),
    };

    let block = Block::default().style(style);

    let text = Paragraph::new(Text::styled(format!("{}", key.to_uppercase()), style)).block(block);

    text
}

enum PianoKeyType {
    White,
    Black,
    Spacer,
}
