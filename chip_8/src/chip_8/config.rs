/// Emulation compatibility configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Original interpreters copied `Vy` into `Vx` before byte shifting.
    /// Newer implementations shift Vx in place.
    ///
    /// `true` is most compatible.
    ///
    /// Affected instructions:
    /// * `8xy6`
    /// * `8xyE`
    pub shift_ignores_vy: bool,
    /// Original interpreters used `V0` for offset.
    /// Newer implementations use erroneously `Vx`.
    ///
    /// `false` is most compatible.
    ///
    /// Affected instructions:
    /// * `Bnnn`
    pub jump_reads_from_vx: bool,
    /// Newer implementations store an overflow flag when the memory pointer register is outside the valid range.
    ///
    /// `true` is most compatible.
    ///
    /// Affected instructions:
    /// * `Fx1E`
    pub add_to_index_stores_overflow: bool,
    /// Original interpreters incremented `I` while storing and loading memory from RAM.
    /// Newer implementations do it in place without modifying `I`.
    ///
    /// `false` is most compatible.
    ///
    /// Affected instructions:
    /// * `Fx55`
    /// * `Fx65`
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
