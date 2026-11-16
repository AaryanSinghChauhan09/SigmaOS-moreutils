// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-moreutils: moreutils component model.
//!
//! Real implementation: moreutils_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoreutilsItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoreutilsError {
    EmptyMoreutilsItemName,
    DuplicateMoreutilsItem(String),
    EmptyEventSubject,
}

impl fmt::Display for MoreutilsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMoreutilsItemName => write!(f, "moreutils_item name must not be empty"),
            Self::DuplicateMoreutilsItem(id) => write!(f, "duplicate moreutils_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MoreutilsError {}

pub struct Moreutils {
    pub name: String,
    moreutils_items: BTreeMap<String, MoreutilsItem>,
    events: BTreeMap<String, Event>,
}

impl Moreutils {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            moreutils_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_moreutils_item(&mut self, item: MoreutilsItem) -> Result<(), MoreutilsError> {
        if item.name.trim().is_empty() {
            return Err(MoreutilsError::EmptyMoreutilsItemName);
        }
        let id = item.id.clone();
        if self.moreutils_items.contains_key(&id) {
            return Err(MoreutilsError::DuplicateMoreutilsItem(id.clone()));
        }
        self.moreutils_items.insert(id, item);
        Ok(())
    }

    pub fn moreutils_item_count(&self) -> usize {
        self.moreutils_items.len()
    }

    pub fn get_moreutils_item(&self, id: &str) -> Option<&MoreutilsItem> {
        self.moreutils_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MoreutilsError> {
        if event.subject.trim().is_empty() {
            return Err(MoreutilsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_moreutils_item(id: &str, name: &str) -> MoreutilsItem {
        MoreutilsItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Moreutils::new("Moreutils");
        assert_eq!(m.name, "Moreutils");
        assert_eq!(m.moreutils_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_moreutils_item() {
        let mut m = Moreutils::new("Moreutils");
        m.add_moreutils_item(sample_moreutils_item("a1", "Alpha")).unwrap();
        assert_eq!(m.moreutils_item_count(), 1);
        assert!(m.get_moreutils_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Moreutils::new("Moreutils");
        assert!(m.add_moreutils_item(sample_moreutils_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Moreutils::new("Moreutils");
        m.add_moreutils_item(sample_moreutils_item("x", "One")).unwrap();
        assert!(m.add_moreutils_item(sample_moreutils_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Moreutils::new("Moreutils");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
