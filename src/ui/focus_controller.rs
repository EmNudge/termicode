pub trait HandleInput {
    pub fn handle_input(&mut self, key_code: &event::KeyCode);
    pub fn is_disabled(&self) -> bool;
}

struct FocusComponent {
    component: Box::new(impl HandleInput);
    key: String,
}
pub FocusController {
    focus_components: Vec<&FocusComponent>;
    active_key: String,
}

impl FocusController {
    pub fn distribute_input(&mut self, key_code: &event::KeyCode) {
        if let Some(focus_component) = self.focus_components
            .into_iter()
            .find(|&&focus_component| self.active_key == focus_component.key) {

            if !focus_component.component.is_disabled() {
                focus_component.component.handle_input(key_code);
            }
        }
    }
    pub fn add_component(&mut self, component: impl HandleInput, key: String) {
        self.focus_components.push_back(component);
    }
}
