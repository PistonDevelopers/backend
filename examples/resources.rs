/*
Demonstrates a pattern where resources of types
are connected to the associated types of various traits.

The individual traits can be implemented separately from the backend.
A `Resources` trait has a role of integrating the types from various traits.

This pattern can be used when associated types are shared across backends.
*/

#[macro_use]
extern crate backend;

pub trait Resources {
    type Vec2;
}

pub trait VecMath {
    type Vec2;

    fn add_vec2(&self, a: Self::Vec2, b: Self::Vec2) -> Self::Vec2;
}

backend!(
    Resources [Resources],
    VecMath [VecMath<Vec2 = <Self::Resources as Resources>::Vec2>]
);

pub struct Math;

backend_impl!(
    Math {
        Resources, VecMath = Math
    }
);

impl Resources for Math {
    type Vec2 = [f64; 2];
}

impl VecMath for Math {
    type Vec2 = [f64; 2];

    fn add_vec2(&self, a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
        [a[0] + b[0], a[1] + b[1]]
    }
}

fn add<V: VecMath>(v: &V, a: V::Vec2, b: V::Vec2) -> V::Vec2 {
    v.add_vec2(a, b)
}

pub fn main() {
    let res = Math.add_vec2([0.0, 1.0], [1.0, 0.0]);
    let res2 = add(&Math, [0.0, 1.0], [1.0, 0.0]);
    assert_eq!(res, res2);
}
