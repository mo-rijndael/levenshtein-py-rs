#![warn(clippy::pedantic, clippy::nursery)]
extern crate cpython;


use std::cmp::min;
use cpython::{PyResult, Python, py_module_initializer, py_fn};

py_module_initializer!(liblevenshtein, |py, m| {
        m.add(py, "distance", py_fn!(py, distance_py(first: &str, second: &str)))?;
        Ok(())
    }
);
fn distance(first: &str, second: &str) -> usize {
    //first must be shortest
    let (first, second) = if second.len() < first.len() {
        (second.chars().collect::<Box<[_]>>(), first.chars().collect::<Box<[_]>>())
    }else{
        (first.chars().collect(), second.chars().collect())
    };
    let horizontal = first.len();
    let vertical = second.len();
    let mut matrix = vec![vec![0; horizontal+1].into_boxed_slice(); 2].into_boxed_slice();
    for a in 0..=horizontal {
        matrix[0][a] = a;
    }
    for i in 0..vertical {
        matrix[1][0] = matrix[0][0] +1;
        for j in 1..=horizontal {
            let add = matrix[0][j] +1;
            let remove = matrix[1][j-1] +1;
            let replace = if second[i] == first[j-1] {
                matrix[0][j-1]
            }else{
                matrix[0][j-1] +1
            };
            matrix[1][j] = min(min(add, remove),replace);
        }
        matrix.swap(0,1);
    };
    matrix[0][horizontal]
}
fn distance_py(_: Python, first: &str, second: &str) -> PyResult<usize> {
    let out = distance(first, second);
    Ok(out)
}

#[cfg(test)]
mod test {
    use super::distance;
    #[test]
    fn compare_1() {
        let result = distance("sas", "sos");
        assert_eq!(result, 1)
    }
    #[test]
    fn compare_max() {
        let result = distance("aaaa", "bbbb");
        assert_eq!(result, 4)
    }
    #[test]
    fn different_lens() {
        let result = distance("aaaa", "a");
        assert_eq!(result, 3)
    }
}
