pub trait DisplayBackend {
    fn connect() -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;
    fn set_gamma(&mut self, temp: f32, r: f32, g: f32, b: f32) -> Result<(), Box<dyn std::error::Error>>;
}
pub mod x11;
pub mod wayland;

