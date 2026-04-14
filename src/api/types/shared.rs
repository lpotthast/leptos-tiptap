use serde::{Deserialize, Serialize};
use serde_json::Map;

/// Arbitrary Tiptap node or mark attributes.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TiptapAttributes(pub(crate) Map<String, serde_json::Value>);

impl TiptapAttributes {
    /// Creates an empty attribute map.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts an attribute value and returns the previous value, if present.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> Option<serde_json::Value> {
        self.0.insert(key.into(), value.into())
    }

    /// Returns an attribute value by key.
    #[must_use]
    pub fn get(&self, key: impl AsRef<str>) -> Option<&serde_json::Value> {
        self.0.get(key.as_ref())
    }

    /// Returns the underlying attribute map.
    #[must_use]
    pub fn as_map(&self) -> &Map<String, serde_json::Value> {
        &self.0
    }

    /// Returns the underlying attribute map mutably.
    pub fn as_mut_map(&mut self) -> &mut Map<String, serde_json::Value> {
        &mut self.0
    }

    /// Consumes the attributes and returns the underlying map.
    #[must_use]
    pub fn into_map(self) -> Map<String, serde_json::Value> {
        self.0
    }
}

impl From<Map<String, serde_json::Value>> for TiptapAttributes {
    fn from(value: Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}

impl<K, V> FromIterator<(K, V)> for TiptapAttributes
where
    K: Into<String>,
    V: Into<serde_json::Value>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut attributes = Self::new();
        attributes.extend(iter);
        attributes
    }
}

impl<K, V> Extend<(K, V)> for TiptapAttributes
where
    K: Into<String>,
    V: Into<serde_json::Value>,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.0.extend(
            iter.into_iter()
                .map(|(key, value)| (key.into(), value.into())),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;

    #[test]
    fn attributes_support_map_accessors_and_collect() {
        let mut attributes = [
            ("href", serde_json::json!("https://example.com")),
            ("rel", serde_json::json!("noopener")),
        ]
        .into_iter()
        .collect::<TiptapAttributes>();

        assert_that!(attributes.get("href")).is_some();

        attributes
            .as_mut_map()
            .insert("target".to_owned(), serde_json::json!("_blank"));

        assert_that!(attributes.as_map().contains_key("target")).is_true();
        assert_that!(attributes.into_map().contains_key("rel")).is_true();
    }
}
