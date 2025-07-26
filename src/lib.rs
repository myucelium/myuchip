pub use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(version, about)]
pub struct Args {
    /// Path to Chip-8 ROM
    rom_path: String,
}
