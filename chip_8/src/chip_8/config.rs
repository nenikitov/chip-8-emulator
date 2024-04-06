pub struct Config {
    pub shift_ignores_vy: bool,
    pub jump_reads_from_vx: bool,
    pub store_load_modifies_i: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shift_ignores_vy: true,
            jump_reads_from_vx: false,
            store_load_modifies_i: false,
        }
    }
}
