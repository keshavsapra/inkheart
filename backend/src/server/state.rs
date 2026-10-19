use std::{
    io,
    ops::Deref,
    sync::{mpsc, Arc, Mutex},
};

use firebase_admin_auth_rs::jwk_auth::JwkAuth;

use crate::{
    config::Config,
    indexer::{
        text_search::{TextIndexJob, TextSearchIndex},
        LookupMap,
    },
    pdfium::bindings::BindingsWrapper,
    user_settings::UserSettings,
};

pub struct AppData {
    pub lookup_table: LookupMap,
    pub config: Config,
    pub auth: Option<JwkAuth>,
    pub pdfium: Arc<BindingsWrapper>,
    pub is_indexer_running: bool,
    pub text_index: Option<Arc<TextSearchIndex>>,
    pub text_index_sender: Option<mpsc::Sender<TextIndexJob>>,
    pub is_text_indexer_running: bool,
    pub text_index_total: usize,
    pub text_index_processed: usize,
}

#[derive(Clone)]
pub struct AppState(pub Arc<Mutex<AppData>>);

impl AppState {
    pub fn new(app_data: AppData) -> Self {
        AppState(Arc::new(Mutex::new(app_data)))
    }

    pub fn get_config(&self) -> io::Result<Config> {
        let data = self.0.lock().map_err(|_| io::ErrorKind::Other)?;
        Ok(data.config.clone())
    }

    pub fn get_user_settings(&self) -> io::Result<UserSettings> {
        let config_dir = self.get_config_dir()?;

        UserSettings::load(&config_dir)
    }

    pub fn save_user_settings(&self, settings: &UserSettings) -> io::Result<()> {
        let config_dir = self.get_config_dir()?;

        settings.save(&config_dir)
    }

    pub fn get_pdfium(&self) -> io::Result<Arc<BindingsWrapper>> {
        let data = self.0.lock().map_err(|_| io::ErrorKind::Other)?;
        Ok(data.pdfium.clone())
    }

    pub fn get_indexer_status(&self) -> io::Result<bool> {
        let data = self.0.lock().map_err(|_| io::ErrorKind::Other)?;
        Ok(data.is_indexer_running)
    }

    pub fn set_indexer_status(&self, status: bool) -> io::Result<()> {
        let mut data = self.0.lock().map_err(|_| io::ErrorKind::Other)?;
        data.is_indexer_running = status;
        Ok(())
    }

    pub fn get_lookup(&self) -> io::Result<LookupMap> {
        let data = self.0.lock().map_err(|_| io::ErrorKind::Other)?;
        Ok(data.lookup_table.clone())
    }

    pub fn get_config_dir(&self) -> io::Result<String> {
        let config = self.get_config()?;
        Ok(config.config_dir.clone())
    }

    pub fn get_text_index(&self) -> io::Result<Option<Arc<TextSearchIndex>>> {
        let data = self.0.lock().map_err(|_| io::ErrorKind::Other)?;
        Ok(data.text_index.clone())
    }

    pub fn set_text_index_sender(&self, sender: mpsc::Sender<TextIndexJob>) -> io::Result<()> {
        let mut data = self.0.lock().map_err(|_| io::ErrorKind::Other)?;
        data.text_index_sender = Some(sender);
        Ok(())
    }

    pub fn get_text_index_sender(&self) -> io::Result<Option<mpsc::Sender<TextIndexJob>>> {
        let data = self.0.lock().map_err(|_| io::ErrorKind::Other)?;
        Ok(data.text_index_sender.clone())
    }

    pub fn get_text_indexer_status(&self) -> io::Result<bool> {
        let data = self.0.lock().map_err(|_| io::ErrorKind::Other)?;
        Ok(data.is_text_indexer_running)
    }

    pub fn set_text_indexer_status(&self, status: bool) -> io::Result<()> {
        let mut data = self.0.lock().map_err(|_| io::ErrorKind::Other)?;
        data.is_text_indexer_running = status;
        Ok(())
    }

    pub fn set_text_index_progress(&self, total: usize, processed: usize) -> io::Result<()> {
        let mut data = self.0.lock().map_err(|_| io::ErrorKind::Other)?;
        data.text_index_total = total;
        data.text_index_processed = processed;
        Ok(())
    }

    pub fn increment_text_index_processed(&self) -> io::Result<()> {
        let mut data = self.0.lock().map_err(|_| io::ErrorKind::Other)?;
        data.text_index_processed += 1;
        Ok(())
    }

    pub fn get_text_index_progress(&self) -> io::Result<(usize, usize)> {
        let data = self.0.lock().map_err(|_| io::ErrorKind::Other)?;
        Ok((data.text_index_total, data.text_index_processed))
    }

    pub fn set_lookup(&self, lookup: LookupMap) -> io::Result<()> {
        let mut data = self.0.lock().map_err(|_| io::ErrorKind::Other)?;
        data.lookup_table = lookup;
        Ok(())
    }
}

impl Deref for AppState {
    type Target = Arc<Mutex<AppData>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
