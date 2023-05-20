//a Hack matrix inversion
//tp LUPDecomposition
pub struct LUPDecomposition {
    pub size: usize,
    pub data: Vec<f64>,
    /// P is a vector of row numbers
    /// P[0] is the row of the matrix that must be returned in row 0 after unpivot
    /// P[1] is the row of the matrix that must be returned in row 1 after unpivot
    pub pivot: Vec<usize>,
}

//ip LUPDecomposition
impl LUPDecomposition {
    //fp new
    /// Create a new LUP decomposition
    pub fn new(matrix: &[f64], size: usize) -> Option<Self> {
        assert!(size != 0);
        assert_eq!(matrix.len(), size * size);
        let mut data = vec![0.; size * size];
        data.copy_from_slice(matrix);
        Self::decompose(&mut data, size).map(|pivot| Self { size, data, pivot })
    }

    //fi decompose
    /// Perform the decomposition
    ///
    /// This generates L and U within the matrix itself; the data for
    /// the two does not overlap as L has 1 on the diagonal and 0s
    /// above, U has 0s below the diagonal
    ///
    /// It also generates the pivot permutation matrix
    ///
    /// The following should hold:
    /// `P . matrix = L . U`
    ///
    /// Should probably have an epsilon for v_max > epsilon requirement
    fn decompose(matrix: &mut [f64], size: usize) -> Option<Vec<usize>> {
        let mut permute = Vec::with_capacity(size);
        for i in 0..size {
            permute.push(i);
        }

        // for each element of the diagonal except the last
        for d in 0..size - 1 {
            // Find the row with the max value in the column d
            let mut v_max = 0.0;
            let mut r_max = None;
            for r in d..size {
                let t = matrix[r * size + d].abs();
                if t > v_max {
                    v_max = t;
                    r_max = Some(r);
                }
            }

            if let Some(r_max) = r_max {
                // Swap row i with r_max and update the pivot list
                if r_max != d {
                    permute.swap(r_max, d);
                    for c in 0..size {
                        matrix.swap(r_max * size + c, d * size + c);
                    }
                }
            } else {
                // not decomposable
                return None;
            }

            // Subtract out from rows below scaling down by LU[d][d] (in p) and up by LU[r][r]
            for r in (d + 1)..size {
                let scale = matrix[r * size + d] / matrix[d * size + d];
                matrix[r * size + d] = scale;
                for c in (d + 1)..size {
                    matrix[r * size + c] -= scale * matrix[d * size + c];
                }
            }
        }
        Some(permute)
    }

    //fp get_l
    /// Get a lower matrix from the decomposition - the diagonals are
    /// 1, and the upper (column > row) is 0.
    #[allow(dead_code)]
    pub fn get_l(&self) -> Vec<f64> {
        let size = self.size;
        let mut result = self.data.clone();
        for r in 0..size {
            for c in r..size {
                if c == r {
                    result[r * size + c] = 1.;
                } else {
                    result[r * size + c] = 0.;
                }
            }
        }
        result
    }

    //fp get_u
    /// Get the upper matrix from the decomposition - the lower (row >
    /// column) is 0.
    #[allow(dead_code)]
    pub fn get_u(&self) -> Vec<f64> {
        let size = self.size;
        let mut result = self.data.clone();
        for r in 1..size {
            for c in 0..r {
                result[r * size + c] = 0.;
            }
        }
        result
    }

