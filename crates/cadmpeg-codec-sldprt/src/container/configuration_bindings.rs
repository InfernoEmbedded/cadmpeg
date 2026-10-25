// SPDX-License-Identifier: Apache-2.0
//! Redundant native configuration identities across Features and Keywords.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct ConfigurationBindings {
    keywords: Vec<Keyword>,
    documents: Vec<Manifest>,
    resolved: BTreeMap<String, Option<usize>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Keyword {
    name: Option<String>,
    source_index: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Manifest {
    configurations: Vec<Configuration>,
    models: Vec<Model>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Configuration {
    name: Option<String>,
    index: Option<String>,
    model: Option<String>,
    most_recent: Option<String>,
    needs_update: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Model {
    id: Option<String>,
    name: Option<String>,
    index: Option<String>,
}

fn decimal(value: Option<&str>) -> Option<usize> {
    let value = value.filter(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))?;
    value.parse().ok()
}

impl ConfigurationBindings {
    pub(super) fn record_keywords(&mut self, document: &roxmltree::Document<'_>) {
        self.keywords.extend(
            document
                .descendants()
                .filter(|node| node.is_element() && node.tag_name().name() == "Configuration")
                .map(|node| Keyword {
                    name: node.attribute("Name").map(str::to_owned),
                    source_index: node.attribute("SourceIndex").map(str::to_owned),
                }),
        );
    }

    pub(super) fn record_features(&mut self, document: &roxmltree::Document<'_>) {
        let mut manifest = Manifest::default();
        for node in document.descendants().filter(roxmltree::Node::is_element) {
            match node.tag_name().name() {
                "swConfiguration" => manifest.configurations.push(Configuration {
                    name: node.attribute("swName").map(str::to_owned),
                    index: node.attribute("swID").map(str::to_owned),
                    model: node.attribute("swModelRef").map(str::to_owned),
                    most_recent: node
                        .attribute("swMostRecentConfiguration")
                        .map(str::to_owned),
                    needs_update: node
                        .attribute("swConfigurationNeedsUpdate")
                        .map(str::to_owned),
                }),
                "swModel" => manifest.models.push(Model {
                    id: node.attribute("id").map(str::to_owned),
                    name: node.attribute("swConfigurationName").map(str::to_owned),
                    index: node.attribute("swConfigurationId").map(str::to_owned),
                }),
                _ => {}
            }
        }
        self.documents.push(manifest);
    }

    pub(super) fn has_manifest_rows(&self) -> bool {
        self.documents.iter().any(|d| !d.configurations.is_empty())
    }

    fn rows(&self) -> impl Iterator<Item = (&Manifest, &Configuration)> {
        self.documents.iter().flat_map(|document| {
            document
                .configurations
                .iter()
                .map(move |row| (document, row))
        })
    }

    pub(super) fn finish(&mut self) {
        let mut keywords = BTreeMap::<&str, Vec<&Keyword>>::new();
        let mut explicit_counts = BTreeMap::<usize, usize>::new();
        for keyword in &self.keywords {
            if let Some(index) = decimal(keyword.source_index.as_deref()) {
                *explicit_counts.entry(index).or_default() += 1;
            }
            if let Some(name) = keyword.name.as_deref() {
                keywords.entry(name).or_default().push(keyword);
            }
        }
        let mut rows = BTreeMap::<&str, Vec<(usize, &Configuration)>>::new();
        let mut counts = BTreeMap::<usize, usize>::new();
        let mut update_required = BTreeSet::new();
        let mut models = Vec::new();
        for (document_index, document) in self.documents.iter().enumerate() {
            let mut by_id = BTreeMap::<&str, Vec<&Model>>::new();
            for model in &document.models {
                if let Some(id) = model.id.as_deref() {
                    by_id.entry(id).or_default().push(model);
                }
            }
            models.push(by_id);
            for row in &document.configurations {
                if let Some(name) = row.name.as_deref() {
                    rows.entry(name).or_default().push((document_index, row));
                }
                if let Some(index) = decimal(row.index.as_deref()) {
                    *counts.entry(index).or_default() += 1;
                    if row.needs_update.as_deref() == Some("YES") {
                        update_required.insert(index);
                    }
                }
            }
        }
        let mut resolved = BTreeMap::new();
        for (name, group) in keywords {
            let index = (|| {
                let [keyword] = group.as_slice() else {
                    return None;
                };
                if name.is_empty() {
                    return None;
                }
                let manifest_index = (|| {
                    let [(document, row)] = rows.get(name)?.as_slice() else {
                        return None;
                    };
                    if row.needs_update.as_deref() != Some("NO") {
                        return None;
                    }
                    let index = decimal(row.index.as_deref())?;
                    if counts.get(&index) != Some(&1) {
                        return None;
                    }
                    let reference = row.model.as_deref().filter(|s| !s.is_empty())?;
                    let [model] = models[*document].get(reference)?.as_slice() else {
                        return None;
                    };
                    if model.name.as_deref() != Some(name)
                        || model.index.as_deref() != row.index.as_deref()
                    {
                        return None;
                    }
                    Some(index)
                })();
                let Some(raw_index) = keyword.source_index.as_deref() else {
                    return manifest_index;
                };
                let index = decimal(Some(raw_index))?;
                if explicit_counts.get(&index) != Some(&1)
                    || counts.get(&index).is_some_and(|count| *count > 1)
                    || rows.get(name).is_some_and(|rows| rows.len() > 1)
                    || update_required.contains(&index)
                    || manifest_index.is_some_and(|manifest| manifest != index)
                {
                    return None;
                }
                Some(index)
            })();
            resolved.insert(name.to_owned(), index);
        }
        self.resolved = resolved;
    }

    pub(super) fn index_for(&self, name: &str) -> Option<Option<usize>> {
        self.has_manifest_rows()
            .then(|| self.resolved.get(name).copied().flatten())
    }

    pub(super) fn active(&self) -> super::ManifestActiveConfiguration {
        if !self.has_manifest_rows() {
            return super::ManifestActiveConfiguration::Absent;
        }
        let mut rows = self
            .rows()
            .filter(|(_, r)| r.most_recent.as_deref() == Some("YES"));
        let Some((document, row)) = rows.next() else {
            return super::ManifestActiveConfiguration::Ambiguous;
        };
        if rows.next().is_some() {
            return super::ManifestActiveConfiguration::Ambiguous;
        }
        let Some(index) = decimal(row.index.as_deref()) else {
            return super::ManifestActiveConfiguration::Ambiguous;
        };
        if self
            .rows()
            .filter(|(_, row)| decimal(row.index.as_deref()) == Some(index))
            .count()
            != 1
        {
            return super::ManifestActiveConfiguration::Ambiguous;
        }
        let mut names = self
            .resolved
            .iter()
            .filter(|(_, value)| **value == Some(index));
        let Some((name, _)) = names.next() else {
            return super::ManifestActiveConfiguration::Ambiguous;
        };
        if names.next().is_some() || row.name.as_deref().is_some_and(|value| value != name) {
            return super::ManifestActiveConfiguration::Ambiguous;
        }
        if let Some(reference) = row.model.as_deref() {
            if reference.is_empty() {
                return super::ManifestActiveConfiguration::Ambiguous;
            }
            let mut models = document
                .models
                .iter()
                .filter(|model| model.id.as_deref() == Some(reference));
            let Some(model) = models.next() else {
                return super::ManifestActiveConfiguration::Ambiguous;
            };
            if models.next().is_some()
                || model.name.as_deref().is_some_and(|value| value != name)
                || model
                    .index
                    .as_deref()
                    .is_some_and(|value| Some(value) != row.index.as_deref())
            {
                return super::ManifestActiveConfiguration::Ambiguous;
            }
        }
        super::ManifestActiveConfiguration::Unique(index, Some(name.clone()))
    }
}

#[cfg(test)]
mod tests;
