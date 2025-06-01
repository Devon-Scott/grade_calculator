pub struct RowFields {
    pub section: String,
    pub marks: String,
    pub weight: String,
    pub section_grade: String,
    pub f_section_grade: Option<f32>,
}

impl RowFields {
    pub fn new() -> Self {
        Self {
            section: String::new(),
            marks: String::new(),
            weight: String::new(),
            section_grade: ("%".to_string()),
            f_section_grade: None
        }
    }

    pub fn new_named(section_name: String) -> Self {
        Self {
            section: section_name,
            marks: String::new(),
            weight: String::new(),
            section_grade: ("%".to_string()),
            f_section_grade: None
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) -> bool {
        ui.add(egui::TextEdit::singleline(&mut self.section));
        let resp = 
            ui.add(egui::TextEdit::singleline(&mut self.marks));
        ui.add(egui::TextEdit::singleline(&mut self.weight));

        resp.changed()
    }   
}

pub struct RowHeaders {
    pub delete: String,
    pub section: String,
    pub marks: String,
    pub weight: String,
}

impl RowHeaders {
    pub fn new() -> Self {
        Self {
            delete: "Delete".to_string(),
            section: "Section".to_string(),
            marks: "Marks/Total".to_string(),
            weight: "Weight %".to_string(),
        }
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        ui.label(&self.delete);
        ui.label(&self.section).on_hover_text_at_pointer("Section name");
        ui.label(&self.marks).on_hover_text_at_pointer("e.g. (25 + 10 + 76) / (50 + 20 + 80)\nPoints earned / Total points possible");
        ui.label(&self.weight).on_hover_text_at_pointer("Weight of this section in the overall grade\nEnter a number between 0 and 100");
    }
}