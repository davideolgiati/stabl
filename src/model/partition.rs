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


#[cfg(test)]
mod tests {
    use chrono::Days;

    use super::*;
    
    #[test]
    fn happy_path_new_partition() {
            let partition_id: String = "FEDORA-2025-db6c37de88".to_string();
            let release_type: SemanticVersion = SemanticVersion::Major;
            let severity: SecurityClassification = SecurityClassification::None;
            let release_ts: DateTime<Utc> = chrono::NaiveDateTime::parse_from_str("2025-03-17 01:37:24", "%F %X")
                        .unwrap().and_utc();

            let output = Partition::new(
                    partition_id.clone(), release_type.clone(), 
                    severity.clone(), release_ts
            );

            assert_eq!(*output.get_release_type(), release_type);
            assert_eq!(*output.get_id(), partition_id);
            assert_eq!(*output.get_security_classification(), severity);
            assert_eq!(*output.get_date(), release_ts);
    }

    #[test]
    #[should_panic]
    fn panic_new_partition_empty_id() {
            let partition_id: String = "".to_string();
            let release_type: SemanticVersion = SemanticVersion::Major;
            let severity: SecurityClassification = SecurityClassification::None;
            let release_ts: DateTime<Utc> = chrono::NaiveDateTime::parse_from_str("2025-03-17 01:37:24", "%F %X")
                        .unwrap().and_utc();

            Partition::new(
                    partition_id.clone(), release_type.clone(), 
                    severity.clone(), release_ts
            );
    }

    #[test]
    #[should_panic]
    fn panic_new_partition_release_ts_in_the_future() {
            let partition_id: String = "FEDORA-2025-db6c37de88".to_string();
            let release_type: SemanticVersion = SemanticVersion::Major;
            let severity: SecurityClassification = SecurityClassification::None;
            let release_ts: DateTime<Utc> = Utc::now()
                .checked_add_days(Days::new(1)).unwrap();

            Partition::new(
                    partition_id.clone(), release_type.clone(), 
                    severity.clone(), release_ts
            );
    }
}