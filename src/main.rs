extern crate quick_xml;
extern crate sfml;
extern crate cgmath;

use quick_xml::events::Event as xmlEvent;
use quick_xml::Reader;
use sfml::graphics::{
    CircleShape, Color, Drawable, 
    RectangleShape, RenderStates, 
    RenderTarget, RenderWindow, Shape,
    Transformable,
};
use sfml::window::{Event, Key, Style};
use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;
use sfml::graphics::{ Texture, Sprite, IntRect};
use cgmath::{InnerSpace, Vector2 };

fn read_spritesheet(filename: String) -> HashMap<String, HashMap<String, i32>> {
    let mut reader = Reader::from_file(filename).unwrap();
    reader.trim_text(true);

    let mut count = 0;
    let mut buf = Vec::new();


    let mut t : HashMap<String, HashMap<String, i32>> = HashMap::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(xmlEvent::Start(ref e)) => {
                match e.name() {
                    b"TextureAtlas" => println!("attributes values: {:?}",
                                                e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()),
                    _ => (),
                }
            },
            Ok(xmlEvent::Empty(d)) => {

                let mut map_entry: HashMap<String, i32> = HashMap::new();

                //println!("{:?}", d.name());

                let mut name: String = String::new();

                for i in d.attributes() {

                    let attr = i.expect("Couldn't grab attribute");

                    let key = String::from_utf8_lossy(attr.key);

                    if key == "name" {

                        let value = match attr.value {
                            Cow::Borrowed(r) => String::from_utf8_lossy(&r),
                            Cow::Owned(r) => break
                        };
                        name = value.to_lowercase()
                    } else {

                        let value = match attr.value {
                            Cow::Borrowed(r) => String::from_utf8_lossy(&r),
                            Cow::Owned(r) => break
                        };

                        map_entry.insert(String::from(key), FromStr::from_str(&value[..]).expect(""));
                    }
                }

                t.insert(name,map_entry);
            },
            Ok(xmlEvent::Eof) => break,
            _ => (),
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    return t;
}


/// Our custom drawable type. It looks like a bullet.
struct Player<'s> {
    head: CircleShape<'s>,
    delta: Vector2<f32>,
    pos: Vector2<f32>,
}

impl<'s> Player<'s> { 
   pub fn impulse(&mut self, delta_v: &Vector2<f32>) {
        self.delta.x = delta_v.x;
        self.delta.y = delta_v.y;
        self.delta.normalize();
   }

   pub fn update(&mut self, delta_t: f32) {
        self.pos.x += self.delta.x * delta_t;
        self.pos.y += self.delta.y * delta_t;

        self.delta *= 0.95;

        self.head.set_position((delta.x, delta.y));
   }

   pub fn new() -> Self {
       let mut delta = Vector2::new(0.0, 0.0);
       let mut pos   = Vector2::new(0.0, 0.0);

       let mut head = CircleShape::new(10.0, 10);
       head.set_position((delta.x, delta.y));
       head.set_fill_color(&Color::RED);

       Self { head, delta, pos }
    }
}

impl<'s> Drawable for Player<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        render_target: &mut RenderTarget,
        _: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        render_target.draw(&self.head);
//        render_target.draw(&self.torso)
    }
}


fn main() {

    let spritesheet_desc = read_spritesheet(String::from("spritesheet_complete.xml"));
    let spritesheet_text = Texture::from_file("spritesheet_complete.png")
        .expect("Couldn't load texture");

    let mut sprite = Sprite::new();
    sprite.set_texture(&spritesheet_text, false);

    println!("{:?}", spritesheet_desc);
    let sprite_desc = spritesheet_desc.get("enemyflyingalt_4.png").expect("Can't load sprite");

    sprite.set_texture_rect(&IntRect::new(
        *sprite_desc.get("x").unwrap(),
        *sprite_desc.get("y").unwrap(),
        *sprite_desc.get("width").unwrap(),
        *sprite_desc.get("height").unwrap()
    ));

    let mut window = RenderWindow::new(
        (100, 100),
        "Custom drawable",
        Style::CLOSE,
        &Default::default(),
    );

    let mut player = Player::new();

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                Event::KeyPressed { code, .. } => {
                    if code == Key::Escape {
                        return;
                    } else if code == Key::W {
                        player.impulse(&Vector2::new(0.0, -1.0));
                    } else if code == Key::A {
                        player.impulse(&Vector2::new(-1.0, 0.0));
                    } else if code == Key::S {
                        player.impulse(&Vector2::new(0.0, 1.0));
                    } else if code == Key::D {
                        player.impulse(&Vector2::new(1.0, 0.0));
                    }
                }
                _ => {}
            }
        }

        window.clear(&Color::BLACK);
        window.draw(&sprite);
        window.draw(&player);
        window.display()
    }
}
