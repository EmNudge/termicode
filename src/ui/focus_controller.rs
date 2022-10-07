use crossterm::event;

pub trait HandleInput {
    fn handle_input(&mut self, key_code: &event::KeyCode);
    fn is_disabled(&self) -> bool;
}

struct FocusComponent {
    component: Box::<dyn HandleInput>,
    key: String,
}

pub struct FocusController {
    focus_components: Vec<FocusComponent>,
    active_key: String,
}

impl FocusController {
    pub fn distribute_input(&mut self, key_code: &event::KeyCode) {
        if let Some(focus_component) = self.focus_components
            .into_iter()
            .find(|&focus_component| self.active_key == focus_component.key) {

            if !focus_component.component.is_disabled() {
                (*focus_component.component).handle_input(key_code);
            }
        }
    }
    pub fn add_component(&mut self, component: &dyn HandleInput, key: String) {
        self.focus_components.push(FocusComponent { key, component: Box::new(*component) });
    }
}
