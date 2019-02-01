use crate::entstate::EntState;
use sfml::graphics::Sprite;
use sfml::graphics::Transformable;
use ncollide2d::partitioning::BVT;
use ncollide2d::bounding_volume::AABB;
use ncollide2d::bounding_volume;
use std::cell::Ref;

pub struct Collision<'a> {
    dynamic_bvh     : Option<BVT<&'a Sprite<'a>, AABB<f64>>>,
    static_bvh      : Option<BVT<&'a Sprite<'a>, AABB<f64>>>,
}


impl<'a> Collision<'a> {
    pub fn new() -> Collision<'a> {
        Collision {
            dynamic_bvh: Option::None,
            static_bvh: Option::None,
        }
    }

    pub fn gen_bvt(mut self, entity_state: &'a EntState<'a>) {

        let mut dynamic_sprites: Box<Vec<(&'a Sprite<'a>, AABB<f64>)>> = Box::new(Vec::new());
        {
            let vec = entity_state.dynamic_entities.borrow();
            for i in vec.iter() {
                let bounds = i.global_bounds();
                let pos = i.position();
                let volume = bounding_volume::AABB::new(na::Point2::new(pos.x as f64, pos.y as f64),
                                                        na::Point2::new((pos.x + bounds.width) as f64, (pos.y + bounds.width) as f64));

                dynamic_sprites.push((i, volume));
            }
        }
        self.dynamic_bvh = Some(BVT::new_balanced(dynamic_sprites));

//        let mut static_sprites: Vec<(&Sprite, AABB<f64>)> = Vec::new();
//        {
//            for i in self.static_entities {
//                let bounds = i.local_bounds();
//                let pos = i.position();
//                let volume = bounding_volume::AABB::new(na::Point2::new(pos.x as f64, pos.y as f64),
//                                                        na::Point2::new((pos.x + bounds.width) as f64, (pos.y + bounds.width) as f64));
//
//                static_sprites.push((i, volume));
//            }
//        }
//        self.static_bvh = Some(BVT::new_balanced(static_sprites));
    }
}