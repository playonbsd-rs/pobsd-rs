use crossterm::event::{Event, Event::Key, KeyCode};
use pobsd_parser::Game;
use tui::widgets::ListState;

pub(crate) enum AppStatus {
    Continue,
    Close,
}

#[derive(Debug, PartialEq)]
pub(crate) enum SearchMode {
    Name,
    Tag,
    Genre,
}

#[derive(Debug, PartialEq)]
pub(crate) enum InputMode {
    Normal,
    Search(SearchMode),
}

pub(crate) struct AppState {
    pub(crate) mode: InputMode,
    pub(crate) games: Vec<Game>,
    pub(crate) list_state: ListState,
    pub(crate) search_text: String,
    pub(crate) search_list: Vec<Game>,
}

impl AppState {
    pub(crate) fn new() -> AppState {
        AppState {
            mode: InputMode::Normal,
            games: vec![],
            list_state: ListState::default(),
            search_text: String::new(),
            search_list: vec![],
        }
    }
    pub(crate) fn event_handler(&mut self, event: Event) -> AppStatus {
        if let Key(key) = event {
            match &self.mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return AppStatus::Close,
                    KeyCode::Char('s') => {
                        self.change_mode(InputMode::Search(SearchMode::Name));
                        self.list_state.select(None);
                    }
                    KeyCode::Up | KeyCode::Char('k') => self.move_up(),
                    KeyCode::Down | KeyCode::Char('j') => self.move_down(),
                    _ => {}
                },
                InputMode::Search(search_mode) => match key.code {
                    KeyCode::Esc => {
                        self.change_mode(InputMode::Normal);
                        self.search_text.clear();
                        self.list_state.select(None);
                    }
                    KeyCode::Char(c) => {
                        self.search_text.push(c);
                        self.search();
                    }
                    KeyCode::Backspace => {
                        self.search_text.pop();
                        self.search();
                    }
                    KeyCode::Tab => {
                        match search_mode {
                            SearchMode::Name => {
                                self.change_mode(InputMode::Search(SearchMode::Tag))
                            }
                            SearchMode::Tag => {
                                self.change_mode(InputMode::Search(SearchMode::Genre))
                            }
                            SearchMode::Genre => {
                                self.change_mode(InputMode::Search(SearchMode::Name))
                            }
                        }
                        self.search();
                    }
                    KeyCode::Up => self.move_up(),
                    KeyCode::Down => self.move_down(),
                    _ => {}
                },
            }
        }
        AppStatus::Continue
    }
    pub(crate) fn change_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }
    pub(crate) fn search(&mut self) {
        match &self.mode {
            InputMode::Search(search_mode) => match search_mode {
                SearchMode::Name => {
                    self.search_list = self
                        .games
                        .clone()
                        .into_iter()
                        .filter(|item| {
                            item.name
                                .to_lowercase()
                                .contains(&self.search_text.to_lowercase())
                        })
                        .collect();
                }
                SearchMode::Tag => {
                    self.search_list = self
                        .games
                        .clone()
                        .into_iter()
                        .filter(|item| match &item.tags {
                            Some(tags) => {
                                let tags = tags.join("|");
                                tags.to_lowercase()
                                    .contains(&self.search_text.to_lowercase())
                            }
                            None => false,
                        })
                        .collect();
                }
                SearchMode::Genre => {
                    self.search_list = self
                        .games
                        .clone()
                        .into_iter()
                        .filter(|item| match &item.genres {
                            Some(genres) => {
                                let genres = genres.join("|");
                                genres
                                    .to_lowercase()
                                    .contains(&self.search_text.to_lowercase())
                            }
                            None => false,
                        })
                        .collect();
                }
            },
            _ => unreachable!(),
        }
    }
    pub(crate) fn move_up(&mut self) {
        let len_list = match self.mode {
            InputMode::Search(_) => {
                if self.search_list.is_empty() {
                    self.games.len()
                } else {
                    self.search_list.len()
                }
            }
            _ => self.games.len(),
        };
        let selected = match self.list_state.selected() {
            Some(v) => {
                if len_list == 0 {
                    None
                } else if v == 0 {
                    Some(v)
                } else {
                    Some(v - 1)
                }
            }
            None => {
                if len_list == 0 {
                    None
                } else {
                    Some(0)
                }
            }
        };
        self.list_state.select(selected);
    }
    pub(crate) fn move_down(&mut self) {
        let len_list = match self.mode {
            InputMode::Search(_) => {
                if self.search_list.is_empty() {
                    self.games.len()
                } else {
                    self.search_list.len()
                }
            }
            _ => self.games.len(),
        };
        let selected = match self.list_state.selected() {
            Some(v) => {
                if len_list == 0 {
                    None
                } else if v >= len_list - 1 {
                    Some(len_list - 1)
                } else {
                    Some(v + 1)
                }
            }
            None => {
                if len_list == 0 {
                    None
                } else {
                    Some(0)
                }
            }
        };
        self.list_state.select(selected);
    }
}
#[cfg(test)]
mod test_app_states {
    use super::*;
    use pobsd_parser::Game;
    fn get_games() -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();
        let params = vec![
            ("AAa", "GenreA", "TagA"),
            ("Bbb", "GenreB", "TagB"),
            ("Ccc", "GenreC", "TagC"),
        ];
        for (name, genres, tags) in params {
            let mut game = Game::default();
            game.name = name.into();
            game.genres = Some(vec![genres.split(',').map(|a| a.trim()).collect()]);
            game.tags = Some(vec![tags.split(',').map(|a| a.trim()).collect()]);
            games.push(game)
        }
        games
    }
    #[test]
    fn test_app_state_change_mode_method() {
        let mut app_state = AppState::new();
        assert_eq!(app_state.mode, InputMode::Normal);
        app_state.change_mode(InputMode::Search(SearchMode::Name));
        assert_eq!(app_state.mode, InputMode::Search(SearchMode::Name));
    }
    #[test]
    fn test_app_state_search_method_in_name_mode() {
        let games = get_games();
        let mut app_state = AppState::new();
        app_state.mode = InputMode::Search(SearchMode::Name);
        app_state.games = games.clone();
        app_state.search_text = "Aaa".into();
        app_state.search();
        assert_eq!(app_state.search_list[0], games[0]);
        assert_eq!(app_state.search_list.len(), 1);
    }
}
