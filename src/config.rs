use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub emote_slot: u8,
    pub gear_slot: String,
    pub peak_delay: f64,
    pub unequip_after: bool,
    pub unshiftlock_after: bool,
    pub shiftlock_key: ShiftlockKey,
    pub com_hotkey: String,
    pub clip_hotkey: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShiftlockKey {
    Shift,
    Control,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            emote_slot: 1,
            gear_slot: "1".to_string(),
            peak_delay: 0.97,
            unequip_after: true,
            unshiftlock_after: false,
            shiftlock_key: ShiftlockKey::Shift,
            com_hotkey: "f7".to_string(),
            clip_hotkey: "f8".to_string(),
        }
    }
}

impl Config {
    pub fn validate_emote_slot(&self) -> Result<(), String> {
        if !(1..=8).contains(&self.emote_slot) {
            return Err("Emote slot must be between 1-8".to_string());
        }
        Ok(())
    }
    
    pub fn validate_gear_slot(&self) -> Result<(), String> {
        if self.gear_slot.len() != 1 {
            return Err("Gear slot must be a single character".to_string());
        }
        let valid_keys = "1234567890";
        if !valid_keys.contains(&self.gear_slot) {
            return Err("Gear slot must be 1-9 or 0".to_string());
        }
        Ok(())
    }
    
    pub fn validate_delay(&self) -> Result<(), String> {
        if !(0.05..=5.0).contains(&self.peak_delay) {
            return Err("Peak delay must be between 0.05-5.0 seconds".to_string());
        }
        Ok(())
    }
    
    pub fn validate_all(&self) -> Result<(), String> {
        self.validate_emote_slot()?;
        self.validate_gear_slot()?;
        self.validate_delay()?;
        Ok(())
    }
}
