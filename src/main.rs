#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

mod config;
mod hotkeys;
mod macros;
mod ui;

use ui::MacroApp;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 600.0])
            .with_resizable(false)
            .with_icon(load_icon()),
        ..Default::default()
    };
    
    eframe::run_native(
        "Moonwalk Macros",
        options,
        Box::new(|cc| {
            // Set dark theme
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            
            // Create app
            let app = MacroApp::new(cc);
            Ok(Box::new(app))
        }),
    )
}

fn load_icon() -> egui::IconData {
    let icon_data = include_bytes!("../favicon.ico");
    
    // Parse ICO file to extract the largest icon
    if let Ok(icon) = image::load_from_memory_with_format(icon_data, image::ImageFormat::Ico) {
        let rgba_image = icon.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let rgba = rgba_image.into_raw();
        
        egui::IconData {
            rgba,
            width: width as u32,
            height: height as u32,
        }
    } else {
        // Fallback icon if loading fails
        egui::IconData {
            rgba: vec![255u8; 32 * 32 * 4], // 32x32 white square
            width: 32,
            height: 32,
        }
    }
}
