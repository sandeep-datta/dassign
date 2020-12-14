// $ ( ... ) sep rep
// rustc -Z unstable-options --pretty expanded recurrence.rs
#[macro_export]
macro_rules! dassign {
    (($($i:ident),+) = $e:expr) => {
        {
            use ::paste::paste;
            paste! {
                let ($([<_ $i>], )+) = $e;

                $(
                    $i = [<_ $i>];
                )+
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut x;
        let mut y;
        let mut z;

        dassign!((x, y) = (1, 2));
        assert_eq!((x, y), (1, 2));

        dassign!((x, y) = (3, 4));
        assert_eq!((x, y), (3, 4));

        dassign!((x, y, z) = (5, 6, 7));
        assert_eq!((x, y, z), (5, 6, 7));

        dassign!((x, y, z) = (8, 9, 10));
        assert_eq!((x, y, z), (8, 9, 10));
    }
}
