use chrono::Utc;
use chrono::DateTime;

use crate::model::{release_type::ReleaseType, severity::Severity};

#[derive(Clone)]
pub struct Partition {
        _id: String,
        _release_type: ReleaseType,
        _severity: Severity,
        _date: DateTime<Utc>
}

impl Partition {
    pub fn new(
        id: String, release_type: ReleaseType,
        severity: Severity, date: DateTime<Utc>
    ) -> Partition {
        assert!(!id.is_empty());
        assert!(date <= Utc::now());
        
        Partition {
                _id: id,
                _release_type: release_type,
                _severity: severity,
                _date: date
        }
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

    pub fn get_id(&self) -> &String {
        &self._id
    }
}