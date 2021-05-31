#[macro_export]
macro_rules! vec_char {
    ($($e:expr),*) => {vec![$(std::char::from_digit($e, 10).unwrap()), *]};
    ($($e:expr,)*) => {vec![$(std::char::from_digit($e, 10).unwrap()), *]};
}
