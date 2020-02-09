fn main() {

}

fn angle(n: u32) -> u32 {
   180 * (n-2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle() {
        assert_eq!(angle(3), 180);
        assert_eq!(angle(4), 360);
    }
}
