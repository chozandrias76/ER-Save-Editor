pub mod general_view_model {
    use crate::{
        save::{nya::nya_save::NyaSave, save::save::Save},
        vm::{importer::general_view_model::Character, vm::vm::ViewModel},
    };

    pub struct ReplacerViewModel {
        pub valid: bool,
        pub selected_from_index: usize,
        pub selected_to_index: usize,
        pub from_list: [Character; 0x10],
        pub to_list: [Character; 0x10],
        from_save: NyaSave,
    }

    impl Default for ReplacerViewModel {
        fn default() -> Self {
            Self {
                valid: false,
                selected_from_index: 0,
                selected_to_index: 0,
                from_list: Default::default(),
                to_list: Default::default(),
                from_save: NyaSave::default(),
            }
        }
    }

    impl ReplacerViewModel {
        pub fn new(nya_save: NyaSave, vm: &ViewModel) -> Self {
            let mut replacer_view_model = ReplacerViewModel::default();
            replacer_view_model.valid = true;

            replacer_view_model.from_save = nya_save;

            let character_name: String = replacer_view_model.from_save.name.clone();
            replacer_view_model.from_list[0].active = true;
            replacer_view_model.from_list[0].index = 0;
            replacer_view_model.from_list[0].name = character_name;

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

        pub fn import_character(&mut self, to_save: &mut Save, vm: &mut ViewModel) {
            // Retain slot version
            // let mut from_slot = self
            //     .from_save;
            // let to_slot = to_save.save_type.get_slot(self.selected_to_index);
            // from_slot.ver = to_slot.ver;

            // // Save Slot
            // to_save
            //     .save_type
            //     .set_slot(self.selected_to_index, &from_slot);

            // // Profile Summary
            // to_save.save_type.set_profile_summary(
            //     self.selected_to_index,
            //     self.from_save
            //         .save_type
            //         .get_profile_summary(self.selected_from_index),
            // );

            // Refresh view model
            vm.slots[self.selected_to_index] =
                crate::vm::slot::slot_view_model::SlotViewModel::from_save(
                    to_save.save_type.get_slot(self.selected_to_index),
                );
        }
    }
}
