use crate::model::release_type::ReleaseType;
use crate::model::severity::Severity;
use crate::model::update::Update;

#[derive(Clone)]
pub struct Partition {
        _updates: Vec<String>,
        _release_type: ReleaseType,
        _severity: Severity
}

impl Partition {
    pub fn new() -> Partition {
        Partition {
                _updates: Vec::new(),
                _release_type: ReleaseType::Repack,
                _severity: Severity::None
        }
    }

    pub fn add_update(&mut self, update: Update) {
        let update_release_type: ReleaseType = update.get_release_type().clone();
        let update_severity: Severity = update.get_severity().clone();
        let signature: String = update.get_signature().clone();

        if update_release_type > self._release_type {
                self._release_type = update_release_type
        }

        if update_severity > self._severity {
                self._severity = update_severity
        }

        self._updates.push(signature);
    }

    pub fn get_release_type(&self) -> &ReleaseType {
        return &self._release_type
    }

    pub fn get_severity(&self) -> &Severity {
        return &self._severity
    }

    pub fn get_signatures(&self) -> &Vec<String> {
        return &self._updates
    }
}