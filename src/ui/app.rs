use tui::widgets::ListState;
use crate::data::{ UnicodeFile, UnicodeData };
use crate::query::query_name;

enum SelectionMove {
    UP,
    DOWN,
}
struct SearchSelection {
    list_state: ListState,
}

impl SearchSelection {
    pub fn new() -> SearchSelection {
        SearchSelection { list_state: ListState::default() }
    }
    pub fn selection_move(&mut self, results: &Vec<&UnicodeData>, direction: SelectionMove) {
        let len = results.len();
        if len == 0 {
            return;
        }

        self.list_state
            .select(match self.list_state.selected() {
                Some(i) => Some(match direction {
                    SelectionMove::DOWN => if i == len - 1 { 0 } else { i + 1 },
                    SelectionMove::UP => i.checked_sub(1).unwrap_or(len - 1),
                }),
                None => Some(0),
            });
    }
    pub fn get_selection<'a>(&self, results: &Vec<&'a UnicodeData>) -> Option<&'a UnicodeData> {
        let len = results.len();
        if len == 0 {
            return None;
        }

        let index = self.list_state.selected().unwrap_or(0);
        return Some(&results.get(index).unwrap());
    }
}

pub struct App<'a> {
    pub input: String,
    file: &'a UnicodeFile,
    search_selection: SearchSelection,
    pub results: Vec<&'a UnicodeData>,
}

impl<'a> App<'a> {
    pub fn new(file: &'a UnicodeFile) -> App<'a> {
        App {
            input: String::new(),
            file,
            search_selection: SearchSelection::new(),
            results: vec![],
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.input.push(c);
        self.update_query();
    }
    pub fn delete_char(&mut self) {
        self.input.pop();
        self.update_query();
    }
    pub fn delete_word(&mut self) {
        while let Some(_letter) = self.input.pop() {
            let last_char = self.input.chars().last();
            if let Some(letter) = last_char {
                if letter == ' ' {
                    break;
                }
            }
        }
    }

    pub fn update_query(&mut self) {
        self.results = query_name(self.input.clone(), self.file).take(20).collect();
    }

    pub fn selection_up(&mut self) {
        self.search_selection.selection_move(&self.results, SelectionMove::UP);
    }
    pub fn selection_down(&mut self) {
        self.search_selection.selection_move(&self.results, SelectionMove::DOWN);
    }
    pub fn get_selection(&'a self) -> Option<&'a UnicodeData> {
        self.search_selection.get_selection(&self.results)
    }
    pub fn get_list_state(&mut self) -> &mut ListState {
        &mut self.search_selection.list_state
    }
}