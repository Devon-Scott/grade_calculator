mod ui_elements;
use crate::ui_elements::RowFields;

struct TotalMarks {
    pub total_marks: f32,
    pub total_weight: f32,
}

impl TotalMarks {
    pub fn new(rows: &vec<RowFields>) -> Self {
        Self {
            total_marks: 0.0,
            total_weight: 0.0,
        }
    }

    pub fn add(&mut self, marks: f32, weight: f32) {
        self.total_marks += marks * weight;
        self.total_weight += weight;
    }

    pub fn calculate_final_grade(&self) -> Option<f32> {
        if self.total_weight > 0.0 {
            Some(self.total_marks / self.total_weight)
        } else {
            None
        }
    }
}