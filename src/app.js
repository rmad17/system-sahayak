
use anyhow::{Context, Result};


struct App {
    state: TableState,
    items: Vec<Vec<&'a str>>,
}

impl App {
    fn new() -> Result<Self> {
        Ok(Self {
            state: TableState::default(),
            items: vec![
                vec!["system"],
                vec!["vim"],
                vec!["omz"],
            ],
        })
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
