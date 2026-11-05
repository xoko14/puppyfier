use left_right::{self, Absorb, ReadHandle, WriteHandle};

#[derive(Clone)]
pub struct GlobalConfig {
    pub enabled: bool,
    pub percentage: u32,
    pub words: Vec<String>,
}

impl GlobalConfig {
    pub fn with_words(words: Vec<String>) -> Self {
        Self {
            words,
            ..Default::default()
        }
    }
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            percentage: 20,
            words: Default::default(),
        }
    }
}

enum LRGlobalConfigOp {
    SetEnabled(bool),
    SetPercentage(u32),
    SetWords(Vec<String>),
}

impl Absorb<LRGlobalConfigOp> for GlobalConfig {
    fn absorb_first(&mut self, operation: &mut LRGlobalConfigOp, _other: &Self) {
        match operation {
            LRGlobalConfigOp::SetEnabled(enabled) => self.enabled = *enabled,
            LRGlobalConfigOp::SetPercentage(percentage) => self.percentage = *percentage,
            LRGlobalConfigOp::SetWords(items) => self.words = items.clone(),
        }
    }

    fn sync_with(&mut self, first: &Self) {
        *self = first.clone();
    }
}

pub struct LRGlobalConfigWriter(WriteHandle<GlobalConfig, LRGlobalConfigOp>);

impl LRGlobalConfigWriter {
    pub fn set_enabled(&mut self, enabled: bool) {
        self.0.append(LRGlobalConfigOp::SetEnabled(enabled));
    }

    pub fn set_percentage(&mut self, percentage: u32) {
        self.0.append(LRGlobalConfigOp::SetPercentage(percentage));
    }

    pub fn set_words(&mut self, words: Vec<String>) {
        self.0.append(LRGlobalConfigOp::SetWords(words));
    }

    pub fn publish(&mut self) {
        self.0.publish();
    }
}

pub struct LRGlobalConfigReader(ReadHandle<GlobalConfig>);

unsafe impl Sync for LRGlobalConfigReader {}

impl LRGlobalConfigReader {
    pub fn read(&self) -> GlobalConfig {
        self.0.enter().unwrap().clone()
    }
}

pub fn new_from_empty(global_config: GlobalConfig) -> (LRGlobalConfigReader, LRGlobalConfigWriter) {
    let (w, r) = left_right::new_from_empty::<GlobalConfig, LRGlobalConfigOp>(global_config);
    (LRGlobalConfigReader(r), LRGlobalConfigWriter(w))
}
