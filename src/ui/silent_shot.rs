use anyhow::Error;

use crate::config::save_config;

use super::gui::Gui;

pub async fn silent_shot_gui(mut gui: Gui) -> Result<(), Error> {
    gui.lines(vec![
        format!("Lethal Key: {}", gui.config.silent_shot_lethal_key),
        format!(
            "Weapon Swap Key: {}",
            gui.config.silent_shot_weapon_swap_key
        ),
        format!(
            "Lethal Key Delay: {}ms",
            gui.config.silent_shot_lethal_key_delay
        ),
        format!(
            "Weapon Swap Delay: {}ms",
            gui.config.silent_shot_weapon_swap_delay
        ),
    ]);

    let input = gui.menu(vec![
        "Edit Lethal Key",
        "Edit Weapon Swap Key",
        "Edit Lethal Key Delay",
        "Edit Weapon Swap Delay",
        "Exit menu",
    ]);

    match input.as_str() {
        "1" => {
            let choice = gui.interactive_input("Enter a new lethal key: ");

            if choice.is_empty() {
                gui.lines(vec!["Invalid input.".to_string()]);
            } else {
                gui.config.silent_shot_lethal_key = choice;

                save_config(gui.config.clone())?;
            }

            gui.clear();
        }
        "2" => {
            let choice = gui.interactive_input("Enter a new weapon swap key: ");

            if choice.is_empty() {
                gui.lines(vec!["Invalid input.".to_string()]);
            } else {
                gui.config.silent_shot_weapon_swap_key = choice;

                save_config(gui.config.clone())?;
            }

            gui.clear();
        }
        "3" => {
            let choice = gui.input("Enter a new lethal key delay: ");

            if choice.is_empty() {
                gui.lines(vec!["Invalid input.".to_string()]);
            } else {
                gui.config.silent_shot_lethal_key_delay = choice.parse::<u64>()?;

                save_config(gui.config.clone())?;
            }

            gui.clear();
        }
        "4" => {
            let choice = gui.input("Enter a new weapon swap delay: ");

            if choice.is_empty() {
                gui.lines(vec!["Invalid input.".to_string()]);
            } else {
                gui.config.silent_shot_weapon_swap_delay = choice.parse::<u64>()?;

                save_config(gui.config.clone())?;
            }

            gui.clear();
        }
        "5" => {
            gui.clear();
        }
        _ => {}
    }

    Ok(())
}
