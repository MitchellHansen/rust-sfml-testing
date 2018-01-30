use std::collections::HashMap;
use sfml::graphics::Texture;
use crate::util;
use crate::EntState;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use sfml::graphics::Sprite;
use sfml::graphics::Transformable;

pub struct Loader {

    spritesheet_desc : HashMap<String, HashMap<String, i32>>,
    spritesheet_text : Texture

}

impl Loader {

    pub fn new() -> Loader {
        Loader{
            spritesheet_desc: util::read_spritesheet(String::from("spritesheet_complete.xml")),
            spritesheet_text: Texture::from_file("spritesheet_complete.png").expect("Couldn't load texture")
        }
    }


    pub fn read_static_entities<'a>(&'a self, filename: String, entities: &EntState<'a>) {

        let file = File::open(filename).expect("Could not open file");

        let mut first_line: bool = true;
        let mut w: f32 = 0.0;
        let mut h: f32 = 0.0;

        let mut x: i32 = 0;
        let mut y: i32 = 0;


        for line in BufReader::new(file).lines() {

            if first_line {

                first_line = !first_line;
                let val = line.unwrap();
                let arr : Vec<&str> = val.split_whitespace().collect();

                w = arr.get(0).unwrap().parse::<f32>().unwrap();
                h = arr.get(1).unwrap().parse::<f32>().unwrap();

            } else {

                x = 0;

                let val = line.unwrap();
                for i in val.split_whitespace() {
                    match i.parse::<i32>().unwrap() {
                        0 => {
                            // Do nothing for now
                        }
                        1 => {
                            let mut sprite = Sprite::new();
                            sprite.set_texture(&self.spritesheet_text, false);
                            sprite.set_texture_rect(&util::grab_sheet_rec(String::from("blockBrown.png"), &self.spritesheet_desc));
                            sprite.set_position((x as f32 * w, y as f32 * h));

                            entities.static_entities.borrow_mut().push(sprite);
                        }
                        _ => {
                            panic!("ahhhhhh");
                        }
                    }
                    x += 1;
                }
                y += 1;
            }
        }
    }

    pub fn read_dynamic_entities(filename: String, entities: &EntState) {
        let file = File::open(filename).expect("Could not open file");
        for line in BufReader::new(file).lines() {

        }

        let mut sprite1 = Sprite::new();
        entities.dynamic_entities.borrow_mut().push(sprite1);
    }

}