use std::env;
use std::fmt;
use std::num::ParseFloatError;

//let strict_grades: Vec<f32> = vec![1.0, 1.3, 1.7, 2.0, 2.3, 2.7, 3.0, 3.3, 3.7, 4.0, 4.3, 4.7, 5.0];

#[derive(Debug)]
struct Grade {
    val: usize,
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out_val = (self.val as f32) / 10.0;
        write!(f, "{:.1}", out_val)
    }
}

impl Grade {
    fn from(input: &str) -> Result<Grade, ParseFloatError> {
        let parsed_val = input.parse::<f32>()?;
        let val = (parsed_val * 10.0) as usize;
        Ok(Grade { val })
    }

    fn verbal(&self) -> &str {
        if self.val >= 10 && self.val < 16 {
            "sehr gut"
        } else if self.val >= 16 && self.val < 25 {
            "gut"
        } else if self.val >= 26 && self.val < 35 {
            "befriedigent"
        } else if self.val >= 36 && self.val < 41 {
            "ausreichend"
        } else if self.val >= 41 && self.val <= 50 {
            "nicht ausreichend"
        } else {
            "keine Note"
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut grades = Vec::new();
    for arg in &args[1..] {
        if let Ok(grade) = Grade::from(&arg) {
            grades.push(grade);
        } else {
            eprintln!("Error: Could not parse '{}'", arg);
        }
    }

    let avg_grade = avg(grades);
    println!("Average: {} ({})", avg_grade, avg_grade.verbal());
}

fn avg(grade_list: Vec<Grade>) -> Grade {
    if grade_list.len() > 0 {
        let acc = grade_list.iter().fold(0, |sum, i| sum + i.val);
        let grade = (acc / grade_list.len()) as usize;
        Grade { val: grade }
    } else {
        Grade { val: 0 }
    }
}
