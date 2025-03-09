use chrono::TimeZone;
use chrono::Utc;
use chrono::DateTime;

use crate::model::enums::release_type::ReleaseType;
use crate::model::enums::severity::Severity;
use crate::model::updates::update::Update;

#[derive(Clone)]
pub struct Partition {
        _updates: Vec<Update>,
        _release_type: ReleaseType,
        _severity: Severity,
        _date: DateTime<Utc>
}

impl Partition {
    pub fn new() -> Partition {
        let default_date: DateTime<Utc> = Utc
            .timestamp_opt(0, 0)
            .unwrap();

        Partition {
                _updates: Vec::new(),
                _release_type: ReleaseType::Repack,
                _severity: Severity::None,
                _date: default_date
        }
    }

    pub fn add_update(&mut self, update: &Update) {
        let update_release_type: &ReleaseType = update.get_release_type();
        let update_severity: &Severity = update.get_severity();
        let update_date: &DateTime<Utc> = update.get_release_timestamp();

        if update_release_type > &self._release_type {
                self._release_type = update_release_type.clone()
        }

        if update_severity > &self._severity {
                self._severity = update_severity.clone()
        }

        if update_date > &self._date {
                self._date = *update_date
        }


        self._updates.push(update.clone());
    }

    pub fn get_release_type(&self) -> &ReleaseType {
        &self._release_type
    }

    pub fn get_severity(&self) -> &Severity {
        &self._severity
    }

    pub fn get_date(&self) -> &DateTime<Utc> {
        &self._date
    }

    pub fn get_updates(&self) -> &Vec<Update> {
        &self._updates
    }
}