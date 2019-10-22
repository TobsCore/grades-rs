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
    } else {
        panic!("Failed to parse")
    }
}

#[test]
fn test_parse_out_of_bounds() {
    match Grade::from("6.0") {
        Ok(_) => assert!(false, "Should be out of bounds"),
        Err(_) => assert!(true),
    }
}
#[test]
fn test_avg_single() {
    let grade = new_grade("1.3");
    let grade_list = [grade];
    assert_eq!(Some(grade), grades::avg(&grade_list));
}

#[test]
fn test_avg_none() {
    assert_eq!(None, grades::avg(&[]));
}

#[test]
fn test_avg_two() {
    let expected = new_grade("1.5");
    assert_eq!(
        Some(expected),
        grades::avg(&[new_grade("1.0"), new_grade("2.0")])
    );
}

#[test]
fn test_print_correct_grade() {
    assert_eq!("sehr gut", new_grade("1.0").verbal());
}

fn new_grade(input: &str) -> Grade {
    Grade::from(input).unwrap()
}
