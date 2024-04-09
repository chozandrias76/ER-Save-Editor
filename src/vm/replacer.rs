pub mod general_view_model {
    use crate::{
        save::save::save::Save, vm::{importer::general_view_model::Character, vm::vm::ViewModel}
    };

    pub struct ReplacerViewModel {
        pub valid: bool,
        pub selected_from_index: usize,
        pub selected_to_index: usize,
        pub from_list: [Character; 0x10],
        pub to_list: [Character; 0x10],
        from_save: Save,
    }

    impl Default for ReplacerViewModel {
        fn default() -> Self {
            Self {
                valid: false,
                selected_from_index: 0,
                selected_to_index: 0,
                from_list: Default::default(),
                to_list: Default::default(),
                from_save: Save::default(),
            }
        }
    }

    impl ReplacerViewModel {
        pub fn new(from: Save, vm: &ViewModel) -> Self {
            let mut replacer_view_model = ReplacerViewModel::default();

            replacer_view_model.from_save = from;

            for (i, active) in replacer_view_model
                .from_save
                .save_type
                .active_slots()
                .iter()
                .enumerate()
            {
                if *active {
                    // Character Name
                    let character_name = replacer_view_model
                        .from_save
                        .save_type
                        .get_slot(i)
                        .player_game_data
                        .character_name;
                    let mut character_name_trimmed: [u16; 0x10] = [0; 0x10];
                    for (i, char) in character_name.iter().enumerate() {
                        if *char == 0 {
                            break;
                        }
                        character_name_trimmed[i] = *char;
                    }
                    let character_name: String =
                        String::from_utf16(&character_name_trimmed).expect("");
                    replacer_view_model.from_list[i].active = true;
                    replacer_view_model.from_list[i].index = i;
                    replacer_view_model.from_list[i].name = character_name;
                }
            }

            for i in 0..0xA {
                if vm.profile_summary[i].active {
                    replacer_view_model.to_list[i].active = true;
                    replacer_view_model.to_list[i].index = i;
                    replacer_view_model.to_list[i].name =
                        vm.slots[i].general_vm.character_name.to_string();
                }
            }

            replacer_view_model
        }
    }
}
