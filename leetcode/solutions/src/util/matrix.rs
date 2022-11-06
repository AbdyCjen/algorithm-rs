#[macro_export]
macro_rules! matrix {
    ($($e:expr),*) => {vec![$($e.iter().copied().collect::<Vec<_>>()), *]};
    ($($e:expr,)*) => {vec![$($e.iter().copied().collect::<Vec<_>>()), *]};
}