    //fp inverse
    //
    // Note that LUP decomposition of M has M = P.L.U
    //
    // If L.U.x(c) (for a column vector x(c)) = I(c) (for the c'th column of the identity matrix)
    // then we can put together the x(c) according to the pivot vector P to generate the inverse.
    //
    // Now, if L.U.x(c) = I(c), then L.y(c) = I(c), where y(c) = U.x(c)
    //
    // Since L is a lower matrix, for column c can construct y with top c-1 elements are 0,
    // whose next element is 1; this will ensure that the top c-1 elements of L.y will be 0,
    // and the c'th element will be 1. Then the c+k'th element of L.y is:
    //  sum(L(c+k,l)*y(l)), which needs to be 0
    // (l=c,c+1,...c+k since y(l<c)=0 and L(c+k,>c+k)=0)
    // Also, L(c+k,c+k)=1
    // Hence y(c+k) = 1/L(c+k,c+k) * -(sum(L(c+k,l)*y(l))) for l=c,c+1,..c+k-1
    // and since L(c+k,c+k) is 1, this simplifies to:
    // y(c+k) = -(sum(L(c+k,l)*y(l))) for l=c,c+1,..c+k-1
    //
    // Now if we have a vector y(c) as above, we know that y(c) = U.x(c), and we need
    // to construct x(c)
    //
    // Note that y(c)(r) = sum(U(l,r)*x(c)(l)) (0<=l<n)
    // with U(<n-1,n-1) being 0, for example, for n by n matrices
    // Hence x(c)(n-1) = y(c)(n-1)/U(c,n-1)
    //
    // Hence again x(c)(r) = (y(c)(r)-sum(U(l,r)*x(c)(l))/U(c,r),
    // for l=r+1..n
    ///
    pub fn inverse(&self, result: &mut Vec<f64>) -> bool {
        assert_eq!(self.data.len(), result.len());
        let size = self.size;
        for c in 0..size {
            // Find y(c) such that L.y = c'th column of I
            // y(c) is held in result[row c]
            for r in 0..size {
                result[c * size + r] = 0.0;
            }
            result[c * size + c] = 1.0;
            for r in 0..size {
                for k in (r + 1)..size {
                    result[c * size + k] -= self.data[k * size + r] * result[c * size + r];
                }
            }

            // Find x(c) such that U.x(c) = y(c)
            // x(c) is held in result[row c]
            for r_m in 0..size {
                let r = size - 1 - r_m;
                let scale = self.data[r * size + r];
                if scale == 0. {
                    return false;
                }
                result[c * size + r] /= scale;
                let x_r = result[c * size + r];
                // For the rest of the column remove multiples of x[r] (Uir, r>i>=0)
                for i in 0..r {
                    result[c * size + i] -= self.data[i * size + r] * x_r;
                }
            }
        }
        true
    }

    //mp invert
    pub fn invert(&self, result: &mut Vec<f64>) -> bool {
        let size = self.size;
        assert_eq!(result.len(), size * size);
        let mut temp = self.data.clone();
        if self.inverse(&mut temp) {
            for c in 0..size {
                // L.U.x(c) = c'th column of I; hence R[P[c]] = x(c)
                let p_c = self.pivot[c];
                for r in 0..size {
                    result[r * size + p_c] = temp[c * size + r];
                }
            }
            true
        } else {
            false
        }
    }

    //mp unpivot
    #[allow(dead_code)]
    pub fn unpivot(&self) -> Vec<f64> {
        let size = self.size;
        let mut result = self.data.clone();
        for r in 0..size {
            let pr = self.pivot[r];
            for c in 0..size {
                result[r * size + c] = self.data[pr * size + c];
            }
        }
        result
    }
}

//a Test
#[cfg(test)]
mod test_lup {
    use super::LUPDecomposition;
    //ft m_mult_v
    #[allow(dead_code)]
    fn m_mult_v<const D: usize, const D2: usize>(m: &[f64; D2], v: &[f64; D]) -> [f64; D] {
        let mut r = [0.; D];
        for n in 0..D {
            let mut x = 0.;
            for j in 0..D {
                x += m[n * D + j] * v[j];
            }
            r[n] = x;
        }
        r
    }

    //ft m_mult_m
    fn m_mult_m<const D: usize>(m1: &[f64], m2: &[f64], r: &mut [f64]) {
        for i in 0..D {
            for j in 0..D {
                let mut x = 0.;
                for k in 0..D {
                    x += m1[i * D + k] * m2[k * D + j];
                }
                r[i * D + j] = x;
            }
        }
    }

    //ft dist_identity
    fn dist_identity<const D: usize>(m: &[f64]) -> f64 {
        let mut x = 0.;
        for i in 0..D {
            for j in 0..D {
                let v = m[i * D + j];
                if i == j {
                    x += (v - 1.).abs();
                } else {
                    x += v.abs();
                }
            }
        }
        x
    }

