use global_hotkey::{GlobalHotKeyManager, HotKeyState, GlobalHotKeyEvent, hotkey::{HotKey, Code}};
use crossbeam_channel::Receiver;

pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    receiver: Receiver<GlobalHotKeyEvent>,
    com_hotkey_id: Option<u32>,
    clip_hotkey_id: Option<u32>,
}

impl HotkeyManager {
    pub fn new() -> Result<Self, String> {
        let manager = GlobalHotKeyManager::new()
            .map_err(|e| format!("Failed to create hotkey manager: {}", e))?;
        
        let receiver = GlobalHotKeyEvent::receiver().clone();
        
        Ok(Self { 
            manager, 
            receiver,
            com_hotkey_id: None,
            clip_hotkey_id: None,
        })
    }
    
    pub fn register_hotkeys(&mut self, com_hotkey: &str, clip_hotkey: &str) -> Result<(), String> {
        // Unregister existing hotkeys if any
        self.unregister_all();
        
        // Parse and register COM hotkey
        let com_code = self.parse_key_string(com_hotkey)?;
        let com_hotkey = HotKey::new(None, com_code);
        self.manager.register(com_hotkey.clone())
            .map_err(|e| format!("Failed to register COM hotkey: {}", e))?;
        self.com_hotkey_id = Some(com_hotkey.id());
        
        // Parse and register Clip hotkey  
        let clip_code = self.parse_key_string(clip_hotkey)?;
        let clip_hotkey = HotKey::new(None, clip_code);
        self.manager.register(clip_hotkey.clone())
            .map_err(|e| format!("Failed to register Clip hotkey: {}", e))?;
        self.clip_hotkey_id = Some(clip_hotkey.id());
        
        Ok(())
    }
    
    fn unregister_all(&mut self) {
        // Simply reset the IDs since we'll re-register everything
        // The global-hotkey crate will handle cleanup when the manager is dropped
        self.com_hotkey_id = None;
        self.clip_hotkey_id = None;
    }
    
    fn parse_key_string(&self, key_str: &str) -> Result<Code, String> {
        match key_str.to_lowercase().as_str() {
            "f1" => Ok(Code::F1),
            "f2" => Ok(Code::F2),
            "f3" => Ok(Code::F3),
            "f4" => Ok(Code::F4),
            "f5" => Ok(Code::F5),
            "f6" => Ok(Code::F6),
            "f7" => Ok(Code::F7),
            "f8" => Ok(Code::F8),
            "f9" => Ok(Code::F9),
            "f10" => Ok(Code::F10),
            "f11" => Ok(Code::F11),
            "f12" => Ok(Code::F12),
            "a" => Ok(Code::KeyA),
            "b" => Ok(Code::KeyB),
            "c" => Ok(Code::KeyC),
            "d" => Ok(Code::KeyD),
            "e" => Ok(Code::KeyE),
            "f" => Ok(Code::KeyF),
            "g" => Ok(Code::KeyG),
            "h" => Ok(Code::KeyH),
            "i" => Ok(Code::KeyI),
            "j" => Ok(Code::KeyJ),
            "k" => Ok(Code::KeyK),
            "l" => Ok(Code::KeyL),
            "m" => Ok(Code::KeyM),
            "n" => Ok(Code::KeyN),
            "o" => Ok(Code::KeyO),
            "p" => Ok(Code::KeyP),
            "q" => Ok(Code::KeyQ),
            "r" => Ok(Code::KeyR),
            "s" => Ok(Code::KeyS),
            "t" => Ok(Code::KeyT),
            "u" => Ok(Code::KeyU),
            "v" => Ok(Code::KeyV),
            "w" => Ok(Code::KeyW),
            "x" => Ok(Code::KeyX),
            "y" => Ok(Code::KeyY),
            "z" => Ok(Code::KeyZ),
            "1" => Ok(Code::Digit1),
            "2" => Ok(Code::Digit2),
            "3" => Ok(Code::Digit3),
            "4" => Ok(Code::Digit4),
            "5" => Ok(Code::Digit5),
            "6" => Ok(Code::Digit6),
            "7" => Ok(Code::Digit7),
            "8" => Ok(Code::Digit8),
            "9" => Ok(Code::Digit9),
            "0" => Ok(Code::Digit0),
            _ => Err(format!("Unsupported key: {}", key_str)),
        }
    }
    
    pub fn check_events(&self) -> Vec<HotkeyEvent> {
        let mut events = Vec::new();
        
        while let Ok(event) = self.receiver.try_recv() {
            if event.state == HotKeyState::Pressed {
                if Some(event.id) == self.com_hotkey_id {
                    events.push(HotkeyEvent::ComOffset);
                } else if Some(event.id) == self.clip_hotkey_id {
                    events.push(HotkeyEvent::WallClip);
                }
            }
        }
        
        events
    }
}

#[derive(Debug, Clone)]
pub enum HotkeyEvent {
    ComOffset,
    WallClip,
}
