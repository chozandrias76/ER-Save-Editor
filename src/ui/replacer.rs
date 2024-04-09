pub mod replacer {
    use std::path::PathBuf;
  struct File {
    pub name: String,
    pub extensions: Vec<String>,
  }

  impl Default for File {
    fn default() -> Self {
        Self {
          name: String::from("JSON"),
          extensions: vec![String::from("json")]
        }
    }
  }
  pub struct Replacer {}

  impl Replacer {
    pub fn open_file_dialog() -> Option<PathBuf>{
      let file_defaults = File::default();
      rfd::FileDialog::new()
      .add_filter(file_defaults.name, &file_defaults.extensions)
      .set_directory("/")
      .pick_file()
    }
  }
}