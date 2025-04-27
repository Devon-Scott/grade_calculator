pub struct RowFields {
    pub section: String,
    pub marks: String,
    pub weight: String,
    pub section_grade: String,
}

impl RowFields {
    pub fn new() -> Self {
        Self {
            section: String::new(),
            marks: String::new(),
            weight: String::new(),
            section_grade: ("".to_string()),
        }
    }

    pub fn new_named(section_name: String) -> Self {
        Self {
            section: section_name,
            marks: String::new(),
            weight: String::new(),
            section_grade: ("%".to_string()),
        }
    }
}