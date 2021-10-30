use std::iter::repeat_with;

pub fn get_random_str(len: usize) -> String {
    return repeat_with(fastrand::alphanumeric).take(len)
        .collect();
}
