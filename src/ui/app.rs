use anyhow::Error;

use super::{gui::Gui, silent_shot::silent_shot_gui};

pub async fn initialize_ui() -> Result<(), Error> {
    let gui = Gui::init()?;

    loop {
        let input = gui.menu(vec!["Silent Shot", "Exit"]);

        match input.as_str() {
            "1" => {
                silent_shot_gui(gui.clone()).await?;
            }
            "2" => {
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
