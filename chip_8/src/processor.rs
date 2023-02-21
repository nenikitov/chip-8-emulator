use crate::components::Components;

#[derive(Debug)]
pub struct Processor {
    components: Components
}

impl Processor {
    pub fn new() -> Self {
        Self {
            components: Components::new()
        }
    }
}

