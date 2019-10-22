use grades::Grade;

#[test]
fn test_parse_working() {
    if let Ok(grade) = Grade::from("1.3") {
        assert_eq!("1.3", format!("{}", grade));
    }
}

#[test]
fn test_parse_comma() {
    if let Ok(grade) = Grade::from("1,3") {
        assert_eq!("1.3", format!("{}", grade));
    }
}
