use crate::ui::utils;
use eframe::egui::{self, Color32};

#[derive(Default)]
pub struct Keyboard {
    pub first_row: Vec<egui::Key>,
    pub second_row: Vec<egui::Key>,
    pub third_row: Vec<egui::Key>,
}

impl Keyboard {
    pub fn default() -> Self {
        let converter = utils::KeyConversion::new();
        Self {
            first_row: converter.map_string_to_keys("qwertyuiop"),
            second_row: converter.map_string_to_keys("asdfghjkl"),
            third_row: converter.map_string_to_keys("zxcvbnm")
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        egui::Frame::none()
        .inner_margin(egui::style::Margin::symmetric(ui.available_width() / 3.0 , 0.0))
        .show(ui, |ui| {
            egui::Frame::none().inner_margin(egui::style::Margin::symmetric(10., 0.)).show(ui, |ui| {
                ui.spacing_mut().item_spacing = (10.0, 0.0).into();
                ui.horizontal(|ui| {
                    for key in &self.first_row {
                        egui::Frame::none().fill(Color32::BLACK).show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(format!("{:?}", key))
                                .size(20.)
                                .color(Color32::WHITE)
                            )
                        });
                    }
                });
            });
            egui::Frame::none().inner_margin(egui::style::Margin::symmetric(30., 0.)).show(ui, |ui| {
                ui.spacing_mut().item_spacing = (10.0, 0.0).into();
                ui.horizontal(|ui| {
                    for key in &self.second_row {
                        egui::Frame::none().fill(Color32::BLACK).show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(format!("{:?}", key))
                                .size(20.)
                                .color(Color32::WHITE)
                            )
                        });
                    }
                });
            });
            egui::Frame::none().inner_margin(egui::style::Margin::symmetric(40., 0.)).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = (10.0, 0.0).into();
                    for key in &self.third_row {
                        egui::Frame::none().fill(Color32::BLACK).show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(format!("{:?}", key))
                                .size(20.)
                                .color(Color32::WHITE)
                            )
                        });
                    }
                });
            });
        });
    }
}
