use chrono::Utc;
use chrono::DateTime;

use crate::model::{semantic_version::SemanticVersion, security_classification::SecurityClassification};

#[derive(Clone)]
pub struct Partition {
        _id: String,
        _release_type: SemanticVersion,
        _security_classification: SecurityClassification,
        _date: DateTime<Utc>
}

impl Partition {
    pub fn new(
        id: String, release_type: SemanticVersion,
        severity: SecurityClassification, date: DateTime<Utc>
    ) -> Partition {
        assert!(!id.is_empty());
        assert!(date <= Utc::now());
        
        Partition {
                _id: id,
                _release_type: release_type,
                _security_classification: severity,
                _date: date
        }
    }

    pub fn get_release_type(&self) -> &SemanticVersion {
        &self._release_type
    }

    pub fn get_security_classification(&self) -> &SecurityClassification {
        &self._security_classification
    }

    pub fn get_date(&self) -> &DateTime<Utc> {
        &self._date
    }

    pub fn get_id(&self) -> &String {
        &self._id
    }
}