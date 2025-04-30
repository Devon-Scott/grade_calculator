use eframe::egui;

mod ui_elements;
mod calculator;
use crate::ui_elements::RowFields;
use crate::calculator::Calculator;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Grade Weight Calculator",
        options,
        Box::new(|_cc| {
            // This gives us image support:
            Ok(Box::<MyApp>::new(MyApp::new())) 
        }),
    )
}

struct MyApp {
    rows: Vec<RowFields>,
    _course_name: String,
    _final_grade: Option<f32>,
    
}

impl MyApp {
    fn new() -> Self {
        let names = ["Assignments", "Project", "Midterm", "Final"];
        let mut rows = Vec::new();
        for name in names.iter() {
            rows.push(RowFields::new_named(name.to_string()));
        }
        Self {
            rows,
            _course_name: "".to_string(),
            _final_grade: None,
        }
    }

    fn render_header(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("header")
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Course Name");
                    ui.add_space(8.0);
                    ui.add_sized([ui.available_width() / 3.0, 20.0], egui::TextEdit::singleline(&mut self._course_name));
                    ui.add_space(8.0);
                });
            });
        }

    fn render_grid(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // compute column widths
            let total_width = ui.available_width();
            let spacing_x  = 8.0;
            let delete_w   = 24.0;              
            // subtract out the 4 gaps between columns:
            let total_gaps = spacing_x * 3.0;   
            let field_w    = (total_width - delete_w - total_gaps) / 3.0;

            egui::Grid::new("grade_grid")
                .spacing([spacing_x, 4.0])
                .min_col_width(delete_w)
                .max_col_width(field_w)
                .show(ui, |ui| {
                    // ——— Headers ———
                    ui.label("Delete");           // over the delete‐button
                    ui.label("Section").on_hover_text_at_pointer("Section name");
                    ui.label("Marks/Total").on_hover_text_at_pointer("Enter a mathematical expression for the marks as a fraction of the total");
                    ui.label("Section Grade");
                    ui.label("Weight");
                    ui.end_row();

                    // ——— Rows ———
                    let mut remove_idx = None;
                    for (i, row) in self.rows.iter_mut().enumerate() {
                        // delete button (fixed width)
                        if ui
                            .add_sized([delete_w, 20.0], egui::Button::new("❌"))
                            .clicked()
                        {
                            remove_idx = Some(i);
                        }
                        // text inputs (equal width)
                        ui.add_sized([field_w, 20.0], egui::TextEdit::singleline(&mut row.section));
                        ui.add_sized([field_w, 20.0], egui::TextEdit::singleline(&mut row.marks));

                        row.f_section_grade = Calculator::calculate_str_2_f(&row.marks);

                        ui.label(&row.section_grade);
                        ui.add_sized([field_w, 20.0], egui::TextEdit::singleline(&mut row.weight));

                        ui.end_row();
                    }
                    if let Some(i) = remove_idx {
                        self.rows.remove(i);
                    }
                }
            );
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        self.render_header(ctx);
        self.render_grid(ctx);
        
        egui::TopBottomPanel::bottom("add_row_panel")
            .min_height(40.0)
            .show(ctx, |ui| {
                // force left alignment
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    if ui.button("Add row").clicked() {
                        self.rows.push(RowFields::new());
                    }
                    if ui.button("Calculate").clicked() {
                        let mut grades = Vec::new();
                        let mut weights = Vec::new();
                        // TODO enforce matching indices
                        for row in self.rows.iter() {
                            if let Some(grade) = row.f_section_grade {
                                grades.push(grade);
                            }
                            if let Ok(weight) = row.weight.parse::<f32>() {
                                weights.push(weight);
                            }
                        }
                        self._final_grade = Calculator::weighted_sum(&grades, &weights);
                    }
                });
            });

        
    }
}