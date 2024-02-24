use std::process::Command;
use std::process::Stdio;
use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{prelude::*, widgets::*};

mod app;
use app::App;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw_ui(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Enter => app.show_popup = !app.show_popup,
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    _ => {}
                }
            }
        }
    }
}

fn draw_ui(f: &mut Frame, app: &mut App) {
    let size = f.size();
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(size);

    let default_style = Style::default()
        .bg(Color::Rgb(246, 250, 142))
        .fg(Color::Black);

    let popup_style = Style::default().bg(Color::Rgb(3, 46, 64)).fg(Color::White);

    let selected_style = Style::default()
        .bg(Color::Rgb(252, 138, 25))
        .fg(Color::Black);
    // let rows = app.items.iter().map(|item| {
    //     let height = item
    //         .iter()
    //         .map(|content| content.chars().filter(|c| *c == '\n').count())
    //         .max()
    //         .unwrap_or(0)
    //         + 1;
    //     let cells = item.iter().map(|c| Cell::from(*c));
    //     Row::new(cells).height(height as u16).style(default_style) //.bottom_margin(1)
    // });
    let list = List::new(app.items)
        // .header(header)
        .block(
            Block::default().borders(Borders::ALL).title("Jobs"), //.style(Style::default().bg(Color::Rgb(25, 26, 25))),
        )
        .highlight_style(selected_style);
    f.render_stateful_widget(list, rects[0], &mut app.state);

    if app.show_popup {
        let block = Block::default()
            .title("Running task: Command")
            .borders(Borders::ALL)
            .style(popup_style);
        let area = centered_rect(70, 50, size);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(block, area);
    }
}

fn draw_text() {}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
