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
        ui.add(egui::TextEdit::singleline(&mut self.section).hint_text("Section name"));
        let resp = 
            ui.add(egui::TextEdit::singleline(&mut self.marks).hint_text("25/50 + 10/20 + 76/80"));
        ui.add(egui::TextEdit::singleline(&mut self.weight).hint_text("Weight % (e.g., 20)"));
        ui.label(&self.section_grade);

        resp.changed()
    }   
}

pub struct RowHeaders {
    pub delete: String,
    pub section: String,
    pub marks: String,
    pub weight: String,
    pub section_grade: String,
}

impl RowHeaders {
    pub fn new() -> Self {
        Self {
            delete: "Delete".to_string(),
            section: "Section".to_string(),
            marks: "Marks/Total".to_string(),
            weight: "Weight %".to_string(),
            section_grade: "Section Grade %".to_string(),
        }
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        ui.label(&self.delete);
        ui.label(&self.section).on_hover_text_at_pointer("Section name");
        ui.label(&self.marks).on_hover_text_at_pointer("Enter a mathematical expression for the marks as a fraction of the total");
        ui.label(&self.weight);
        ui.label(&self.section_grade);
    }
}