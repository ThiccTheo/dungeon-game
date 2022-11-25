use ggez::{
    graphics::Color,
    mint::{Point2, Vector2, Vector3},
};

pub fn normalize<T>(pt: T) -> Vector2<f32>
where
    T: Into<Vector2<f32>>,
{
    let Vector2 { x, y } = pt.into();
    let len = (x.powi(2) + y.powi(2)).sqrt();
    Vector2 {
        x: x / len,
        y: y / len,
    }
}

pub fn vector_lerp<T>(a: T, b: T, mut t: f32) -> Point2<f32>
where
    T: Into<Vector2<f32>>,
{
    t = t.clamp(0.0, 1.0);
    let Vector2 { x: ax, y: ay } = a.into();
    let Vector2 { x: bx, y: by } = b.into();

    Point2 {
        x: ax + (bx - ax) * t,
        y: ay + (by - ay) * t,
    }
}

pub fn color_lerp(a: Color, b: Color, mut t: f32) -> Color {
    t = t.clamp(0.0, 1.0);
    let max = u8::MAX as f32;

    Color::new(
        (a.r + (b.r - a.r) * t) / max,
        (a.g + (b.g - a.g) * t) / max,
        (a.b + (b.b - a.b) * t) / max,
        (a.a + (b.a - a.a) * t) / max,
    )
}

pub fn dot_product<T>(a: T, b: T) -> f32
where
    T: Into<Vector2<f32>>,
{
    let Vector2 { x: ax, y: ay } = a.into();
    let Vector2 { x: bx, y: by } = b.into();

    ax * bx + ay * by
}

pub fn cross_product<T>(a: T, b: T) -> Vector3<f32>
where
    T: Into<Vector3<f32>>,
{
    let Vector3 {
        x: ax,
        y: ay,
        z: az,
    } = a.into();
    let Vector3 {
        x: bx,
        y: by,
        z: bz,
    } = b.into();

    Vector3 {
        x: ay * bz - az * by,
        y: az * bx - ax * bz,
        z: ax * by - ay * bx,
    }
}