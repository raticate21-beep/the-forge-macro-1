use eframe::egui;

pub fn font_set(font_data: &'static [u8]) -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Montserrat".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(font_data)),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "Montserrat".to_owned());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "Montserrat".to_owned());

    fonts
}
pub fn load_icon() -> eframe::egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(include_bytes!("../assets/icon.png"))
            .expect("Not found icon.png in assets folder!")
            .into_rgba8();
        let (width, height) = image.dimensions();

        (image.into_raw(), width, height)
    };
    eframe::egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
