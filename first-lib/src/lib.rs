pub fn public_function() {
    println!("called first-lib's `public_function()`");
}

fn private_function() {
    println!("called firs-lib's `private_function()`");
}

pub fn indirect_access() {
    print!("called first-lib's `indirect_access()`, that\n> ");

    private_function();
}

pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn get_x(&self) -> f32 {
        self.x
    }
    fn get_y(&self) -> f32 {
        self.y
    }
}

pub fn create_point(x: f32, y: f32) -> Point {
    Point { x, y }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
