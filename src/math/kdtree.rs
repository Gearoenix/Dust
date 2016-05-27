use std;

use ::math::aabbox::{
    AABBox3,
    ExpandableToOther,
};
use ::math::triangle::{
    Triangle,
};
use ::math::ray::Ray3;
use ::math::vector::{
    Vec3,
    Axis,
};
use ::render::vertex::Vertex;

pub struct KDNode {
    pub area:    AABBox3,
    pub left:    Option<Box<KDNode>>,
    pub right:   Option<Box<KDNode>>,
    pub indices: Vec<usize>,
}

impl KDNode {
    pub fn new() -> KDNode {
        KDNode {
            area:    AABBox3::new(),
            left:    None,
            right:   None,
            indices: Vec::new(),
        }
    }

    pub fn build(indices: &Vec<usize>, vertices: &Vec<Vertex>, triangles: &Vec<Triangle>) -> Option<Box<KDNode>> {
        let mut node = KDNode::new();
        if indices.len() < 1 {
            return None;
        }
        if indices.len() < 2 {
            node.indices.push(indices[0]);
            node.area = triangles[indices[0]].get_aabb(vertices);
            return Some(Box::new(node));
        }

        node.area = triangles[indices[0]].get_aabb(vertices);
        let mut midpt = Vec3::new();
        let tris_recp = 1f64 / (indices.len() as f64);

        for index in indices {
            node.area.expand(&triangles[*index].get_aabb(vertices));
            midpt += triangles[*index].get_midpoint(vertices) * tris_recp;
        }

        let mut left_indices = Vec::new();
        let mut right_indices = Vec::new();
        let axis = node.area.get_longest_axis();

        // TODO for performance imporvement: put match block out of the for loop.
        for index in indices {
            match axis {
                Axis::X => if midpt.x >= triangles[*index].get_midpoint(vertices).x { right_indices.push(*index); } else { left_indices.push(*index); },
                Axis::Y => if midpt.y >= triangles[*index].get_midpoint(vertices).y { right_indices.push(*index); } else { left_indices.push(*index); },
                Axis::Z => if midpt.z >= triangles[*index].get_midpoint(vertices).z { right_indices.push(*index); } else { left_indices.push(*index); },
                _ => panic!("Unexpected Axis value.")
            }
        }

        if left_indices.len() == 0 || right_indices.len() == 0 {
            node.indices = indices.clone();
            // TODO for performance imporvement: I thinck these following three lines have redundant calculating,
            //      I already calulated them, I ave doubt about it..
            return Some(Box::new(node));
        }
        node.left = KDNode::build(&left_indices, vertices, triangles);
        node.right = KDNode::build(&right_indices, vertices, triangles);
        Some(Box::new(node))
    }

    pub fn hit(node: &KDNode, ray: &Ray3, tmin: f64, vertices: &Vec<Vertex>, triangles: &Vec<Triangle>) -> Option<(f64, f64, f64, usize)> {
        let (does_inter, dist) = node.area.intersection(ray);
        if !does_inter {
            return None;
        }
        if dist.gt(&tmin) {
            return None;
        }
        match node.left {
            Some(ref left_node_p) => {
                let l = KDNode::hit(&*left_node_p, ray, tmin, vertices, triangles);
                if l.is_some() {
                    return l;
                }
            }
            None => {}
        }
        match node.right {
            Some(ref right_node_p) => {
                return KDNode::hit(&*right_node_p, ray, tmin, vertices, triangles);
            }
            None => {}
        }
        let mut hit_tri = false;
        let mut tri_ind: usize = 0;
        let mut t = tmin;
        let mut u = 0f64;
        let mut v = 0f64;
        for index in node.indices.iter() {
            match triangles[*index].intersect(ray, tmin, vertices) {
                Some((_t, _u, _v)) => {
                    if t.lt(&_t) {
                        hit_tri = true;
                        tri_ind = *index;
                        t = _t;
                        u = _u;
                        v = _v;
                    }
                }
                None => {}
            }
        }
        if hit_tri {
            return Some((t, u, v, tri_ind));
        }
        return None;
    }
}
