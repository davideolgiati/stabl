use crate::model::release_type::ReleaseType;
use crate::model::severity::Severity;
use crate::model::update::Update;

pub struct Partition {
        _updates: Vec<Update>,
        _release_type: ReleaseType,
        _severity: Severity
}

