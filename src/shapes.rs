#[derive(Clone, PartialEq)]
pub enum ShapeKind {
    Sphere(f32),
    Plane(f32, f32),
}