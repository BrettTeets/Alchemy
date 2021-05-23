pub mod graphics;
pub mod texture;
pub mod camera;
pub mod gpu;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}