    //ft test_inversion
    fn test_inversion<const D2: usize, const D: usize>(v: &[f64]) {
        let lup = LUPDecomposition::new(&v, D).unwrap();
        let mut r = vec![0.; D2];
        assert!(lup.invert(&mut r));
        let mut id = [0.; D2];
        m_mult_m::<D>(&v, &r, &mut id);
        println!("{:?}", id);
        assert!(dist_identity::<D>(&id) < 0.00001);
    }
    //ft test_0
    #[test]
    fn test_0() {
        let lup = LUPDecomposition::new(&[1., 0., 0., 1.], 2).unwrap();
        assert_eq!(lup.size, 2);
        assert_eq!(lup.data, vec![1., 0., 0., 1.]);
        assert_eq!(lup.pivot, vec![0, 1]);
    }
    //ft test_1
    #[test]
    fn test_1() {
        let lup = LUPDecomposition::new(&[0., 1., 1., 0.], 2).unwrap();
        assert_eq!(lup.size, 2);
        assert_eq!(lup.data, vec![1., 0., 0., 1.]);
        assert_eq!(lup.pivot, vec![1, 0]);
    }
    //ft test_2
    #[test]
    fn test_2() {
        let lup = LUPDecomposition::new(&[4., 3., 6., 3.], 2).unwrap();
        assert_eq!(lup.size, 2);
        assert_eq!(lup.data, vec![6., 3., 2. / 3., 1.]);
        assert_eq!(lup.pivot, vec![1, 0]);
    }
    //ft test_3
    #[test]
    fn test_3() {
        let lup = LUPDecomposition::new(&[8., 16., 2., 4.], 2).unwrap();
        assert_eq!(lup.size, 2);
        assert_eq!(lup.data, vec![8., 16., 0.25, 0.]);
        assert_eq!(lup.pivot, vec![0, 1]);
    }
    //ft test_4
    #[test]
    fn test_4() {
        let lup = LUPDecomposition::new(&[1., 0., 1., 0., 1., 0., 0., 0., 2.], 3).unwrap();
        assert_eq!(lup.size, 3);
        assert_eq!(lup.data, vec![1., 0., 1., 0., 1., 0., 0., 0., 2.]);
        assert_eq!(lup.pivot, vec![0, 1, 2]);
    }
    //ft test_5
    #[test]
    fn test_5() {
        let lup = LUPDecomposition::new(&[1., 0., 1., 4., 1., 0., 0., 0., 2.], 3).unwrap();
        assert_eq!(lup.size, 3);
        assert_eq!(lup.data, vec![4., 1., 0., 0.25, -0.25, 1., 0., 0., 2.]);
        assert_eq!(lup.pivot, vec![1, 0, 2]);
        let mut r = vec![0.; 9];
        assert!(lup.invert(&mut r));
        assert_eq!(r, vec![1., 0., -0.5, -4., 1., 2., 0., 0., 0.5]);
    }
    //ft test_6
    #[test]
    fn test_6() {
        let v = [1., 0., 1., 4., 1., 0., 0., 0., 2.];
        let lup = LUPDecomposition::new(&v, 3).unwrap();
        let mut r = vec![0.; 9];
        assert!(lup.invert(&mut r));
        let mut id = [0.; 9];
        m_mult_m::<3>(&v, &r, &mut id);
        assert!(dist_identity::<3>(&id) < 0.00001);
    }
    //ft test_7
    #[test]
    fn test_7() {
        test_inversion::<4, 2>(&[1., 0., 1., 4.]);
        test_inversion::<4, 2>(&[4., 1., 1., 9.]);
        test_inversion::<4, 2>(&[3., 2., 1., 6.]);
        test_inversion::<4, 2>(&[7., 3., 0., 3.]);
    }
    //ft test_8
    #[test]
    fn test_8() {
        test_inversion::<9, 3>(&[1., 0., 1., 4., 1., 0., 0., 0., 2.]);
        test_inversion::<9, 3>(&[4., 1., 1., 9., 2., 3., 4., 3., 5.]);
        test_inversion::<9, 3>(&[3., 2., 1., 6., 1., 8., 6., 5., 4.]);
        test_inversion::<9, 3>(&[7., 3., 0., 3., 2., 5., 1., 2., 5.]);
    }
    //ft test_9
    #[test]
    fn test_9() {
        test_inversion::<36, 6>(&[
            1., 0., 1., 4., 1., 0., 0., 0., 2., 4., 1., 1., 9., 2., 3., 4., 3., 5., 3., 2., 1., 6.,
            1., 8., 6., 5., 4., 7., 3., 0., 3., 2., 5., 1., 2., 5.,
        ]);
    }
}
