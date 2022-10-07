mod app;
mod focus_controller;

use app::{App, CursorMove};

use crate::data::{UnicodeData, UnicodeFile};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use unicode_width::UnicodeWidthChar;

fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    {
        let header_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(50),
                    Constraint::Length(1),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(chunks[0]);

        let search_box = Paragraph::new(app.search_box.get_rendered_input())
            .block(Block::default().borders(Borders::ALL).title("Search Box"));

        let options_block = Paragraph::new(" None for now...")
            .style(Style::default().fg(Color::DarkGray))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .title("Options"),
            );

        f.render_widget(search_box, header_chunks[0]);
        f.render_widget(options_block, header_chunks[2]);
    }

    {
        let search_results: Vec<ListItem> = app
            .results
            .iter()
            .map(|unicode_data| {
                let pad_char = |c: char, len: usize| {
                    let mut padding = String::new();
                    for _ in 0..len {
                        padding.push(c);
                    }
                    padding
                };

                let symbol_char = char::from_u32(unicode_data.codepoint).unwrap_or(' ');
                let symbol_right_padding = {
                    let symbol_len = UnicodeWidthChar::width(symbol_char).unwrap_or(0);
                    pad_char(' ', 4 - symbol_len)
                };
                let codepoint_right_padding = {
                    let num_len =
                        (f64::log2(unicode_data.codepoint as f64 + 1.0) / f64::log2(16.0)).ceil();

                    pad_char(' ', 6 - num_len as usize)
                };

                ListItem::new(Spans::from(vec![
                    Span::raw(symbol_char.to_string()),
                    Span::raw(symbol_right_padding),
                    Span::raw(format!("(U+{:x})", unicode_data.codepoint)),
                    Span::raw(codepoint_right_padding),
                    Span::styled(&unicode_data.name, Style::default().fg(Color::Cyan)),
                ]))
            })
            .take(f.size().height as usize)
            .collect();
        let search_results_block = List::new(search_results)
            .block(Block::default().borders(Borders::ALL).title("Results"))
            .highlight_style(Style::default().fg(Color::LightGreen));
        f.render_stateful_widget(search_results_block, chunks[1], app.get_list_state());
    }
}

fn draw_with_listener<B: Backend>(
    terminal: &mut Terminal<B>,
    data: &UnicodeFile,
) -> std::io::Result<Option<UnicodeData>> {
    let mut app = App::new(data);
    app.update_query();

    loop {
        terminal.draw(|f| draw(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(None),
                KeyCode::Enter => {
                    return Ok(app.get_selection().cloned());
                }
                KeyCode::Char(c) => {
                    app.search_box.add_char(c);
                    app.update_query();
                }
                KeyCode::Backspace => {
                    if key.modifiers.contains(KeyModifiers::ALT) {
                        app.search_box.delete_word();
                    } else {
                        app.search_box.delete_char();
                    }
                    app.update_query();
                }
                KeyCode::Down => {
                    app.selection_down();
                }
                KeyCode::Up => {
                    app.selection_up();
                }
                KeyCode::Right => {
                    app.search_box.move_cursor(CursorMove::Right);
                }
                KeyCode::Left => {
                    app.search_box.move_cursor(CursorMove::Left);
                }
                _ => {}
            }
        }
    }
}

pub fn create_interface(data: &UnicodeFile) -> Result<Option<UnicodeData>, std::io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let user_selection = draw_with_listener(&mut terminal, data)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(user_selection)
}
