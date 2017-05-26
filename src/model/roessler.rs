//! Roessler system
//! https://en.wikipedia.org/wiki/Lorenz_syste://en.wikipedia.org/wiki/R%C3%B6ssler_attractor

use ndarray::*;
use traits::EOM;

#[derive(Clone,Copy,Debug)]
pub struct Roessler {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Default for Roessler {
    fn default() -> Self {
        Roessler {
            a: 0.2,
            b: 0.2,
            c: 5.7,
        }
    }
}

impl Roessler {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Roessler { a: a, b: b, c: c }
    }
}

impl<'a> EOM<f64, OwnedRcRepr<f64>, Ix1> for &'a Roessler {
    #[inline(always)]
    fn rhs(self, mut v: RcArray<f64, Ix1>) -> RcArray<f64, Ix1> {
        let x = v[0];
        let y = v[1];
        let z = v[2];
        v[0] = -y - z;
        v[1] = x + self.a * y;
        v[2] = self.b + x * z - self.c * z;
        v
    }
}