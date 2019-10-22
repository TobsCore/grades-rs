use std::fmt;
use std::num::ParseFloatError;
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
    pub fn from(input: &str) -> Result<Grade, ParseFloatError> {
        let parsed_val = input.replace(",", ".").parse::<f32>()?;
        let val = (parsed_val * 10.0) as usize;
        Ok(Grade { val })
    }

    pub fn verbal(&self) -> &str {
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

pub fn avg(grade_list: &[Grade]) -> Grade {
    if !grade_list.is_empty() {
        let acc: usize = grade_list.iter().map(|e| e.val).sum();
        let grade = (acc / grade_list.len()) as usize;
        Grade { val: grade }
    } else {
        Grade { val: 0 }
    }
}

pub fn avg_prec(grade_list: &[Grade]) -> f64 {
    if !grade_list.is_empty() {
        let acc: usize = grade_list.iter().map(|grade| grade.val).sum();
        (acc as f64) / (10.0 * grade_list.len() as f64)
    } else {
        0.0
    }
}
