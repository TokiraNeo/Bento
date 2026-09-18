/*
 * ---- Bento ----
 * Copyright (C) 2026-present TokiraNeo <TokiraNeo@outlook.com>
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod config;
mod embedder;
mod indexer;

pub use config::SemanticRetrieveConfig;
pub(crate) use embedder::{EmbedVector, SemanticEmbedder};
pub(crate) use indexer::SemanticIndexer;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bento_protocol::tool::{ToolDefinition, ToolRisk};

    use crate::model::IndexedTool;

    use super::*;

    fn tool(name: &str, description: &str, tags: &[&str]) -> Arc<IndexedTool> {
        Arc::new(IndexedTool::new(
            "BentoTest",
            "BentoTest#1",
            ToolDefinition {
                name: name.to_string(),
                description: description.to_string(),
                tags: tags.iter().map(|s| s.to_string()).collect(),
                risk: ToolRisk::Normal,
                input_schema: serde_json::json!({ "type": "object" }),
            },
        ))
    }

    #[test]
    fn test_semantic_index() {
        let config = SemanticRetrieveConfig::default();

        let embedder = SemanticEmbedder::new().expect("failed to create embedder");

        let tools = vec![
            tool("search_user", "tool for searching user.", &[]),
            tool("get_weather", "tool for getting weather.", &[]),
            tool("translate", "tool for translating text.", &[]),
            tool("purchase", "tool for purchasing items.", &[]),
            tool("shop", "tool for shopping.", &[]),
            tool("online_website", "tool for online website.", &[]),
        ];

        let indexer = SemanticIndexer::build(&tools, &config);

        let docs: Vec<String> = tools
            .iter()
            .map(|tool| tool.semantic_query.clone())
            .collect();

        let embeddings = embedder.embed_docs(&docs).expect("embedding docs failed");

        indexer.update(embeddings);

        let query = "buy guitar online";
        let query_embedding = embedder.embed_query(query).expect("embedding query failed");

        println!("{:?}", indexer.search(query_embedding));
    }
}
