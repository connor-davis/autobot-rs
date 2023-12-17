use anyhow::Error;
use multiinput::{DeviceType, KeyId, RawEvent, RawInputManager};

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct Gui {
    pub config: Config,
}

impl Gui {
    pub fn init() -> Result<Gui, Error> {
        let config = Config::init()?;

        Ok(Gui { config })
    }

    pub fn art(&self) {
        println!(
            r#"
        
    ___         __        __          __ 
    /   | __  __/ /_____  / /_  ____  / /_
   / /| |/ / / / __/ __ \/ __ \/ __ \/ __/
  / ___ / /_/ / /_/ /_/ / /_/ / /_/ / /_  
 /_/  |_\__,_/\__/\____/_.___/\____/\__/  
                                          
 
        "#
        );
    }

    pub fn menu(&self, options: Vec<&str>) -> String {
        self.clear();
        self.art();

        println!("Select an option:\n");

        for (index, option) in options.iter().enumerate() {
            println!("{}: {}", index + 1, option);
        }

        println!("\n");

        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_lowercase();

        return input;
    }

    pub fn input(&self, message: &str) -> String {
        self.clear();
        self.art();

        println!("{}", message);

        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_lowercase();

        return input;
    }

    pub fn interactive_input(&self, message: &str) -> String {
        let input_manager_result = RawInputManager::new();

        match input_manager_result {
            Ok(mut input_manager) => {
                input_manager.register_devices(DeviceType::Keyboards);
                input_manager.register_devices(DeviceType::Mice);

                loop {
                    if let Some(event) = input_manager.get_event() {
                        self.clear();
                        self.art();

                        println!("{}", message);

                        match event {
                            RawEvent::KeyboardEvent(_, key_id, _) => {
                                if key_id != KeyId::Escape && key_id != KeyId::Return {
                                    return format!("{:?}", key_id);
                                }
                            }
                            RawEvent::MouseButtonEvent(_, button, _) => {
                                return format!("{:?}", button);
                            }
                            _ => (),
                        }
                    }
                }
            }
            Err(error) => {
                println!("Error: {}", error);

                println!("Failed to initialize input manager. Exiting in 2 seconds...");

                std::thread::sleep(std::time::Duration::from_secs(2));

                std::process::exit(0);
            }
        }
    }

    pub fn clear(&self) {
        println!("{}[2J", 27 as char);
    }
}
