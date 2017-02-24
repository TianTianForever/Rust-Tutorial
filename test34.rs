// allow required to silence warnings because only one variant used.
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    // These likewise tie 'u32' tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}
fn main() {
    let color = Color::RGB(255, 122, 12);
    // Try to different variants for 'color'.
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Green => println!("The color is Green"),
        Color::Blue => println!("The color is Blue"),
        Color::RGB(r, g, b) => {
            println!("Red: {}, Green: {}, Blue: {}", r, g, b);
        }

        Color::HSV(h, s, v) => {
            println!("Hue: {}, Saturation: {}, Value: {}", h, s, v);
        }
         
        Color::HSL(h, s, l) => {
            println!("Hue: {}, Saturation: {}, Lightness: {}", h, s, l);
        }
    
        Color::CMY(c, m, y) => {
            println!("Cyan: {}, Magenta: {}, Yellow: {}", c, m, y);
        }
     
        Color::CMYK(c, m, y, k) => {
            println!("Cyan: {}, Magenta: {}, Yellow: {}, Key: {}", c, m, y, k);
        }
    }
}
