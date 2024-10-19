use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn combine(self, other: Rgb) -> Rgb {
        Rgb {
            r: ((self.r as u16 + other.r as u16) / 2) as u8, // Average the red component
            g: ((self.g as u16 + other.g as u16) / 2) as u8, // Average the green component
            b: ((self.b as u16 + other.b as u16) / 2) as u8, // Average the blue component
        }
    }
    pub fn generate_random_rgb(brightness: Option<u8>) -> Self {
        let mut rng = rand::thread_rng();
        let min_brightness = brightness.unwrap_or(128); // Minimum value for brightness

        // Generate random values for each color component
        let r = rng.gen_range(min_brightness..=255); // Red component
        let g = rng.gen_range(min_brightness..=255); // Green component
        let b = rng.gen_range(min_brightness..=255); // Blue component

        Self { r, g, b } // Return the RGB tuple
    }
}
