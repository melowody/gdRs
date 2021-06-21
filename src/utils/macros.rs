#[macro_export]
macro_rules! remove {
    ($item:expr,$other:expr) => {
        {
            match $item {
                Some(x) => x,
                None => $other
            }
        }
    };
}