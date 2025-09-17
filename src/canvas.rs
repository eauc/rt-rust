use crate::colors;
use crate::floats::Float;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<colors::Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![colors::BLACK; width * height],
        }
    }

    fn position_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> colors::Color {
        self.pixels[self.position_to_index(x, y)]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: colors::Color) {
        let index = self.position_to_index(x, y);
        self.pixels[index] = color;
    }

    pub fn to_ppm(&self) -> String {
        [self.ppm_header(), self.ppm_pixels(), String::from("")].join("\n")
    }

    fn ppm_header(&self) -> String {
        ["P3", &format!("{} {}", self.width, self.height), "255"].join("\n")
    }

    fn ppm_pixels(&self) -> String {
        let mut lines: Vec<String> = vec![];
        for y in 0..self.height {
            let mut line: Vec<String> = vec![];
            for x in 0..self.width {
                let pixel = self.pixel_at(x, y);
                line.push(ppm_clamp_color(pixel.red()).to_string());
                line.push(ppm_clamp_color(pixel.green()).to_string());
                line.push(ppm_clamp_color(pixel.blue()).to_string());
            }
            let l = line
                .into_iter()
                .reduce(|acc, s| {
                    let last_line = acc.rfind('\n').unwrap_or(0);
                    if acc.len() - last_line + s.len() + 1 > 70 {
                        format!("{}\n{}", acc, s)
                    } else {
                        format!("{} {}", acc, s)
                    }
                })
                .unwrap();
            lines.push(l);
        }
        lines.join("\n")
    }
}

fn ppm_clamp_color(v: Float) -> u8 {
    (v * 255.0).clamp(0.0, 255.0).round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for x in 0..10 {
            for y in 0..20 {
                assert_eq!(c.pixel_at(x, y), crate::colors::BLACK);
            }
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = colors::Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();

        let ppm_header = ppm.lines().take(3).collect::<Vec<_>>();
        assert_eq!(ppm_header, vec!["P3", "5 3", "255"]);
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = colors::Color::new(1.5, 0.0, 0.0);
        let c2 = colors::Color::new(0.0, 0.5, 0.0);
        let c3 = colors::Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.to_ppm();
        let ppm_pixel_data = ppm.lines().skip(3).collect::<Vec<_>>();
        assert_eq!(
            ppm_pixel_data,
            vec![
                "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
            ]
        );
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        for x in 0..10 {
            for y in 0..2 {
                c.write_pixel(x, y, colors::Color::new(1.0, 0.8, 0.6));
            }
        }
        let ppm = c.to_ppm();
        let ppm_pixel_data = ppm.lines().skip(3).collect::<Vec<_>>();
        assert_eq!(
            ppm_pixel_data,
            vec![
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
            ]
        );
    }

    //  Scenario: PPM files are terminated by a newline character
    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert!(ppm.ends_with('\n'));
    }
}
