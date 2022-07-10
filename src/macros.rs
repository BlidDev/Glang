#[macro_export]
macro_rules! commands {
    (($q:expr,$a:expr), {$($k:ident => ($f:expr,$n:expr)),* $(,)?}) => {
        {
            $(
                add_command(&mut $q, &mut $a,stringify!($k),$f,$n);
            )*
        }
    };
}


