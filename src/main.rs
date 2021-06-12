mod shape;
use shape::*;

fn main() {
    let mut model_shape = Shape::new(ShapeTypeEnum::CUBE);
    model_shape.perform_plane_cut(5.0);
    println!("Testing");
}
