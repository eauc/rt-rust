use crate::colors::{BLACK, Color, WHITE};
use crate::floats::Float;
use crate::lights::PointLight;
use crate::patterns::Pattern;
use crate::objects::Object;
use crate::tuples::Tuple;
use std::fmt;

#[derive(Clone)]
pub struct Material {
    pub pattern: Option<Pattern>,
    pub color: Color,
    pub ambient: Float,
    pub diffuse: Float,
    pub reflective: Float,
    pub refractive_index: Float,
    pub shininess: Float,
    pub specular: Float,
    pub transparency: Float,
}

impl Material {
    pub fn default() -> Material {
        Material {
            pattern: None,
            color: WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            reflective: 0.0,
            refractive_index: 1.0,
            shininess: 200.0,
            specular: 0.9,
            transparency: 0.0,
        }
    }
    pub fn glass() -> Material {
        Material {
            pattern: None,
            color: WHITE,
            ambient: 0.0,
            diffuse: 0.588235,
            specular: 0.9,
            transparency: 1.0,
            reflective: 0.08,
            refractive_index: 1.5,
            shininess: 300.0,
        }
    }

    pub fn lighting(
        &self,
        object: &Object,
        light: &PointLight,
        position: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        let color = if let Some(pattern) = &self.pattern {
            pattern.color_at_object(object, position)
        } else {
            self.color
        };
        let effective_color = color * light.intensity;
        let ambient = effective_color * self.ambient;
        let lightv = (light.position - position).normalize();
        let light_dot_normal = lightv.dot(normalv);
        let (diffuse, specular) = if in_shadow || light_dot_normal < 0.0 {
            (BLACK, BLACK)
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye <= 0.0 {
                (diffuse, BLACK)
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                let specular = light.intensity * self.specular * factor;
                (diffuse, specular)
            }
        };
        ambient + diffuse + specular
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Material) -> bool {
        ((self.pattern.is_none() && other.pattern.is_none())
            || (self.pattern.is_some() && other.pattern.is_some()))
            && self.color == other.color
            && self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.specular == other.specular
            && self.shininess == other.shininess
    }
}

impl fmt::Debug for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Material")
            .field("color", &self.color)
            .field("ambient", &self.ambient)
            .field("diffuse", &self.diffuse)
            .field("specular", &self.specular)
            .field("shininess", &self.shininess)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_material() {
        let m = Material::default();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert!(m.pattern.is_none());
        assert_eq!(m.reflective, 0.0);
        assert_eq!(m.refractive_index, 1.0);
        assert_eq!(m.shininess, 200.0);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.transparency, 0.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let s = Object::new_sphere();
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&s, &light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_offset_45_degrees() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, (2.0_f32).sqrt() / 2.0, (2.0_f32).sqrt() / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let s = Object::new_sphere();
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&s, &light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let s = Object::new_sphere();
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&s, &light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, -(2.0_f32).sqrt() / 2.0, -(2.0_f32).sqrt() / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let s = Object::new_sphere();
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&s, &light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(1.6363853, 1.6363853, 1.6363853));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let s = Object::new_sphere();
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&s, &light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let s = Object::new_sphere();
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let result = m.lighting(&s, &light, position, eyev, normalv, in_shadow);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let mut m = Material::default();
        m.pattern = Some(Pattern::new_stripe(WHITE, BLACK));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let s = Object::new_sphere();
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), WHITE);
        let c1 = m.lighting(
            &s,
            &light,
            Tuple::point(0.9, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        let c2 = m.lighting(&s, &light, Tuple::point(1.1, 0.0, 0.0), eyev, normalv, true);
        assert_eq!(c1, WHITE);
        assert_eq!(c2, BLACK);
    }
}
