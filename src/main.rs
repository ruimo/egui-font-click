#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    pos: egui::Pos2,
}

impl MyApp {
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            pos: egui::Pos2::ZERO,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let shape = egui::Shape::text(
                &ui.fonts(),
                egui::Pos2::new(0., 0.),
                egui::Align2::LEFT_TOP,
                "A",
                egui::FontId::new(360., egui::FontFamily::Proportional),
                egui::Color32::BLACK
            );

            let surrounding_rect = shape.visual_bounding_rect();
            ui.painter().add(shape);

            for e in &ui.input().events {
                if let egui::Event::PointerMoved(pos) = *e {
                    self.pos = pos
                }
            }

            if surrounding_rect.contains(self.pos) {
                ui.output().cursor_icon = egui::CursorIcon::Grabbing;
            }
        });
    }
}
