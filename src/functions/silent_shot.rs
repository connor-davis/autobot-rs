use std::time::Duration;

use anyhow::Error;
use enigo::{Enigo, Key, KeyboardControllable};
use tokio::time::sleep;

use crate::config::Config;

pub async fn silent_shot() -> Result<(), Error> {
    let config = Config::init()?;

    if config.silent_shot_enabled {
        let mut enigo = Enigo::new();

        sleep(Duration::from_millis(config.silent_shot_lethal_key_delay)).await;

        enigo.key_down(Key::Layout(
            config.silent_shot_lethal_key.chars().next().unwrap_or('e'),
        ));

        sleep(Duration::from_millis(config.silent_shot_weapon_swap_delay)).await;

        enigo.key_down(Key::Layout(
            config
                .silent_shot_weapon_swap_key
                .chars()
                .next()
                .unwrap_or('1'),
        ));

        sleep(Duration::from_millis(35)).await;

        enigo.key_up(Key::Layout(
            config
                .silent_shot_weapon_swap_key
                .chars()
                .next()
                .unwrap_or('1'),
        ));

        sleep(Duration::from_millis(30)).await;

        enigo.key_up(Key::Layout(
            config.silent_shot_lethal_key.chars().next().unwrap_or('e'),
        ));
    }

    Ok(())
}
