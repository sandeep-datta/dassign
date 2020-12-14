// $ ( ... ) sep rep
// rustc -Z unstable-options --pretty expanded recurrence.rs
#[macro_export]
macro_rules! dassign {
    (($a:ident, $b:ident) = $e:expr) => {
        {
            let retv = $e;
            $a = retv.0;
            $b = retv.1;
        }
    };
}


#[cfg(test)]
mod tests {
    fn make_tuple<T>(a: T, b: T) -> (T, T) {
        (a, b)
    }

    #[test]
    fn it_works() {
        let mut x;
        let mut y;

        dassign!((x, y) = make_tuple(1, 2));
        assert_eq!((x, y), (1, 2));

        dassign!((x, y) = make_tuple(3, 4));
        assert_eq!((x, y), (3, 4));
    }
}
