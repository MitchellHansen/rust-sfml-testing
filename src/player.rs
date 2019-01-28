use sfml::system::Vector2;
use sfml::graphics::{CircleShape, Color, Drawable,
                     RenderStates, RenderTarget};
use sfml::graphics::Transformable;
use sfml::graphics::Shape;
use sfml::graphics::Sprite;
use ncollide2d::bounding_volume::AABB;
use sfml::graphics::FloatRect;

pub struct Player<'s> {
    head: CircleShape<'s>,
    delta: Vector2<f32>,
    pub pos: Vector2<f32>,
}

impl<'s> Player<'s> {
    pub fn impulse(&mut self, delta_v: &Vector2<f32>) {
        self.delta.x += delta_v.x * 2.0;
        self.delta.y += delta_v.y * 2.0;
    }

    pub fn collision(&mut self, objects: &Vec<&Sprite>) -> FloatRect {

        let mut collided = FloatRect::new(0.0,0.0,0.0,0.0);

        for i in objects {
            match self.head.global_bounds().intersection(&i.global_bounds()) {
                Some(r) => {
                    let tested = &i.global_bounds();

//                    if r.width > tested.width / 2.0 {
//                        self.delta.x = 1.0;
//                    } else if r.width < tested.width / 2.0 {
//                        self.delta.x = -1.0;
//                    }
//
//                    if r.height > tested.height / 2.0 {
//                        self.delta.y = -1.0;
//                    } else if r.height < tested.height / 2.0 {
//                        self.delta.y = 1.0;
//                    }
                    //println!("{:?}", r);
                    collided = r;
                },
                None => continue
            }
        }

        return collided;
    }

    pub fn bounding_aabb(&mut self) -> AABB<f64> {
        let pos = self.pos;
        let a = na::Point2::new(pos.x as f64, pos.y as f64);
        let b = na::Point2::new((pos.x + 10.0) as f64, (pos.y + 10.0) as f64);
        AABB::new(a, b)
    }

    pub fn update(&mut self, delta_t: f32) {
        self.pos.x += self.delta.x * delta_t * 1.0;
        self.pos.y += self.delta.y * delta_t * 1.0;

        let friction = 10.0 * self.delta;
        self.delta -= friction * delta_t;

        self.head.set_position((self.pos.x, self.pos.y));
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
    }
}