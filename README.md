# Moonwalk Macros  
A Rust desktop application for Roblox automation featuring COM offset and wall clip macros with precise timing control.  
<img width="482" height="632" alt="image" src="https://github.com/user-attachments/assets/4f960f0d-5ee3-4cc1-a1ae-26c0a8051aa1" />
## Features

### Two Main Macros
- **COM Offset Macro**
- **Wall Clip Macro**
### Configuration
- Emote slots (1-8)
- Gear/item slots (1-9, 0) 
- Precise timing delays (default 0.97s)
- Shiftlock key selection (Shift/Ctrl)
- Toggle options for unequipping and unshiftlocking

### Control Methods
- Global hotkeys (F7 for COM, F8 for Clip by default)
- Manual execution buttons
- Real-time status feedback

## Prerequisites

1. **Install Rust**: Visit [rustup.rs](https://rustup.rs/) and follow the installation instructions
2. **Windows**: Visual Studio C++ Build Tools may be required for some dependencies

## Installation & Running

1. Clone or download this project
2. Open a terminal in the project directory
3. Install dependencies and run:
   ```bash
   cargo run --release
   ```

## Dependencies

- **eframe/egui**: Modern cross-platform GUI framework
- **enigo**: Cross-platform input simulation
- **global-hotkey**: Global hotkey detection
- **tokio**: Async runtime for threading
- **serde**: Configuration serialization

## Usage

1. **Configure Settings**: Set your preferred emote slot, gear slot, timing delay, and options
2. **Set Hotkeys**: Customize your hotkey bindings and click "Apply Hotkeys"
3. **Execute Macros**: Use hotkeys or click the action buttons
4. **Monitor Status**: Watch the status bar for execution feedback

## Timing Information

- **Peak Position**: Moonwalk peak forward position occurs at ~1.00s
- **Default Delay**: 0.97s (50ms early to account for scheduling and game latency)  
- **Fine-tuning**: Adjust between 0.95-0.98s for optimal consistency
- **Wall Clip Movement**: 0.25s hold duration for W key movement

## Troubleshooting

- **Hotkeys not working**: Run as administrator on Windows
- **Input not detected**: Ensure the game window has focus
- **Timing issues**: Adjust the peak delay in 0.01s increments
- **Build errors**: Ensure Rust and C++ build tools are properly installed

## Technical Details

- **Language**: Rust 2021 edition
- **GUI Framework**: egui with native backend
- **Input System**: enigo for cross-platform key simulation
- **Hotkeys**: global-hotkey for background detection
- **Performance**: Near-zero overhead with precise microsecond timing

## License

This project is provided as-is for educational and personal use.
