#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fmt::Display;

use eframe::egui;
use egui::{ImageSource, RichText};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Boat {
    SegelfahrzeugUnter7m,
    SegelfahrzeugBis20m,
    SegelfahrzeugUeber20m,
    MaschinenfahrzeugBis50m,
    MaschinenfahrzeugUeber50m,
    Lostenfahrzeug,
    NichtTrawlendeFischer, // Kein Schleppnetz
    TrawlendeFischer,      // Schleppnetz
    Manoevrierbehindert,
    Manoevrierunfaehig,
    Tiefgangbehindert,
    Ankerlieger,
    Grundsitzer,
    Schleppverband,
}

impl Display for Boat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Boat::SegelfahrzeugUnter7m => write!(f, "Segelfahrzeug unter 7m"),
            Boat::SegelfahrzeugBis20m => write!(f, "Segelfahrzeug bis 20m"),
            Boat::SegelfahrzeugUeber20m => write!(f, "Segelfahrzeug über 20m"),
            Boat::MaschinenfahrzeugBis50m => write!(f, "Maschinenfahrzeug bis 50m"),
            Boat::MaschinenfahrzeugUeber50m => write!(f, "Maschinenfahrzeug über 50m"),
            Boat::Lostenfahrzeug => write!(f, "Lostenfahrzeug"),
            Boat::NichtTrawlendeFischer => write!(f, "Nicht trawlende Fischer"),
            Boat::TrawlendeFischer => write!(f, "Trawlende Fischer"),
            Boat::Manoevrierbehindert => write!(f, "Manövrierbehindert"),
            Boat::Manoevrierunfaehig => write!(f, "Manövrierunfähig"),
            Boat::Tiefgangbehindert => write!(f, "Tiefgangbehindert"),
            Boat::Ankerlieger => write!(f, "Ankerlieger"),
            Boat::Grundsitzer => write!(f, "Grundsitzer"),
            Boat::Schleppverband => write!(f, "Schleppverband"),
        }
    }
}

const BOATS: [Boat; 14] = [
    Boat::SegelfahrzeugUnter7m,
    Boat::SegelfahrzeugBis20m,
    Boat::SegelfahrzeugUeber20m,
    Boat::MaschinenfahrzeugBis50m,
    Boat::MaschinenfahrzeugUeber50m,
    Boat::Lostenfahrzeug,
    Boat::NichtTrawlendeFischer,
    Boat::TrawlendeFischer,
    Boat::Manoevrierbehindert,
    Boat::Manoevrierunfaehig,
    Boat::Tiefgangbehindert,
    Boat::Ankerlieger,
    Boat::Grundsitzer,
    Boat::Schleppverband,
];

const BOAT_IMAGES: [(Boat, egui::ImageSource<'static>); 14] = [
    (
        Boat::SegelfahrzeugUnter7m,
        egui::include_image!("../images/segelfahrzeug-unter-7m.png"),
    ),
    (
        Boat::SegelfahrzeugBis20m,
        egui::include_image!("../images/segelfahrzeuge-bis-20m.png"),
    ),
    (
        Boat::SegelfahrzeugUeber20m,
        egui::include_image!("../images/segelfahrzeuge-ueber-20m.png"),
    ),
    (
        Boat::MaschinenfahrzeugBis50m,
        egui::include_image!("../images/maschinenfahrzeuge-bis-50m.png"),
    ),
    (
        Boat::MaschinenfahrzeugUeber50m,
        egui::include_image!("../images/maschinenfahrzeuge-ueber-50m.png"),
    ),
    (
        Boat::Lostenfahrzeug,
        egui::include_image!("../images/lotsenfahrzeuge.png"),
    ),
    (
        Boat::NichtTrawlendeFischer,
        egui::include_image!("../images/nichttrawlende-fischer-kein-schleppnetz.png"),
    ),
    (
        Boat::TrawlendeFischer,
        egui::include_image!("../images/trawlende-fischer-schleppnetz.png"),
    ),
    (
        Boat::Manoevrierbehindert,
        egui::include_image!("../images/manoevrierbehindertes-fahrzeug.png"),
    ),
    (
        Boat::Manoevrierunfaehig,
        egui::include_image!("../images/manoevrierunfaehiges-fahrzeug.png"),
    ),
    (
        Boat::Tiefgangbehindert,
        egui::include_image!("../images/tiefgangbehindertes-fahrzeug.png"),
    ),
    (
        Boat::Ankerlieger,
        egui::include_image!("../images/ankerlieger.png"),
    ),
    (
        Boat::Grundsitzer,
        egui::include_image!("../images/grundsitzer.png"),
    ),
    (
        Boat::Schleppverband,
        egui::include_image!("../images/schleppverband.png"),
    ),
];

struct App {
    playing: bool,
    boat_images: Vec<(Boat, ImageSource<'static>)>,
    boat_images_index: usize,
    selected_boat: Boat,
}

impl Default for App {
    fn default() -> Self {
        Self {
            playing: true,
            boat_images: randomize_boat_images(),
            boat_images_index: 0,
            selected_boat: Boat::Grundsitzer,
        }
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Boat Light Trainer",
        options,
        Box::new(|cc| {
            // Image Support
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<App>::default())
        }),
    )
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Beenden").clicked() {
                std::process::exit(0);
            }

            if self.boat_images_index == self.boat_images.len() {
                self.playing = false;
                ui.heading(RichText::new("Alle Boote durchgespielt!").color(egui::Color32::GREEN));

                if ui.button("Nochmal spielen").clicked() {
                    self.boat_images_index = 0;
                    self.boat_images = randomize_boat_images();
                    self.playing = true;
                }

                if !self.playing {
                    return;
                }
            }

            let (boat, image) = &self.boat_images[self.boat_images_index];
            let is_correct = which_boat_has_these_lights(ui, self, *boat, image.clone());

            if ui.button("Nächste Frage").clicked() && is_correct {
                self.boat_images_index += 1;
            }
        });
    }
}

fn which_boat_has_these_lights(
    ui: &mut egui::Ui,
    app: &mut App,
    real_boat: Boat,
    image: egui::ImageSource<'static>,
) -> bool {
    ui.heading("Welche Boote haben diese Lichter?");

    ui.horizontal(|ui| {
        ui.label("Boot: ");
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", app.selected_boat))
            .show_ui(ui, |ui| {
                for boat in BOATS.iter() {
                    ui.selectable_value(&mut app.selected_boat, *boat, boat.to_string());
                }
            });
    });

    ui.add(egui::Image::new(image).max_width(1000.0));

    if real_boat == app.selected_boat {
        ui.heading(RichText::new("Richtig!").color(egui::Color32::GREEN));
        true
    } else {
        ui.heading(RichText::new("Falsch!").color(egui::Color32::RED));
        false
    }
}

fn randomize_boat_images() -> Vec<(Boat, ImageSource<'static>)> {
    let mut vec = BOAT_IMAGES.to_vec();
    vec.shuffle(&mut thread_rng());
    vec
}
