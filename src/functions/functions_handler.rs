use std::process;

use anyhow::Error;
use multiinput::{DeviceType, KeyId, MouseButton, RawEvent, RawInputManager, State};

use crate::{
    config::{load_config, save_config},
    functions::silent_shot::silent_shot,
    utils::foreground_window::get_foreground_window_exe_name,
};

pub async fn initialize_functions() -> Result<(), Error> {
    let input_manager_result = RawInputManager::new();

    match input_manager_result {
        Ok(mut input_manager) => {
            input_manager.register_devices(DeviceType::Keyboards);
            input_manager.register_devices(DeviceType::Mice);

            loop {
                if let Some(event) = input_manager.get_event() {
                    match event {
                        RawEvent::KeyboardEvent(_, KeyId::F6, State::Released) => {
                            let config = load_config()?;

                            if config.silent_shot_enabled {
                                let mut config = config.clone();

                                config.silent_shot_enabled = false;

                                save_config(config)?;
                            } else {
                                let mut config = config.clone();

                                config.silent_shot_enabled = true;

                                save_config(config)?;
                            }
                        }
                        RawEvent::MouseButtonEvent(_, MouseButton::Left, State::Pressed) => {
                            let foreground_window_exe = get_foreground_window_exe_name();

                            match foreground_window_exe {
                                Ok(foreground_window_exe) => {
                                    if foreground_window_exe.ends_with("cod.exe") {
                                        silent_shot().await?;
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);

            process::exit(0);
        }
    }
}
