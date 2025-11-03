/// TermiEmu - A modern, high-performance terminal emulator
///
/// Design Philosophy: "Fluid Minimalism with Ruthless Performance"
///
/// This is the entry point for the TermiEmu terminal emulator.
/// Phase 0 (Pre-Alpha) - Foundation stage with basic UI.
use termiemu::{logging, ui::TermiEmuApp};
use tracing::info;

fn main() -> iced::Result {
    // Initialize logging infrastructure
    logging::init();

    info!("TermiEmu v{} starting", env!("CARGO_PKG_VERSION"));
    info!("Design Philosophy: Fluid Minimalism with Ruthless Performance");
    info!("Phase 0 (Pre-Alpha) - Foundation with basic UI");

    // Run the Iced application
    iced::application(TermiEmuApp::title, TermiEmuApp::update, TermiEmuApp::view)
        .run_with(TermiEmuApp::new)
}
