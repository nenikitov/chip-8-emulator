#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub shift_ignores_vy: bool,
    pub jump_reads_from_vx: bool,
    pub add_to_index_stores_overflow: bool,
    pub store_load_modifies_i: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shift_ignores_vy: true,
            jump_reads_from_vx: false,
            add_to_index_stores_overflow: true,
            store_load_modifies_i: false,
        }
    }
}
