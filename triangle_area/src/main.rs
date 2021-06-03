#[derive(Debug)]
struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}

impl Triangle {
    fn get_area(&self) -> f32 {
        let Self { a, b, c } = &self;
        let p = (a + b + c) / 2.0;
        let area = (p * (p - a) * (p - b) * (p - c)).sqrt();
        area
    }
}

fn main() {
    let my_triangle = Triangle {a: 26.0, b: 26.0, c:20.0 };
    let area = my_triangle.get_area();
    print!("{:?} area: {}", my_triangle, area)
}
