
extern crate num;

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
    MathVector,
    VectorElement,
    Axis,
};
use ::render::vertex::{
    HasPosition,
};

pub struct KDNode<E, T> where E: VectorElement, T: Triangle<E> {
    pub area:      AABBox3<E>,
    pub left:      Option<Box<KDNode<E, T>>>,
    pub right:     Option<Box<KDNode<E, T>>>,
    pub triangles: Vec<T>,
    pub leaf:      bool // !!!!! I don't know that it is necesary
}

impl<E, T> KDNode<E, T> where E: VectorElement, T: Triangle<E> {

    pub fn new() -> KDNode<E, T> {
        KDNode {
            area: AABBox3::new(),
            left: None,
            right: None,
            triangles: Vec::new(),
            leaf: false,
        }
    }

    pub fn build<V>(tris: &Vec<T>, depth: &i32, vertices: &Vec<V>) -> Option<Box<KDNode<E, T>>> where V: HasPosition<E> {
        let mut node = KDNode::new();
        if tris.len() == 0 {
            return None;
        }
        if tris.len() < 2 {
            node.triangles = (*tris).clone();
            node.leaf = true;
            node.area = tris[0].get_aabb(vertices);
            for tri in tris {
                node.area.expand(&tri.get_aabb(vertices));
            }
            return Some(Box::new(node));
        }

        node.area = tris[0].get_aabb(vertices);
        let mut midpt = Vec3::new(num::cast(0).unwrap());
        let tris_recp = num::cast::<i8, E>(1).unwrap() / num::cast(tris.len()).unwrap();

        for tri in tris {
            node.area.expand(&tri.get_aabb(vertices));
            midpt += tri.get_midpoint(vertices) * tris_recp;
        }

        let mut left_tris = Vec::<T>::new();
        let mut right_tris = Vec::<T>::new();
        let axis = node.area.get_longest_axis();

        // TODO for performance imporvement: put match block out of the for loop.
        for tri in tris {
            match axis {
                Axis::X => if midpt.x >= tri.get_midpoint(vertices).x { right_tris.push(*tri); } else { left_tris.push(*tri); },
                Axis::Y => if midpt.y >= tri.get_midpoint(vertices).y { right_tris.push(*tri); } else { left_tris.push(*tri); },
                Axis::Z => if midpt.z >= tri.get_midpoint(vertices).z { right_tris.push(*tri); } else { left_tris.push(*tri); },
                _ => panic!("Unexpected Axis value.")
            }
        }

        if tris.len() == left_tris.len() || tris.len() == right_tris.len() {
            node.triangles = (*tris).clone();
            node.leaf = true;
            // TODO for performance imporvement: I thinck these following three lines have redundant calculating,
            //      I already calulated them, I ave doubt about it..
            node.area = tris[0].get_aabb(vertices);
            for tri in tris {
                node.area.expand(&tri.get_aabb(vertices));
            }
            return Some(Box::new(node));
        }
        node.left = KDNode::build(&left_tris, &((*depth) + 1), vertices);
        node.right = KDNode::build(&right_tris, &((*depth) + 1), vertices);
        Some(Box::new(node))
    }

    pub fn hit<V>(node: &KDNode<E, T>, ray: &Ray3<E>, tmin: &E, vertices: &Vec<V>) -> Option<(E, usize)> where V: HasPosition<E> {
        let (does_inter, dist) = node.area.intersection(ray);
        if does_inter {
            if dist.gt(tmin) {
                return None;
            }
            if !(node.leaf) {
                match node.left {
                    Some(ref left_node_p) => {
                        let l = KDNode::hit(&*left_node_p, ray, tmin, vertices);
                        if l.is_some() {
                            return l;
                        }
                    }
                    None => {}
                }

                match node.right {
                    Some(ref right_node_p) => {
                        return KDNode::hit(&*right_node_p, ray, tmin, vertices);
                    }
                    None => {}
                }
                return None;
            }
            else {
                let mut tmp_t = *tmin;
                let mut hit_tri = false;
                let mut tri_ind: usize = 0;
                for (i, tri) in node.triangles.iter().enumerate() {
                    match tri.intersect(ray, tmin, vertices) {
                        Some((t, _, _)) => {
                            if tmp_t.lt(&t) {
                                hit_tri = true;
                                tmp_t = t;
                                tri_ind = i;
                            }
                        }
                        None => {}
                    }
                }
                if hit_tri {
                    return Some((tmp_t, tri_ind));
                }
            }
        }
        return None;
    }
}
