/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    grid.rs
@brief   A set of equations for links
 */

//a Imports
use super::LUPDecomposition;

//a EquationSet
//tp EquationSet
/// A set of equations for solving a spring-based grid system
///
/// The equation set is derived from a Hooke's Law energy for each spring, which is:
/// ```raw
///    ((b-a)-length) ^ 2 * elasticity
/// ```
/// for each spring with ends at `a` and `b`
///
/// The change in energy of the spring by a change `da` (moving `a` a touch) is then
/// ```raw
///    (a - b + length) * elasticity * 2
/// ```
/// and by change `db`
/// ```raw
///    (b - a - length) * elasticity * 2
/// ```
///
/// The total energy of the system then is effected by `da`, `db`, etc
/// as a set of equations from the sum of spring energies.
///
/// The minimum energy of the system occurs when dE/dX = 0 for all X.
///
/// Each equation is linear on the positions `a`, `b`, etc; the
/// [EquationSet] hence is a simple set of linear equations, with a
/// standard solution method.
///
/// The equations are not independent though; the whole spring system
/// can be translated by any value and the energy will be the
/// same. Hence at least one of the positions must be 'nailed down'.
///
/// Furthermore, to allow for the springs to stretch from their
/// natural length, another spring end must be `nailed down` to
/// stretch the whole spring system between the two ends. This is
/// accomplished using the `force_value` method.
pub struct EquationSet {
    /// The number of equations
    size: usize,
    /// The matrix of coefficients, length size*size
    ///
    /// This holds the inverse
    matrix: Vec<f64>,
    /// The vector of result values, length size*size
    results: Vec<f64>,
}

//ip EquationSet
impl EquationSet {
    //fp new
    /// Create an [EquationSet] for a given number of equations
    pub fn new(size: usize) -> Self {
        let matrix = vec![0.0f64; size * size];
        let results = vec![0.0f64; size];
        Self {
            size,
            matrix,
            results,
        }
    }

    //fp add_growth_link
    /// Add a link with some growth between two elements
    pub fn add_growth_link(&mut self, start: usize, end: usize, length: f64, growth: f64) {
        let size = self.size;
        let inv_growth = 1.0 / growth;

        self.matrix[start * size + start] += inv_growth;
        self.matrix[start * size + end] -= inv_growth;
        self.results[start] -= inv_growth * length;

        self.matrix[end * size + start] -= inv_growth;
        self.matrix[end * size + end] += inv_growth;
        self.results[end] += inv_growth * length;
    }

    //fp force_value
    /// Force index `n` to have a certain `value`
    ///
    /// This will make its row be 0 0 0 .. 1 .. 0 0 and the result
    /// `value`
    ///
    pub fn force_value(&mut self, n: usize, value: f64) {
        let size = self.size;
        for i in 0..size {
            self.matrix[n * size + i] = 0.;
        }
        self.matrix[n * size + n] = 1.;
        self.results[n] = value;
    }

    //fi column_is_zero
    /// Return true if the column is zero
    #[allow(dead_code)]
    pub fn column_is_zero(&self, n: usize) -> bool {
        let size = self.size;
        for i in 0..size {
            if self.matrix[i * size + n] != 0. {
                return false;
            }
        }
        true
    }

    //fi row_is_zero
    /// Return true if the row is zero
    pub fn row_is_zero(&self, n: usize) -> bool {
        let size = self.size;
        for i in 0..size {
            if self.matrix[n * size + i] != 0. {
                return false;
            }
        }
        true
    }

    //fp invert
    /// Invert the equation set matrix, returning an error if not possible
    ///
    /// Once inverted the equation set can be resolved by applying the
    /// matrix. If an inversion is not possible then the equation set
    /// *cannot* be resolved - it is over- or under-constrained
    pub fn invert(&mut self) -> Result<(), String> {
        if let Some(lup) = LUPDecomposition::new(&self.matrix, self.size) {
            if lup.invert(&mut self.matrix) {
                Ok(())
            } else {
                Err("Failed to invert after decomposition, not invertible".into())
            }
        } else {
            Err("Failed to decompose, not invertible".into())
        }
    }

    //fp solve
    /// Solve the equation set, or return an error if not feasible
    pub fn solve(&mut self) -> Result<(), String> {
        self.invert()?;
        let size = self.size;
        let mut results = Vec::with_capacity(size);
        for n in 0..size {
            let mut x = 0.;
            for j in 0..size {
                x += self.matrix[n * size + j] * self.results[j];
            }
            results.push(x);
        }
        // std::mem::replace(&mut self.results, results);
        self.results = results;
        Ok(())
    }

    //fp results
    /// Iterate through the results, returning an iterator of (usize, f64)
    pub fn results(&self) -> std::iter::Enumerate<std::slice::Iter<'_, f64>> {
        self.results.iter().enumerate()
    }

    //zz All done
}

//ip Display
impl std::fmt::Display for EquationSet {
    //mp fmt
    /// Display for humans
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.size {
            write!(f, "\n|")?;
            for j in 0..self.size {
                write!(f, " {:8}", self.matrix[i * self.size + j])?;
            }
            write!(f, "|  ({:8})", self.results[i])?;
        }
        writeln!(f)
    }
}
