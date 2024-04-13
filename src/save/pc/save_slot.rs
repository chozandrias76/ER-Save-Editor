use crate::{read::read::Read, save::common::save_slot::SaveSlot, write::write::Write};

#[derive(Clone)]
pub struct PCSaveSlot {
    pub checksum: [u8; 0x10],
    pub save_slot: SaveSlot,
}

impl Default for PCSaveSlot {
    fn default() -> Self {
        Self {
            checksum: [0x0; 0x10],
            save_slot: SaveSlot::default(),
        }
    }
}

impl Read for PCSaveSlot {
    fn read(br: &mut binary_reader::BinaryReader) -> Result<Self, std::io::Error> {
        let mut pc_save_slot = PCSaveSlot::default();

        // Checksum
        pc_save_slot.checksum.copy_from_slice(br.read_bytes(0x10)?);

        // Rest of the save slot
        pc_save_slot.save_slot = SaveSlot::read(br)?;

        Ok(pc_save_slot)
    }
}

impl Write for PCSaveSlot {
    fn write(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes: Vec<u8> = Vec::new();

        // Get save slot bytes
        let save_slot_bytes = self.save_slot.write()?;

        // Calculate checksum
        let digest = md5::compute(&save_slot_bytes);

        // Add Checksum
        bytes.extend(digest.iter().collect::<Vec<&u8>>());

        // Add the rest of the save slot
        bytes.extend(save_slot_bytes);

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use binary_reader::BinaryReader;

    use super::*;
    use std::fs;


    #[test]
    fn test_read_pc_save_slot() {
        // Create a mock binary file checksum
        let checksum_data = [156, 166, 241, 135, 131, 219, 101, 191, 157, 118, 71, 159, 124, 220, 37, 68]; // 16 bytes for checksum
        let br = &mut BinaryReader::from_u8(&fs::read("./fixtures/vagabond.pc_slot").expect("Test file should be present in fixtures"));

        // Attempt to read the PCSaveSlot
        let result = PCSaveSlot::read(br);

        assert!(result.is_ok());
        let pc_save_slot = result.unwrap();

        // Check if the checksum is read correctly
        assert_eq!(pc_save_slot.checksum, checksum_data, "Checksum does not match");

        // SaveSlot is mocked so default should be present
        assert_eq!(pc_save_slot.save_slot.ver, SaveSlot::default().ver);
    }
}