use std::collections::HashMap;
use sfml::graphics::Texture;
use crate::util;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use sfml::graphics::Sprite;
use sfml::graphics::Transformable;
use std::cell::RefCell;
use crate::player::Player;
use ncollide2d::partitioning::BVT;
use ncollide2d::bounding_volume::AABB;
use crate::resources::Resources;
use ncollide2d::bounding_volume;
use std::rc::Rc;


pub struct EntState<'a> {
    pub dynamic_entities: Rc<RefCell<Vec< Sprite<'a> >>>,
    pub static_entities : Rc<RefCell<Vec< Sprite<'a> >>>,
    pub player          : Player<'a>,
}

impl<'a> EntState<'a> {

    pub fn new() -> EntState<'a> {
        EntState{
            dynamic_entities: Rc::new(RefCell::new(Vec::new())),
            static_entities: Rc::new(RefCell::new(Vec::new())),
            player: Player::new(),
        }
    }



    pub fn read_static_entities(mut self, filename: String, resources: &'a Resources) {

        let mut sprite : Sprite = Sprite::new();
        sprite.set_texture(&resources.spritesheet_text, false);
        sprite.set_texture_rect(&util::grab_sheet_rec(String::from("blockBrown.png"), &resources.spritesheet_desc));
        sprite.set_position((0.0,0.0));

        //self.static_entities.get_mut().push(sprite);

//        let file = File::open(filename).expect("Could not open file");
//
//        let mut first_line: bool = true;
//        let mut w: f32 = 0.0;
//        let mut h: f32 = 0.0;
//
//        let mut x: i32;
//        let mut y: i32 = 0;
//
//
//        for line in BufReader::new(file).lines() {
//
//            if first_line {
//
//                first_line = !first_line;
//                let val = line.unwrap();
//                let arr : Vec<&str> = val.split_whitespace().collect();
//
//                w = arr.get(0).unwrap().parse::<f32>().unwrap();
//                h = arr.get(1).unwrap().parse::<f32>().unwrap();
//
//            } else {
//
//                x = 0;
//
//                let val = line.unwrap();
//                for i in val.split_whitespace() {
//                    match i.parse::<i32>().unwrap() {
//                        0 => {
//                            // Do nothing for now
//                        }
//                        1 => {
//                            let mut sprite : Sprite<'a> = Sprite::new();
//                            sprite.set_texture(&self.spritesheet_text, false);
//                            sprite.set_texture_rect(&util::grab_sheet_rec(String::from("blockBrown.png"), &self.spritesheet_desc));
//                            sprite.set_position((x as f32 * w, y as f32 * h));
//
//                            self.static_entities.push(RefCell::new(sprite));
//                        }
//                        _ => {
//                            panic!("ahhhhhh");
//                        }
//                    }
//                    x += 1;
//                }
//                y += 1;
//            }
//        }
    }

//    pub fn read_dynamic_entities(&'a mut self, filename: String) {
//
//        let file = File::open(filename).expect("Could not open file");
//
//        for line in BufReader::new(file).lines() {
//
//            let val = line.unwrap();
//            let arr : Vec<&str> = val.split_whitespace().collect();
//
//            let e = arr.get(0).unwrap();
//            let x = arr.get(1).unwrap().parse::<f32>().unwrap();
//            let y = arr.get(2).unwrap().parse::<f32>().unwrap();
//
//            match *e {
//                "enemy" => {
//                    let mut sprite = Sprite::new();
//                    sprite.set_texture(&self.spritesheet_text, false);
//                    sprite.set_texture_rect(&util::grab_sheet_rec(String::from("enemyFloating_1.png"), &self.spritesheet_desc));
//                    sprite.set_position((x, y));
//
//                    self.dynamic_entities.push(RefCell::new(sprite));
//                }
//                "player" => {
//                    let mut sprite = Sprite::new();
//                    sprite.set_texture(&self.spritesheet_text, false);
//                    sprite.set_texture_rect(&util::grab_sheet_rec(String::from("playerBlue_up3.png"), &self.spritesheet_desc));
//                    sprite.set_position((x, y));
//
//                    self.dynamic_entities.push(RefCell::new(sprite));
//                }
//                _ => {
//                    // Do nothing
//                }
//            }
//        }
//    }
}