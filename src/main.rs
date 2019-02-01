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
mod entstate;
mod resources;
mod collision;

use crate::player::Player;
use crate::timer::Timer;
use crate::input::Input;
use crate::entstate::EntState;
use crate::resources::Resources;
use crate::collision::Collision;

extern crate nalgebra as na;
extern crate ncollide2d;

use sfml::graphics::{
    Color, RenderTarget, RenderWindow,
};
use sfml::window::{ Event, Key, Style};
use sfml::system::Vector2 as sfVec2;


fn main() {

    let mut resources = Resources::new();
    let mut collision = Collision::new();
    let mut state = EntState::new();

    state.read_static_entities(String::from("static_entities.txt"), &resources);
    //state.read_static_entities(String::from("static_entities.txt"), &resources);
    //state.read_dynamic_entities(String::from("dynamic_entities.txt"));

    //state.gen_bvt();


    let mut window = RenderWindow::new(
        (512, 512),
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
            player.impulse(&sfVec2::new(0.0, -1.0));
        }
        if input.is_held(Key::A) {
            player.impulse(&sfVec2::new(-1.0, 0.0));
        }
        if input.is_held(Key::S) {
            player.impulse(&sfVec2::new(0.0, 1.0));
        }
        if input.is_held(Key::D) {
            player.impulse(&sfVec2::new(1.0, 0.0));
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

//        // intersection test
//        let mut interferences = Vec::new();
//        {
//            // Get the AABB bounding box
//            let (bv, _) = player.future_bounding_aabb(delta_time);
//            let mut thing = BoundingVolumeInterferencesCollector::new(&bv, &mut interferences);
//            bvt.visit(&mut thing);
//        }
//
//        let collision_rect = player.collision(&interferences, delta_time);

        player.update(delta_time);
//
//        let mut collision_sprite = RectangleShape::new();
//        collision_sprite.set_position((collision_rect.left, collision_rect.top));
//        collision_sprite.set_size((collision_rect.width, collision_rect.height));

        window.clear(&Color::BLACK);
        window.draw(&player);
        //window.draw(&collision_sprite);

//        for ent in state.static_entities.get_mut().iter() {
//            window.draw(ent);
//        }
//
//        for ent in state.dynamic_entities.get_mut().iter() {
//            window.draw(ent);
//        }

        window.display();

    }

}