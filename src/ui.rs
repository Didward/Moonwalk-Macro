use eframe::egui;
use crate::config::{Config, ShiftlockKey};
use crate::macros::MacroExecutor;
use crate::hotkeys::{HotkeyManager, HotkeyEvent};

pub struct MacroApp {
    config: Config,
    status_message: String,
    status_type: StatusType,
    hotkey_manager: Option<HotkeyManager>,
    capturing_hotkey: Option<HotkeyCapture>,
    active_hotkeys: String,
}

#[derive(Debug, Clone)]
enum StatusType {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
enum HotkeyCapture {
    Com,
    Clip,
}

impl MacroApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let hotkey_manager = HotkeyManager::new().ok();
        
        Self {
            config: Config::default(),
            status_message: "Ready".to_string(),
            status_type: StatusType::Info,
            hotkey_manager,
            capturing_hotkey: None,
            active_hotkeys: "None".to_string(),
        }
    }
    
    fn set_status(&mut self, message: &str, status_type: StatusType) {
        self.status_message = message.to_string();
        self.status_type = status_type;
    }
    
    fn execute_com_offset(&mut self) {
        let config = self.config.clone();
        
        match MacroExecutor::new() {
            Ok(mut executor) => {
                match executor.execute_com_offset(&config) {
                    Ok(()) => self.set_status("COM Offset executed successfully", StatusType::Success),
                    Err(e) => self.set_status(&format!("COM Offset failed: {}", e), StatusType::Error),
                }
            }
            Err(e) => self.set_status(&format!("Failed to initialize macro executor: {}", e), StatusType::Error),
        }
    }
    
    fn execute_wall_clip(&mut self) {
        let config = self.config.clone();
        
        match MacroExecutor::new() {
            Ok(mut executor) => {
                match executor.execute_wall_clip(&config) {
                    Ok(()) => self.set_status("Wall Clip executed successfully", StatusType::Success),
                    Err(e) => self.set_status(&format!("Wall Clip failed: {}", e), StatusType::Error),
                }
            }
            Err(e) => self.set_status(&format!("Failed to initialize macro executor: {}", e), StatusType::Error),
        }
    }
    
    fn register_hotkeys(&mut self) {
        if let Some(ref mut manager) = self.hotkey_manager {
            match manager.register_hotkeys(&self.config.com_hotkey, &self.config.clip_hotkey) {
                Ok(()) => {
                    self.active_hotkeys = format!("COM: {}, Clip: {}", self.config.com_hotkey, self.config.clip_hotkey);
                    self.set_status("Hotkeys registered successfully", StatusType::Success);
                }
                Err(e) => self.set_status(&format!("Failed to register hotkeys: {}", e), StatusType::Error),
            }
        } else {
            self.set_status("Hotkey manager not available", StatusType::Error);
        }
    }
}

