use crate::objects::Object;
use crate::tuples::Tuple;

pub struct ObjFile {
    pub default_group: Object,
    #[allow(dead_code)]
    normals: Vec<Tuple>,
    #[allow(dead_code)]
    vertices: Vec<Tuple>,
}

pub fn parse_obj_file_path(path: &str) -> ObjFile {
    parse_obj_file(&std::fs::read_to_string(path).unwrap())
}

fn parse_obj_file(lines: &str) -> ObjFile {
    let mut default_group = Object::new_group();
    let mut normals = vec![];
    let mut vertices = vec![];
    let mut current_group = &mut default_group;
    for line in lines.lines() {
        let mut words = line.split_whitespace();
        match words.next() {
            Some("v") => {
                let x = words.next().unwrap().parse().unwrap();
                let y = words.next().unwrap().parse().unwrap();
                let z = words.next().unwrap().parse().unwrap();
                vertices.push(Tuple::point(x, y, z));
            }
            Some("vn") => {
                let x = words.next().unwrap().parse().unwrap();
                let y = words.next().unwrap().parse().unwrap();
                let z = words.next().unwrap().parse().unwrap();
                normals.push(Tuple::vector(x, y, z));
            }
            Some("f") => {
                let mut indices: Vec<usize> = vec![];
                let mut normal_indices: Vec<usize> = vec![];
                for word in words {
                    if word.contains('/') {
                        let mut ints = word.split('/');
                        indices.push(ints.next().unwrap().parse().unwrap());
                        ints.next();
                        normal_indices.push(ints.next().unwrap().parse().unwrap());
                    } else {
                        indices.push(word.parse().unwrap());
                    }
                }
                fan_triangulation(&vertices, &normals, indices, normal_indices, current_group);
            }
            Some("g") => {
                let new_group = Object::new_group();
                default_group.as_mut_group().add_child(new_group);
                current_group = default_group.as_mut_group().children.last_mut().unwrap();
            }
            _ => {}
        }
    }
    ObjFile {
        default_group,
        normals,
        vertices,
    }
}

fn fan_triangulation(
    vertices: &[Tuple],
    normals: &[Tuple],
    indices: Vec<usize>,
    normal_indices: Vec<usize>,
    group: &mut Object,
) {
    for i in 1..indices.len() - 1 {
        let p1 = vertices[indices[0] - 1];
        let p2 = vertices[indices[i] - 1];
        let p3 = vertices[indices[i + 1] - 1];
        if !normal_indices.is_empty() {
            let n1 = normals[normal_indices[0] - 1];
            let n2 = normals[normal_indices[i] - 1];
            let n3 = normals[normal_indices[i + 1] - 1];
            group
                .as_mut_group()
                .add_child(Object::new_smooth_triangle(p1, p2, p3, n1, n2, n3));
        } else {
            group
                .as_mut_group()
                .add_child(Object::new_triangle(p1, p2, p3));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignoring_unrecognized_lines() {
        let gibberish = "There was a young lady named Bright
who traveled much faster than light.
She set out one day
in a relative way,
and came back the previous night.";
        parse_obj_file(gibberish);
    }

    #[test]
    fn vertex_records() {
        let vertex_records = "v -1 1 0
v -1.0000 0.5000 0.0000
v 1 0 0
v 1 1 0";
        let obj = parse_obj_file(vertex_records);
        assert_eq!(
            obj.vertices,
            vec![
                Tuple::point(-1.0, 1.0, 0.0),
                Tuple::point(-1.0, 0.5, 0.0),
                Tuple::point(1.0, 0.0, 0.0),
                Tuple::point(1.0, 1.0, 0.0),
            ]
        );
    }

    #[test]
    fn parsing_triangle_faces() {
        let lines = "v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
f 1 2 3
f 1 3 4";
        let obj = parse_obj_file(lines);
        let g = obj.default_group;
        let t1 = g.as_group().children[0].as_triangle();
        let t2 = g.as_group().children[1].as_triangle();
        assert_eq!(t1.p1, obj.vertices[0]);
        assert_eq!(t1.p2, obj.vertices[1]);
        assert_eq!(t1.p3, obj.vertices[2]);
        assert_eq!(t2.p1, obj.vertices[0]);
        assert_eq!(t2.p2, obj.vertices[2]);
        assert_eq!(t2.p3, obj.vertices[3]);
    }

    #[test]
    fn triangulating_polygons() {
        let lines = "v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0
f 1 2 3 4 5";
        let obj = parse_obj_file(lines);
        let g = obj.default_group;
        let t1 = g.as_group().children[0].as_triangle();
        let t2 = g.as_group().children[1].as_triangle();
        let t3 = g.as_group().children[2].as_triangle();
        assert_eq!(t1.p1, obj.vertices[0]);
        assert_eq!(t1.p2, obj.vertices[1]);
        assert_eq!(t1.p3, obj.vertices[2]);
        assert_eq!(t2.p1, obj.vertices[0]);
        assert_eq!(t2.p2, obj.vertices[2]);
        assert_eq!(t2.p3, obj.vertices[3]);
        assert_eq!(t3.p1, obj.vertices[0]);
        assert_eq!(t3.p2, obj.vertices[3]);
        assert_eq!(t3.p3, obj.vertices[4]);
    }

    #[test]
    fn triangles_in_groups() {
        let lines = "v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4";
        let obj = parse_obj_file(lines);
        let g1 = obj.default_group.as_group().children[0].as_group();
        let g2 = obj.default_group.as_group().children[1].as_group();
        let t1 = g1.children[0].as_triangle();
        let t2 = g2.children[0].as_triangle();
        assert_eq!(t1.p1, obj.vertices[0]);
        assert_eq!(t1.p2, obj.vertices[1]);
        assert_eq!(t1.p3, obj.vertices[2]);
        assert_eq!(t2.p1, obj.vertices[0]);
        assert_eq!(t2.p2, obj.vertices[2]);
        assert_eq!(t2.p3, obj.vertices[3]);
    }

    #[test]
    fn vertex_normal_records() {
        let lines = "vn 0 0 1
vn 0.707 0 -0.707
vn 1 2 3";
        let obj = parse_obj_file(lines);
        assert_eq!(obj.normals[0], Tuple::vector(0.0, 0.0, 1.0));
        assert_eq!(obj.normals[1], Tuple::vector(0.707, 0.0, -0.707));
        assert_eq!(obj.normals[2], Tuple::vector(1.0, 2.0, 3.0));
    }

    #[test]
    fn faces_with_normals() {
        let lines = "v 0 1 0
v -1 0 0
v 1 0 0
vn -1 0 0
vn 1 0 0
vn 0 1 0
f 1//3 2//1 3//2
f 1/0/3 2/102/1 3/14/2";
        let obj = parse_obj_file(lines);
        let g = obj.default_group;
        let t1 = g.as_group().children[0].as_smooth_triangle();
        let t2 = g.as_group().children[1].as_smooth_triangle();
        assert_eq!(t1.p1, obj.vertices[0]);
        assert_eq!(t1.p2, obj.vertices[1]);
        assert_eq!(t1.p3, obj.vertices[2]);
        assert_eq!(t1.n1, obj.normals[2]);
        assert_eq!(t1.n2, obj.normals[0]);
        assert_eq!(t1.n3, obj.normals[1]);
        assert_eq!(t2, t1);
    }
}
