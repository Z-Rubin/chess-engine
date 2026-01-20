pub mod attacks;
pub mod perft;
pub mod movegen;

/// Initialize all attack tables. Call this before using the move generator.
pub fn init() {
    attacks::init_attack_tables();
}