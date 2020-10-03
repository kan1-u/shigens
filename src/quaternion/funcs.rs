use super::*;

pub fn r<T>(x: T) -> R<T> {
    R::from(x)
}

pub fn i<T>(x: T) -> I<T> {
    I::from(x)
}

pub fn j<T>(x: T) -> J<T> {
    J::from(x)
}

pub fn k<T>(x: T) -> K<T> {
    K::from(x)
}

pub fn quaternion<T>(r: T, i: T, j: T, k: T) -> Quaternion<T> {
    Quaternion::from((r, i, j, k))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn rijk_test() {
        assert_eq!(r(1).0, 1);
        assert_eq!(i(1).0, 1);
        assert_eq!(j(1).0, 1);
        assert_eq!(k(1).0, 1);
    }

    #[test]
    fn quaternion_test() {
        let q = quaternion(1, 2, 3, 4);
        assert_eq!(q.r, r(1));
        assert_eq!(q.i, i(2));
        assert_eq!(q.j, j(3));
        assert_eq!(q.k, k(4));
    }
}
