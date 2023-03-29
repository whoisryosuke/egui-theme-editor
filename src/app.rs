#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum DropdownValues {
    First,
    Second,
    Third,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    boolean: bool,

    radio: DropdownValues,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    theme: egui::Visuals,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            boolean: false,
            radio: DropdownValues::First,
            value: 2.7,
            theme: egui::Visuals::dark(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            boolean,
            radio,
            value,
            theme,
        } = self;

        ctx.set_visuals(theme.clone());

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

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Theme Editor");

            ui.label("Hyperlink color:");
            ui.color_edit_button_srgba(&mut theme.hyperlink_color);
            ui.label("faint_bg_color:");
            ui.color_edit_button_srgba(&mut theme.faint_bg_color);
            ui.label("extreme_bg_color:");
            ui.color_edit_button_srgba(&mut theme.extreme_bg_color);
            ui.label("code_bg_color:");
            ui.color_edit_button_srgba(&mut theme.code_bg_color);
            ui.label("warn_fg_color:");
            ui.color_edit_button_srgba(&mut theme.warn_fg_color);
            ui.label("error_fg_color:");
            ui.color_edit_button_srgba(&mut theme.error_fg_color);
            ui.label("window_fill:");
            ui.color_edit_button_srgba(&mut theme.window_fill);
            ui.label("panel_fill:");
            ui.color_edit_button_srgba(&mut theme.panel_fill);
            ui.label("window_stroke.color:");
            ui.color_edit_button_srgba(&mut theme.window_stroke.color);

            ui.strong("Selection");
            ui.label("selection.bg_fill:");
            ui.color_edit_button_srgba(&mut theme.selection.bg_fill);
            ui.label("selection.stroke.color:");
            ui.color_edit_button_srgba(&mut theme.selection.stroke.color);

            ui.strong("Shadows");
            ui.label("window_shadow.color:");
            ui.color_edit_button_srgba(&mut theme.window_shadow.color);
            ui.label("popup_shadow.color:");
            ui.color_edit_button_srgba(&mut theme.popup_shadow.color);

            ui.heading("Widgets");
            ui.strong("inactive");
            ui.color_edit_button_srgba(&mut theme.widgets.inactive.bg_fill);
            ui.color_edit_button_srgba(&mut theme.widgets.inactive.weak_bg_fill);
            ui.color_edit_button_srgba(&mut theme.widgets.inactive.bg_stroke.color);
            ui.strong("hovered");
            ui.color_edit_button_srgba(&mut theme.widgets.hovered.bg_fill);
            ui.color_edit_button_srgba(&mut theme.widgets.hovered.weak_bg_fill);
            ui.color_edit_button_srgba(&mut theme.widgets.hovered.bg_stroke.color);
            ui.strong("active");
            ui.color_edit_button_srgba(&mut theme.widgets.active.bg_fill);
            ui.color_edit_button_srgba(&mut theme.widgets.active.weak_bg_fill);
            ui.color_edit_button_srgba(&mut theme.widgets.active.bg_stroke.color);
            ui.strong("noninteractive");
            ui.color_edit_button_srgba(&mut theme.widgets.noninteractive.bg_fill);
            ui.color_edit_button_srgba(&mut theme.widgets.noninteractive.weak_bg_fill);
            ui.color_edit_button_srgba(&mut theme.widgets.noninteractive.bg_stroke.color);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("egui theme editor playground");
            ui.label("preview your theme's changes here with most components visible");
            ui.hyperlink("https://github.com/whoisryosuke/egui-theme-editor");
            ui.button("Click me!");
            ui.checkbox(boolean, "Checkbox");
            ui.horizontal(|ui| {
                ui.radio_value(radio, DropdownValues::First, "First");
                ui.radio_value(radio, DropdownValues::Second, "Second");
                ui.radio_value(radio, DropdownValues::Third, "Third");
            });
            ui.horizontal(|ui| {
                ui.selectable_value(radio, DropdownValues::First, "First");
                ui.selectable_value(radio, DropdownValues::Second, "Second");
                ui.selectable_value(radio, DropdownValues::Third, "Third");
            });
            egui::ComboBox::from_label("Take your pick")
                .selected_text(format!("{:?}", radio))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    ui.selectable_value(radio, DropdownValues::First, "First");
                    ui.selectable_value(radio, DropdownValues::Second, "Second");
                    ui.selectable_value(radio, DropdownValues::Third, "Third");
                });
            ui.add(egui::Slider::new(value, 0.0..=360.0).suffix("°"));
            ui.add(egui::DragValue::new(value).speed(1.0));
            ui.add(egui::TextEdit::singleline(label).hint_text("Write something here"));
            ui.code("ui.add(egui::DragValue::new(value).speed(1.0));");
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
