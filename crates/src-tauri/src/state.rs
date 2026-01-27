use launcher_core::utils::{ProgressBar, ProgressbarBase};
use serde_json::json;
use tauri::{AppHandle, Emitter};

pub struct TauriProgressbar {
    inner: ProgressBar,
    id: String,
    status: String,
    app_handle: AppHandle,
}

impl TauriProgressbar {
    pub fn new(id: impl Into<String>, app_handle: AppHandle) -> Self {
        Self {
            inner: ProgressBar::default(),
            id: id.into(),
            status: String::new(),
            app_handle,
        }
    }
}

impl ProgressbarBase for TauriProgressbar {
    fn reset(&mut self) {
        self.inner.reset()
    }

    fn set_total(&mut self, total: usize) {
        self.inner.set_total(total)
    }

    fn set_current(&mut self, current: usize) {
        self.inner.set_current(current)
    }

    fn update(&mut self, increase: usize) {
        self.inner.update(increase)
    }
}

impl TauriProgressbar {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_status(&self) -> String {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: impl Into<String>) {
        self.status = status.into();
        self.emit();
    }

    fn emit(&self) {
        self.app_handle.emit(self.id.as_str(), json!({
            "status": self.status.clone(),
            "progress": self.inner.progress(),
            "total": self.inner.total,
            "current": self.inner.current
        })).unwrap();
    }
}