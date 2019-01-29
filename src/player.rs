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

    default_impulse: f32,
}

impl<'s> Player<'s> {
    pub fn impulse(&mut self, delta_v: &Vector2<f32>) {
        self.delta.x += delta_v.x * self.default_impulse;
        self.delta.y += delta_v.y * self.default_impulse;
    }

    pub fn velocity(&mut self, delta_v: &Vector2<f32>) {
        self.delta.x = delta_v.x * self.default_impulse;
        self.delta.y = delta_v.y * self.default_impulse;
    }

    pub fn collision(&mut self, objects: &Vec<&Sprite>, delta_t: f32) -> FloatRect {

        let mut collided = FloatRect::new(0.0,0.0,0.0,0.0);
        let (_, future_bounding) = self.future_bounding_aabb(delta_t);

        for i in objects {
            match future_bounding.intersection(&i.global_bounds()) {
                Some(overlap) => {

                    // Get the bounds of the object we're intersecting
                    let intersector = &i.global_bounds();

                    let bounding_box =  future_bounding;


                    let mut deflection = self.delta;
                    let mut reposition = self.pos;




                    if overlap.width < overlap.height {

                        if bounding_box.left + bounding_box.width >= intersector.left &&
                            bounding_box.left < intersector.left {
                            deflection.x = -0.1 * delta_t;
                            reposition.x = intersector.left - bounding_box.width - 1.0;
                        } else if bounding_box.left <= intersector.left + intersector.width &&
                            bounding_box.left + bounding_box.width > intersector.left + bounding_box.width {
                            deflection.x = 0.1 * delta_t;
                            reposition.x = intersector.left + intersector.width + 1.0;
                        }

                    } else {
                        if bounding_box.top + bounding_box.height >= intersector.top &&
                            bounding_box.top < intersector.top {
                            deflection.y = -0.1 * delta_t;
                            reposition.y = intersector.top - bounding_box.height - 1.0;
                        } else if bounding_box.top <= intersector.top  + intersector.height &&
                            bounding_box.top + bounding_box.height > intersector.top + bounding_box.height{
                            deflection.y = 0.1 * delta_t;
                            reposition.y = intersector.top + intersector.height + 1.0;
                        }
                    }

                    self.head.set_position(reposition);
                    self.pos = reposition;
                    self.velocity(&deflection);
                    collided = overlap;
                },
                None => continue
            }
        }

        return collided;
    }

    pub fn future_bounding_aabb(&mut self, delta_t: f32) -> (AABB<f64>, FloatRect) {

        let mut bounds = self.head.global_bounds();
        bounds.left += self.delta.x * delta_t * 8.0;
        bounds.top  += self.delta.y * delta_t * 8.0;

        let a = na::Point2::new(bounds.left as f64, bounds.top as f64);
        let b = na::Point2::new((bounds.left + bounds.width) as f64, (bounds.top + bounds.height) as f64);
        (AABB::new(a, b), bounds)
    }

    pub fn update(&mut self, delta_t: f32) {
        self.pos.x += self.delta.x * delta_t * 8.0;
        self.pos.y += self.delta.y * delta_t * 8.0;
        //println!("{:?}", self.delta);
        let friction = 10.0;
        let ratio = 1.0 / (1.0 + delta_t * friction);
        self.delta *= ratio;

        // Gravity
       // self.delta.y += 45.0 * delta_t;

        self.head.set_position((self.pos.x, self.pos.y));
    }

    pub fn new() -> Self {

        let mut delta = Vector2::new(0.0, 0.0);
        let mut pos   = Vector2::new(0.0, 0.0);

        let mut head = CircleShape::new(10.0, 10);
        head.set_position((delta.x, delta.y));
        head.set_fill_color(&Color::RED);

        Self { head, delta, pos,
            default_impulse: 10.0}
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