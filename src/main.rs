use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{prelude::*, widgets::*};

struct App<'a> {
    state: TableState,
    items: Vec<Vec<&'a str>>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            state: TableState::default(),
            items: vec![vec!["system"], vec!["vim"], vec!["omz"]],
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

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
        terminal.draw(|f| ui(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Enter => return select(terminal, app),
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(f.size());

    let default_style = Style::default()
        .bg(Color::Rgb(246, 250, 142))
        .fg(Color::Black);

    let selected_style = Style::default()
        .bg(Color::Rgb(252, 138, 25))
        .fg(Color::Black);
    // .add_modifier(Modifier::REVERSED);
    // let normal_style = Style::default().bg(Color::Rgb(252, 138, 25));
    // let header_cells = ["Header1", "Header2", "Header3"]
    //     .iter()
    //     .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));
    // let header = Row::new(header_cells)
    //     .style(normal_style)
    //     .height(1)
    //     .bottom_margin(1);
    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).style(default_style) //.bottom_margin(1)
    });
    let t = Table::new(
        rows,
        [
            Constraint::Percentage(50),
            Constraint::Max(30),
            Constraint::Min(10),
        ],
    )
    // .header(header)
    .block(
        Block::default().borders(Borders::ALL).title("Jobs"), //.style(Style::default().bg(Color::Rgb(25, 26, 25))),
    )
    .highlight_style(selected_style);
    f.render_stateful_widget(t, rects[0], &mut app.state);
}

fn popup(f: &mut Frame, app: &App) {
    let size = f.size();

    let chunks = Layout::default()
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(size);

    // let text = if app.show_popup {
    //     "Press p to close the popup"
    // } else {
    //     "Press p to show the popup"
    // };
    let text = "Press p to show popup";
    let paragraph = Paragraph::new(text.slow_blink())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);

    let block = Block::default()
        .title("Content")
        .borders(Borders::ALL)
        .on_blue();
    f.render_widget(block, chunks[1]);

    let block = Block::default().title("Popup").borders(Borders::ALL);
    let area = centered_rect(60, 20, size);
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);
}
fn select<B: Backend>(terminal: &mut Terminal<B>, app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| popup(f, &app))?;
    }
}
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
