use egui::{
    menu::{self},
    CentralPanel, Pos2, Window, SidePanel,
};

const EDITOR_NAME: &'static str = "CEdtior";

#[derive(Default)]
struct CEditor {
    allowed_to_close: bool,
    show_close_dialog: bool,

    test: String,
}

impl eframe::App for CEditor {
    fn on_close_event(&mut self) -> bool {
        self.show_close_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        
        SidePanel::right("right_panel")
        .resizable(false)
        .show(ctx, |ui|{
            ui.vertical(|ui|{
                ui.heading("Tools");
            })
        });


        CentralPanel::default().show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New File").clicked() {}
                    if ui.button("Open").clicked() {}
                    if ui.button("Save").clicked() {}
                    if ui.button("Save as").clicked() {}
                });
                ui.menu_button("Edit", |ui| if ui.button("New File").clicked() {});
                ui.menu_button("Select", |ui| if ui.button("New File").clicked() {});
                ui.menu_button("Help", |ui| if ui.button("New File").clicked() {});
            });

            ui.heading(EDITOR_NAME);

            if self.show_close_dialog {
                Window::new("Do you confirm to quit?")
                    .constrain(false)
                    .fixed_pos(Pos2::new(50.0, 50.0))
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Cancel").clicked() {
                                self.show_close_dialog = false;
                            }

                            if ui.button("Yes!").clicked() {
                                self.allowed_to_close = true;
                                frame.close();
                            }
                        })
                    });
            }
        });
    }
}

fn main() {
    let app = CEditor::default();
    let window_options = eframe::NativeOptions::default();
    eframe::run_native("CEditor", window_options, Box::new(|_cc| Box::new(app))).expect("Failed");
}
