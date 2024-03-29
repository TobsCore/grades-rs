use std::error::Error;
use std::fmt;
//let strict_grades: Vec<f32> = vec![1.0, 1.3, 1.7, 2.0, 2.3, 2.7, 3.0, 3.3, 3.7, 4.0, 4.3, 4.7, 5.0];

#[derive(Debug, Clone, Copy)]
pub struct Grade {
    val: usize,
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out_val = (self.val as f32) / 10.0;
        write!(f, "{:.1}", out_val)
    }
}

impl PartialEq for Grade {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Grade {
    pub fn from(input: &str) -> Result<Grade, Box<dyn Error>> {
        let parsed_val = input.replace(",", ".").parse::<f32>()?;
        let val = (parsed_val * 10.0) as usize;
        if val >= 10 && val <= 50 {
            Ok(Grade { val })
        } else {
            Err(format!("{} is out of bounds. Must be between 1.0 and 5.0", input).into())
        }
    }

    pub fn verbal(&self) -> &str {
        if self.val >= 10 && self.val < 16 {
            "very good"
        } else if self.val >= 16 && self.val < 25 {
            "good"
        } else if self.val >= 26 && self.val < 35 {
            "satisfactory"
        } else if self.val >= 36 && self.val < 41 {
            "sufficient"
        } else if self.val >= 41 && self.val <= 50 {
            "not sufficient"
        } else {
            unreachable!()
        }
    }
}

pub fn avg(grade_list: &[Grade]) -> Option<Grade> {
    if !grade_list.is_empty() {
        let acc: usize = grade_list.iter().map(|e| e.val).sum();
        let grade = (acc / grade_list.len()) as usize;
        Some(Grade { val: grade })
    } else {
        None
    }
}

pub fn avg_prec(grade_list: &[Grade]) -> Option<f64> {
    if !grade_list.is_empty() {
        let acc: usize = grade_list.iter().map(|grade| grade.val).sum();
        Some((acc as f64) / (10.0 * grade_list.len() as f64))
    } else {
        None
    }
}
