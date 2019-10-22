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
fn test_avg_single() {
    let grade = new_grade("1.3");
    let grade_list = [grade];
    assert_eq!(grade, grades::avg(&grade_list));
}

#[test]
fn test_avg_none() {
    let expected = new_grade("0.0");
    assert_eq!(expected, grades::avg(&[]));
}

#[test]
fn test_avg_two() {
    let expected = new_grade("1.5");
    assert_eq!(expected, grades::avg(&[new_grade("1.0"), new_grade("2.0")]));
}

fn new_grade(input: &str) -> Grade {
    Grade::from(input).unwrap()
}