impl eframe::App for MacroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard input for hotkey capture
        ctx.input(|i| {
            if let Some(capture) = &self.capturing_hotkey {
                for event in &i.events {
                    if let egui::Event::Key { key, pressed: true, .. } = event {
                        let key_name = format!("{:?}", key).to_lowercase();
                        match capture {
                            HotkeyCapture::Com => {
                                self.config.com_hotkey = key_name;
                                self.set_status("COM hotkey updated", StatusType::Success);
                            }
                            HotkeyCapture::Clip => {
                                self.config.clip_hotkey = key_name;
                                self.set_status("Clip hotkey updated", StatusType::Success);
                            }
                        }
                        self.capturing_hotkey = None;
                        return;
                    }
                }
            }
        });

        // Check for hotkey events
        if let Some(ref manager) = self.hotkey_manager {
            for event in manager.check_events() {
                match event {
                    HotkeyEvent::ComOffset => self.execute_com_offset(),
                    HotkeyEvent::WallClip => self.execute_wall_clip(),
                }
            }
        }
        
        // Minimalist dark and white color scheme
        let _bg_color = egui::Color32::from_rgb(0, 0, 0);  // Pure black
        let accent_color = egui::Color32::WHITE;  // Pure white
        let success_color = egui::Color32::from_rgb(0, 255, 0);  // Green
        let error_color = egui::Color32::from_rgb(255, 0, 0);  // Red
        let warning_color = egui::Color32::from_rgb(255, 255, 0);  // Yellow
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                // Title
                ui.add_space(20.0);
                ui.heading(egui::RichText::new("Moonwalk Macros")
                    .size(24.0)
                    .color(accent_color)
                    .strong());
                ui.add_space(20.0);
                
                // Settings Section
                ui.group(|ui| {
                    ui.set_min_width(440.0);
                    ui.label(egui::RichText::new("Settings").size(14.0).strong());
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        ui.label("Shiftlock Key:");
                        ui.radio_value(&mut self.config.shiftlock_key, ShiftlockKey::Shift, "Left Shift");
                        ui.radio_value(&mut self.config.shiftlock_key, ShiftlockKey::Control, "Control");
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Moonwalk Emote Slot (1-8):");
                        ui.add(egui::DragValue::new(&mut self.config.emote_slot)
                            .range(1..=8)
                            .speed(0.1));
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Gear/Item Slot (1-9,0):");
                        ui.add(egui::TextEdit::singleline(&mut self.config.gear_slot)
                            .desired_width(40.0)
                            .char_limit(1));
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Peak Delay (seconds):");
                        ui.add(egui::DragValue::new(&mut self.config.peak_delay)
                            .range(0.05..=5.0)
                            .speed(0.01)
                            .fixed_decimals(2));
                    });
                    
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.config.unequip_after, "Unequip item after");
                        ui.add_space(20.0);
                        ui.checkbox(&mut self.config.unshiftlock_after, "Unshiftlock after wall clip");
                    });
                });
                
                ui.add_space(15.0);
                
                // Hotkeys Section
                ui.group(|ui| {
                    ui.set_min_width(440.0);
                    ui.label(egui::RichText::new("Hotkeys").size(14.0).strong());
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        ui.label("COM Hotkey:");
                        ui.add(egui::TextEdit::singleline(&mut self.config.com_hotkey)
                            .desired_width(80.0));
                        
                        if ui.button("Set").clicked() {
                            self.capturing_hotkey = Some(HotkeyCapture::Com);
                            self.set_status("Press any key to set COM hotkey...", StatusType::Info);
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Clip Hotkey:");
                        ui.add(egui::TextEdit::singleline(&mut self.config.clip_hotkey)
                            .desired_width(80.0));
                        
                        if ui.button("Set").clicked() {
                            self.capturing_hotkey = Some(HotkeyCapture::Clip);
                            self.set_status("Press any key to set Clip hotkey...", StatusType::Info);
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("Apply Hotkeys").clicked() {
                            self.register_hotkeys();
                        }
                        ui.label(format!("Active: {}", self.active_hotkeys));
                    });
                });
                
                ui.add_space(15.0);
                
                // Action Buttons
                ui.group(|ui| {
                    ui.set_min_width(440.0);
                    ui.label(egui::RichText::new("Actions").size(14.0).strong());
                    ui.separator();
                    
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.horizontal(|ui| {
                            ui.add_space((440.0 - 380.0) / 2.0); // Center the buttons
                            if ui.add_sized([180.0, 40.0], egui::Button::new("Run COM Offset")).clicked() {
                                self.execute_com_offset();
                            }
                            
                            ui.add_space(20.0);
                            
                            if ui.add_sized([180.0, 40.0], egui::Button::new("Run Wall Clip")).clicked() {
                                self.execute_wall_clip();
                            }
                        });
                        
                        ui.add_space(10.0);
                        
                        if ui.add_sized([100.0, 30.0], egui::Button::new("Quit")).clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
                
                ui.add_space(20.0);
                
                // Status Bar
                ui.separator();
                ui.horizontal(|ui| {
                    let status_color = match self.status_type {
                        StatusType::Success => success_color,
                        StatusType::Error => error_color,
                        StatusType::Warning => warning_color,
                        StatusType::Info => egui::Color32::WHITE,
                    };
                    
                    ui.label(egui::RichText::new("Status:").strong());
                    ui.label(egui::RichText::new(&self.status_message).color(status_color));
                });
            });
        });
        
        // Request repaint for hotkey checking
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}
