use egui::{Ui, text_edit, Window};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CalculatorApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    number1: String,
    number2: String,

    show_test_window: bool,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            show_test_window: true,
            number1: "0.0".to_string(),
            number2: "0.0".to_string(),
        }
    }
}

impl CalculatorApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for CalculatorApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, value , show_test_window, number1, number2} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });
        
        
        // // let mut open = true;
        // egui::Window::new("My Window 2")
        // // .open(&mut true)
        // // .open(&mut true)
        // .show(ctx, |ui| {
        //     egui::Window::open(self, &`open mut true);
        // });

        
        
        // egui::Window::new("My Window")
        //             .enabled(enabled)
        //             .show(ctx, |ui| {
        //     ui.label("Hello World!");

        //  });

        // egui::SidePanel::left("side_panel").show(ctx, |ui| {
        //     ui.heading("Side Panel");

        //     ui.horizontal(|ui| {
        //         ui.label("Write something: ");
        //         ui.text_edit_singleline(label);
        //     });

        //     ui.add(egui::Slider::new(value, 0.0..=100.0).text("value"));
        //     if ui.button("Increment").clicked() {
        //         *value += 5.0;
        //     }

        //     ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //         ui.horizontal(|ui| {
        //             ui.spacing_mut().item_spacing.x = 0.0;
        //             ui.label("powered by ");
        //             ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        //             ui.label(" and ");
        //             ui.hyperlink_to(
        //                 "eframe",
        //                 "https://github.com/emilk/egui/tree/master/crates/eframe",
        //             );
        //             ui.label(".");
        //         });
        //     });
        // });

        // egui::SidePanel::right("side_panel").show(ctx, |ui| {
        //     ui.heading("Side Panel");

        // });
        // egui::SidePanel::right("my_right_panel").show(ctx,|ui: &mut Ui| {

        //     ui.heading("Right Side GG");
        //     ui.horizontal(|ui| {
        //         ui.label("Write something: ");
        //         ui.text_edit_singleline(label);
        //     });

        //     if ui.button("Increment").clicked() {
        //         ui.text_edit_singleline(label);
        //     };
        // });
        
        egui::Area::new("1 Button")
            .fixed_pos(egui::pos2(320.0, 320.0))
            .show(ctx, |ui| {
                
                ui.text_edit_singleline(&mut *number1);
                ui.text_edit_singleline(&mut *number2);
                // if ui.button("1").clicked() {
                //     let mut text_edited: String = "hello".to_string();
                //     ui.text_edit_singleline(&mut text_edited);
                // };
                if ui.button("Add").clicked() {
                    let temp1: f32 = number1.parse().unwrap();
                    let temp2: f32 = number2.parse().unwrap();
                    *value = temp1 + temp2;
                    
                }

                ui.label(value.to_string());
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("CALCULATOR");
            // ui.hyperlink("https://github.com/emilk/eframe_template");
            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/master/",
            //     "Source code."
            // ));
            ui.button("1");

            
            
            egui::warn_if_debug_build(ui);

            if ui.button("Open Window").clicked() {
                *show_test_window = true;
            }

        });

        if true {
            egui::Window::new("Window")
            .open(&mut *show_test_window)
            .show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
                
                //self.show_test_window = true;

                // if egui::Window::open(self, open) {

                // }
            });
        }
    }
}
