use std::{
    env::current_dir,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};

use pdfium_render::prelude::Pdfium;

/// make sure we can lock the pdfium instance across threads.
pub struct BindingsWrapper(Mutex<Pdfium>);

impl BindingsWrapper {
    pub fn with_pdfium<R>(&self, f: impl FnOnce(&Pdfium) -> R) -> R {
        let guard = self
            .0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        f(&guard)
    }
}

static PDFIUM_INSTANCE: OnceLock<Arc<BindingsWrapper>> = OnceLock::new();

pub fn init_pdfium() -> Arc<BindingsWrapper> {
    PDFIUM_INSTANCE
        .get_or_init(|| {
            let file_name = Pdfium::pdfium_platform_library_name();

            let lib_dir = Path::new("./lib");
            let current_dir = current_dir().unwrap_or(PathBuf::from("./"));

            let lib_path = match current_dir.join(&file_name).exists() {
                true => current_dir.join(&file_name),
                false => match lib_dir.join(&file_name).exists() {
                    true => lib_dir.join(&file_name),
                    false => panic!(
                        r#"Failed to bind to Pdfium Library! File "{}" not found in either working directory or ./lib"#,
                        &file_name.to_string_lossy().to_owned()
                    ),
                },
            };

            let bindings = match Pdfium::bind_to_library(&lib_path) {
                Ok(b) => b,
                Err(error) => panic!("Failed to load Pdfium library! {}", error),
            };

            Arc::new(BindingsWrapper(Mutex::new(Pdfium::new(bindings))))
        })
        .clone()
}
