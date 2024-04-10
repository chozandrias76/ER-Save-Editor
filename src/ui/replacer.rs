pub mod replacer {
    use std::path::PathBuf;

    use eframe::egui;
    struct File {
        pub name: String,
        pub extensions: Vec<String>,
    }

    impl Default for File {
        fn default() -> Self {
            Self {
                name: String::from("JSON"),
                extensions: vec![String::from("json")],
            }
        }
    }
    pub struct Replacer {}

    impl Replacer {
        pub fn open_file_dialog() -> Option<PathBuf> {
            let file_defaults = File::default();
            rfd::FileDialog::new()
                .add_filter(file_defaults.name, &file_defaults.extensions)
                .set_directory("/")
                .pick_file()
        }
    }

    #[cfg(debug_assertions)]
    pub fn layout(ui: &mut eframe::egui::Ui, app: &mut crate::App) {
        use crate::save::nya::nya_save::NyaSave;

        let import_button = egui::widgets::Button::new(egui::RichText::new(format!(
            "{} Replace Character using JSON",
            egui_phosphor::regular::ARROWS_MERGE
        )));

        if ui
            .add_enabled(!app.vm.steam_id.is_empty(), import_button)
            .clicked()
        {
            let files = crate::ui::replacer::replacer::Replacer::open_file_dialog();
            match files {
                Some(path) => match NyaSave::from_path(&path) {
                    Ok(nya_save) => {
                        app.replacer_vm =
                            crate::vm::replacer::general_view_model::ReplacerViewModel::new(nya_save, &app.vm);
                        app.replacer_open = true;
                    }
                    Err(_) => {}
                },
                None => {}
            }
        }
        crate::ui::replacer::replacer::character_replacer(
            ui,
            &mut app.replacer_open,
            &mut app.replacer_vm,
            &mut app.save,
            &mut app.vm,
        )
    }

    pub fn character_replacer(
        ui: &mut eframe::egui::Ui,
        open: &mut bool,
        replacer_vm: &mut crate::vm::replacer::general_view_model::ReplacerViewModel,
        to_save: &mut crate::save::save::save::Save,
        vm: &mut crate::vm::vm::vm::ViewModel,
    ) {
        eframe::egui::Window::new("Replacer")
            .open(open)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                ui.columns(2, |uis| {
                    uis[0].vertical_centered_justified(|ui| {
                        ui.heading("From");
                        ui.separator();
                        for (i, from_character) in replacer_vm
                            .from_list
                            .iter()
                            .filter(|c| c.active)
                            .enumerate()
                        {
                            if ui
                                .selectable_label(
                                    replacer_vm.selected_from_index == i,
                                    &from_character.name,
                                )
                                .clicked()
                            {
                                replacer_vm.selected_from_index = i
                            }
                        }
                    });
                    uis[1].vertical_centered_justified(|ui| {
                        ui.heading("To");
                        ui.separator();
                        for (i, to_character) in
                            replacer_vm.to_list.iter().filter(|c| c.active).enumerate()
                        {
                            if ui
                                .selectable_label(
                                    replacer_vm.selected_to_index == i,
                                    &to_character.name,
                                )
                                .clicked()
                            {
                                replacer_vm.selected_to_index = i
                            }
                        }
                    });
                });
                ui.add_space(5.);
                ui.vertical_centered_justified(|ui| {
                    if ui
                        .add_sized(
                            [ui.available_width(), 40.],
                            egui::widgets::Button::new("Replace"),
                        )
                        .clicked()
                    {
                        println!("clicked!");
                        replacer_vm.import_character(to_save, vm);
                    }
                });
            });
    }
}
