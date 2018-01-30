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
mod loader;

use crate::player::Player;
use crate::timer::Timer;
use crate::input::Input;
use crate::loader::Loader;

extern crate nalgebra as na;
extern crate ncollide2d;

use sfml::graphics::{
    Color, RenderTarget, RenderWindow,
    Texture, Sprite, Transformable
};
use sfml::window::{ Event, Key, Style};
use sfml::system::Vector2 as sfVec2;
use ncollide2d::bounding_volume::{self, AABB, BoundingVolumeInterferencesCollector};
use ncollide2d::partitioning::BVT;
use sfml::graphics::RectangleShape;
use std::{thread, time};
use std::cell::RefCell;
use std::rc::Rc;


pub struct EntState<'a> {
    dynamic_entities: Rc<RefCell<Vec< Sprite<'a> >>>,
    static_entities : Rc<RefCell<Vec< Sprite<'a> >>>,
}


fn main() {



    let loader = Loader::new();
    let mut state = EntState {
        dynamic_entities: Rc::new(RefCell::new(Vec::new())),
        static_entities: Rc::new(RefCell::new(Vec::new()))
    };
    {

        let mut state_borrow = &state;
        loader.read_static_entities(String::from("static_entities.txt"), state_borrow);
    }

    let static_sprites: Vec<(&Sprite, AABB<f64>)> = vec![
//        (
//            &block_sprite,
//            {
//                let bounds = &block_sprite.local_bounds();
//                let pos    = &block_sprite.position();
//                bounding_volume::AABB::new(na::Point2::new(pos.x as f64, pos.y as f64),
//                                           na::Point2::new((pos.x + bounds.width) as f64, (pos.y + bounds.width) as f64))
//            },
//        ),
//        (
//            &block_sprite2,
//            {
//                let bounds = &block_sprite2.local_bounds();
//                let pos    = &block_sprite2.position();
//                bounding_volume::AABB::new(na::Point2::new(pos.x as f64, pos.y as f64),
//                                           na::Point2::new((pos.x + bounds.width) as f64, (pos.y + bounds.width) as f64))
//            },
//        ),
//        (
//            &block_sprite3,
//            {
//                let bounds = &block_sprite3.local_bounds();
//                let pos    = &block_sprite3.position();
//                bounding_volume::AABB::new(na::Point2::new(pos.x as f64, pos.y as f64),
//                                           na::Point2::new((pos.x + bounds.width) as f64, (pos.y + bounds.width) as f64))
//            },
//        ),
    ];

    let bvt = BVT::new_balanced(static_sprites);

    let mut sprite = Sprite::new();
//    sprite.set_texture(&spritesheet_text, false);
//    sprite.set_texture_rect(&util::grab_sheet_rec(String::from("playerBlue_stand.png"), &spritesheet_desc));

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

        // intersection test
        let mut interferences = Vec::new();
        {
            // Get the AABB bounding box
            let (bv, _) = player.future_bounding_aabb(delta_time);
            let mut thing = BoundingVolumeInterferencesCollector::new(&bv, &mut interferences);
            bvt.visit(&mut thing);
        }

        let collision_rect = player.collision(&interferences, delta_time);

        player.update(delta_time);

        let mut collision_sprite = RectangleShape::new();
        collision_sprite.set_position((collision_rect.left, collision_rect.top));
        collision_sprite.set_size((collision_rect.width, collision_rect.height));


        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);

        window.clear(&Color::BLACK);
        window.draw(&player);
        window.draw(&collision_sprite);


        for ent in state.static_entities.borrow().iter() {
            window.draw(ent);
        }

        if interferences.len() == 0 {
         //   window.draw(&block_sprite);
//            window.draw(&block_sprite2);
//            window.draw(&block_sprite3);
        }

        window.display();

    }

}