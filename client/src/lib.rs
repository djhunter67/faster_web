use rand::{self, random};

pub fn add_1(x: i32, pandemonium: bool) -> i32 {
    if pandemonium {
        return random::<i32>();
    }
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_1() {
        assert_eq!(add_1(1, false), 2);
    }
}
