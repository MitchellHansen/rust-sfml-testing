#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate quick_xml;
extern crate sfml;
extern crate cgmath;

mod timer;
mod player;
mod input;
mod util;

use crate::player::Player;
use crate::timer::Timer;
use crate::input::Input;

use sfml::graphics::{
    Color, RenderTarget, RenderWindow,
    Texture, Sprite, IntRect
};
use sfml::window::{ Event, Key, Style};
use sfml::system::Vector2;


fn main() {

    let spritesheet_desc = util::read_spritesheet(String::from("spritesheet_complete.xml"));
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
        (500, 500),
        "Custom drawable",
        Style::CLOSE,
        &Default::default(),
    );

    let mut player = Player::new();
    let mut timer = Timer::new();
    let mut input = Input::new();


    let step_size:            f32 = 0.005;
    let mut elapsed_time:     f32;
    let mut delta_time:       f32;
    let mut accumulator_time: f32 = 0.0;
    let mut current_time:     f32 = timer.elap_time();



    while window.is_open() {

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                Event::KeyPressed { code, .. } => {
                    if code == Key::Escape {
                        return;
                    }
                }
                _ => {}
            }
            input.ingest(&event)
        }

        if input.is_held(Key::W) {
            player.impulse(&Vector2::new(0.0, -1.0));
        }
        if input.is_held(Key::A) {
            player.impulse(&Vector2::new(-1.0, 0.0));
        }
        if input.is_held(Key::S) {
            player.impulse(&Vector2::new(0.0, 1.0));
        }
        if input.is_held(Key::D) {
            player.impulse(&Vector2::new(1.0, 0.0));
        }

        elapsed_time = timer.elap_time();
        delta_time = elapsed_time - current_time;
        current_time = elapsed_time;
        if delta_time > 0.02 {
            delta_time = 0.02;
        }
        accumulator_time += delta_time;

        while (accumulator_time - step_size) >= step_size {
            accumulator_time -= step_size;


        }

        player.update(delta_time);
        window.clear(&Color::BLACK);
        window.draw(&player);
        window.display();

    }
}