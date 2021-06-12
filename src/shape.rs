pub enum ShapeTypeEnum {
    CUBE,
}

pub struct Shape {
    name: String,
    shape_gen: Box<dyn GenerateShape>,
}

trait GenerateShape {
    fn gen_plane_cloud(&self, parameter: f32) -> Vec<(f32,f32,f32)>;
}

struct CubeShape {}
impl GenerateShape for CubeShape {
    fn gen_plane_cloud(&self, side: f32) -> Vec<(f32,f32,f32)> {
        let mut param_distr = Shape::density(100);
        for i in 0..param_distr.len() {
            param_distr[i] -= 0.5;
            param_distr[i] *= side;
        }
        for entry in &param_distr {
            println!("{}", entry);
        }
        let mut point_cloud = Vec::new();
        for i in 0..param_distr.len() {
            for j in 0..param_distr.len() {
                for k in 0..param_distr.len() {
                    point_cloud.push((param_distr[i], param_distr[j], param_distr[k]));
                }
            }
        }
        println!("{:?}", point_cloud);
        point_cloud
    }
}

impl Shape {
    pub fn new(shape_type: ShapeTypeEnum) -> Shape {
        match shape_type {
            ShapeTypeEnum::CUBE => Shape {name: "Cube Shape".to_string(),
                                          shape_gen: Box::new(CubeShape{})},
        }
    }

    fn density(points: i32) -> Vec<f32> {
        let mut n = 0;
        let mut distribution = Vec::new();
        while n <= points {
            distribution.push(n as f32 / points as f32);
            n += 1;
        }
        distribution
    }

    pub fn perform_plane_cut(&self, side: f32) {
        self.shape_gen.gen_plane_cloud(side);
    }
}
