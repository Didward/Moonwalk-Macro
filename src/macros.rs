// Low-level Windows API input simulation for game compatibility
#[cfg(windows)]
use winapi::um::winuser::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE,
    VK_LSHIFT, VK_LCONTROL, MapVirtualKeyW, MAPVK_VK_TO_VSC
};
#[cfg(windows)]
use winapi::um::winuser::KEYBDINPUT;
#[cfg(windows)]
use std::mem;

use std::thread;
use std::time::Duration;
use crate::config::{Config, ShiftlockKey};

pub struct MacroExecutor {
    // Empty struct - we'll use static Windows API calls
}

impl MacroExecutor {
    pub fn new() -> Result<Self, String> {
        Ok(Self {})
    }
    
    #[cfg(windows)]
    fn send_key_input(vk_code: u16, scan_code: u16, key_up: bool) -> Result<(), String> {
        unsafe {
            let mut input = INPUT {
                type_: INPUT_KEYBOARD,
                u: mem::zeroed(),
            };
            
            // Use both virtual key code and scan code for maximum compatibility
            *input.u.ki_mut() = KEYBDINPUT {
                wVk: vk_code,
                wScan: scan_code,
                dwFlags: if key_up { KEYEVENTF_KEYUP | KEYEVENTF_SCANCODE } else { KEYEVENTF_SCANCODE },
                time: 0,
                dwExtraInfo: 0,
            };
            
            let result = SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
            if result == 0 {
                return Err(format!("Failed to send key input for VK {}", vk_code));
            }
        }
        Ok(())
    }
    
    #[cfg(windows)]
    fn press_key(vk_code: u16) -> Result<(), String> {
        unsafe {
            let scan_code = MapVirtualKeyW(vk_code as u32, MAPVK_VK_TO_VSC) as u16;
            Self::send_key_input(vk_code, scan_code, false)?; // Press
            thread::sleep(Duration::from_millis(10));
            Self::send_key_input(vk_code, scan_code, true)?;  // Release
        }
        Ok(())
    }
    
    #[cfg(windows)]
    fn hold_key(vk_code: u16) -> Result<(), String> {
        unsafe {
            let scan_code = MapVirtualKeyW(vk_code as u32, MAPVK_VK_TO_VSC) as u16;
            Self::send_key_input(vk_code, scan_code, false) // Just press, don't release
        }
    }
    
    #[cfg(windows)]
    fn release_key(vk_code: u16) -> Result<(), String> {
        unsafe {
            let scan_code = MapVirtualKeyW(vk_code as u32, MAPVK_VK_TO_VSC) as u16;
            Self::send_key_input(vk_code, scan_code, true) // Just release
        }
    }
    
    #[cfg(not(windows))]
    fn press_key(_vk_code: u16) -> Result<(), String> {
        Err("Windows-only functionality".to_string())
    }
    
    #[cfg(not(windows))]
    fn hold_key(_vk_code: u16) -> Result<(), String> {
        Err("Windows-only functionality".to_string())
    }
    
    #[cfg(not(windows))]
    fn release_key(_vk_code: u16) -> Result<(), String> {
        Err("Windows-only functionality".to_string())
    }
    
    pub fn execute_com_offset(&mut self, config: &Config) -> Result<(), String> {
        // Validate configuration
        config.validate_all()?;
        
        println!("Starting COM Offset macro...");
        
        #[cfg(windows)]
        {
            // Send "." key using low-level input simulation
            println!("Sending '.' (Period)...");
            Self::press_key(0xBE)?; // Press and release Period directly
            thread::sleep(Duration::from_millis(50));
            
            // Send emote slot number
            let emote_vk = match config.emote_slot {
                1 => 0x31, 2 => 0x32, 3 => 0x33, 4 => 0x34,
                5 => 0x35, 6 => 0x36, 7 => 0x37, 8 => 0x38,
                _ => return Err("Invalid emote slot".to_string()),
            };
            
            println!("Sending emote slot {}...", config.emote_slot);
            Self::press_key(emote_vk)?;
            thread::sleep(Duration::from_millis(50));
            
            // Wait for peak delay
            let delay_ms = (config.peak_delay * 1000.0) as u64;
            println!("Waiting {} ms for peak delay...", delay_ms);
            thread::sleep(Duration::from_millis(delay_ms));
            
            // Send gear slot
            let gear_char = config.gear_slot.chars().next().unwrap_or('1');
            let gear_vk = match gear_char {
                '1' => 0x31, '2' => 0x32, '3' => 0x33, '4' => 0x34, '5' => 0x35,
                '6' => 0x36, '7' => 0x37, '8' => 0x38, '9' => 0x39, '0' => 0x30,
                _ => return Err("Invalid gear slot".to_string()),
            };
            
            println!("Sending gear slot {}...", gear_char);
            Self::press_key(gear_vk)?;
            
            // Unequip if enabled
            if config.unequip_after {
                thread::sleep(Duration::from_millis(100));
                println!("Unequipping gear...");
                Self::press_key(gear_vk)?;
            }
            
            println!("COM Offset macro completed!");
        }
        
        #[cfg(not(windows))]
        {
            return Err("Macro execution only supported on Windows".to_string());
        }
        
        Ok(())
    }
    
    pub fn execute_wall_clip(&mut self, config: &Config) -> Result<(), String> {
        // Validate configuration
        config.validate_all()?;
        
        #[cfg(windows)]
        {
            // Send "." key using low-level input simulation
            Self::press_key(0xBE)?; // Press and release Period directly
            thread::sleep(Duration::from_millis(50));
            
            // Send emote slot number
            let emote_vk = match config.emote_slot {
                1 => 0x31, 2 => 0x32, 3 => 0x33, 4 => 0x34,
                5 => 0x35, 6 => 0x36, 7 => 0x37, 8 => 0x38,
                _ => return Err("Invalid emote slot".to_string()),
            };
            
            Self::press_key(emote_vk)?;
            thread::sleep(Duration::from_millis(50));
            
            // Wait for peak delay
            let delay_ms = (config.peak_delay * 1000.0) as u64;
            thread::sleep(Duration::from_millis(delay_ms));
            
            // Get shiftlock key
            let shift_vk = match config.shiftlock_key {
                ShiftlockKey::Shift => VK_LSHIFT as u16,
                ShiftlockKey::Control => VK_LCONTROL as u16,
            };
            
            // Hold shiftlock + W for movement
            Self::hold_key(shift_vk)?; // Press and hold shiftlock
            thread::sleep(Duration::from_millis(20));
            
            Self::hold_key(0x57)?; // Press and hold W
            
            // Hold both keys for 0.25 seconds
            thread::sleep(Duration::from_millis(250));
            
            // Release keys in reverse order
            Self::release_key(0x57)?; // Release W
            thread::sleep(Duration::from_millis(20));
            Self::release_key(shift_vk)?; // Release shiftlock
            
            // Unshiftlock if enabled
            if config.unshiftlock_after {
                thread::sleep(Duration::from_millis(100));
                Self::press_key(shift_vk)?; // Quick press to toggle shiftlock off
            }
        }
        
        #[cfg(not(windows))]
        {
            return Err("Macro execution only supported on Windows".to_string());
        }
        
        Ok(())
    }
}
