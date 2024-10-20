use std::ops;

pub fn roundup<T1, T2>(value: T1, base: T2) -> T1
where
    T1: ops::Add<Output = T1>
        + ops::BitAnd<Output = T1>
        + ops::Not<Output = T1>
        + Copy
        + From<T2>,
    T2: Copy + ops::Sub<usize, Output = T2>,
{
    (value + T1::from(base - 1)) & !T1::from(base - 1)
}
