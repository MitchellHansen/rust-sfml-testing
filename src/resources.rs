use crate::util;
use sfml::graphics::Texture;
use std::collections::HashMap;

pub struct Resources {
    pub spritesheet_desc : HashMap<String, HashMap<String, i32>>,
    pub spritesheet_text : Texture,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {

            spritesheet_desc: util::read_spritesheet(String::from("spritesheet_complete.xml")),
            spritesheet_text: Texture::from_file("spritesheet_complete.png").expect("Couldn't load texture")
        }
    }
}