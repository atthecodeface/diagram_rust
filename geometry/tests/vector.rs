extern crate geometry;

use geometry::{Vector, Vector3D};
use std::marker::PhantomData;
struct Banana<V:Vector3D<f32>> {p:PhantomData<V>}
impl <V3:Vector<f32,3>, V:Vector3D<f32, Vec3 = V3>> Banana<V> {
    fn test_vec3() {
        let a  =  V3::zero();
        let b  =  V3::from_array([3.,4.,12.]);
        let c  =  V3::from_array([1.,1.,1.]);
        assert_eq!(a.length(), 0., "Length of zero vector is 0");
        assert_eq!(a.length_sq(), 0., "Length^2 of zero vector is 0");
        assert_eq!(b.length(), 13., "Length of 3,4,12 vector is 13");
        assert_eq!(b.length_sq(), 169., "Length^2 of 3,4,12 vector is 13");
        assert_eq!(b.distance(&a), 13., "Separation is 13");
        assert_eq!(b.distance_sq(&a), 169., "Separation^2 is 13");
        assert_eq!(c.length_sq(), 3., "Length^2 of 1,1,1 vector is 3");
        assert_eq!((b * 0.).length(), 0., "Length of thing*0 is 0");
        assert_eq!((b * 1.).length(), 13., "Length of b*1 is 13");
        assert_eq!((b * 2.).length(), 26., "Length of b*2 is 26");
        assert_eq!((b * c).length(), 13., "Length of b*c is 13");
        assert_eq!((b.distance_sq(&c)), 134., "Distance^2 of b-cc is 134");
    }
}

#[test]
fn test_fslice() {
    Banana::<f32>::test_vec3();
}

#[cfg(feature="simd")]
extern crate core_simd;
#[cfg(feature="simd")]
mod test_simd {
    use geometry::simd::{VecF32A16};
    #[test]
    fn test_simd() {
        super::Banana::<VecF32A16>::test_vec3();
    }
}
