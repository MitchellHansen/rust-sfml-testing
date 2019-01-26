use sfml::system::Vector2;
use sfml::graphics::{CircleShape,
                     Color,
                     Drawable,
                     RenderStates,
                     RenderTarget};
use sfml::graphics::Transformable;
use sfml::graphics::Shape;
use sfml::graphics::RectangleShape;
use sfml::graphics::Rect;

pub struct Player<'s> {
    head: CircleShape<'s>,
    delta: Vector2<f32>,
    pos: Vector2<f32>,
}

impl<'s> Player<'s> {
    pub fn impulse(&mut self, delta_v: &Vector2<f32>) {
        self.delta.x += delta_v.x;
        self.delta.y += delta_v.y;
    }

    pub fn collision(&mut self, objects: Vec<Rect<f32>>) {

        for i in objects {

            match self.head.local_bounds().intersection(&i) {
                Some(r) => println!("{:?}", r),
                None => continue
            }
        }
    }

    pub fn update(&mut self, delta_t: f32) {
        self.pos.x += self.delta.x * delta_t * 1.0;
        self.pos.y += self.delta.y * delta_t * 1.0;

        self.delta *= 0.999;

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