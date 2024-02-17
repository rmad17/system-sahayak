use ratatui::widgets::ListState;

pub struct App<'a> {
    pub state: ListState,
    pub items: Vec<&'a str>,
    pub show_popup: bool,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            state: ListState::default(),
            items: vec!["system", "vim", "omz"],
            show_popup: false,
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
