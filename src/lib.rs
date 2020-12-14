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
