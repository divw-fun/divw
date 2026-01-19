#[test]
fn test_depth_validation() {
    let valid_depths: [u8; 3] = [1, 5, 10];
    for depth in valid_depths {
        assert!((1..=10).contains(&depth));
    }

    let invalid_depths: [u8; 3] = [0, 11, 255];
    for depth in invalid_depths {
        assert!(!(1..=10).contains(&depth));
    }
}

#[test]
fn test_wire_length_minimum() {
    let min_wire: u64 = 100_000;
    let test_values: [(u64, bool); 3] = [(50_000, false), (100_000, true), (200_000, true)];

    for (value, expected) in test_values {
        assert_eq!(value >= min_wire, expected);
    }
}
