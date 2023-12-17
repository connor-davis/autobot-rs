use std::thread::sleep;

use anyhow::Error;

use super::gui::Gui;

pub async fn silent_shot_gui(gui: Gui) -> Result<(), Error> {
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

            println!("{} has been selected.", choice);

            sleep(std::time::Duration::from_millis(2000));

            gui.clear();
        }
        "2" => {
            gui.clear();
        }
        "3" => {
            gui.clear();
        }
        "4" => {
            gui.clear();
        }
        "5" => {
            gui.clear();
        }
        _ => {}
    }

    Ok(())
}
