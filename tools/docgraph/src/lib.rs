use std::path::{Path, PathBuf};

pub const NAV_START: &str = "<!-- ferrumweave-nav:start -->";
pub const NAV_END: &str = "<!-- ferrumweave-nav:end -->";
pub const BACKLINKS_START: &str = "<!-- ferrumweave-backlinks:start -->";
pub const BACKLINKS_END: &str = "<!-- ferrumweave-backlinks:end -->";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocMetadata {
    pub doc_id: String,
    pub locale: String,
    pub translation_of: Option<String>,
}

pub fn parse_metadata(_markdown: &str) -> Option<DocMetadata> {
    None
}

pub fn strip_generated_blocks(markdown: &str) -> String {
    markdown.to_owned()
}

pub fn resolve_markdown_link(_source: &Path, _target: &str) -> Option<PathBuf> {
    None
}

pub fn localized_docs_home(_document: &Path, _locale: &str) -> PathBuf {
    PathBuf::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fw_doc_nav_001_reads_doc_id_and_locale_metadata() {
        let source = "<!--\ndoc-id: architecture.repository-layout\nlocale: es\ntranslation-of: docs/architecture/repository-layout.md\n-->\n\n# Estructura\n";

        assert_eq!(
            parse_metadata(source),
            Some(DocMetadata {
                doc_id: "architecture.repository-layout".into(),
                locale: "es".into(),
                translation_of: Some("docs/architecture/repository-layout.md".into()),
            })
        );
    }

    #[test]
    fn fw_doc_nav_002_generated_blocks_do_not_feed_the_link_graph() {
        let source = format!(
            "before\n{NAV_START}\n[Generated](../README.md)\n{NAV_END}\nafter\n{BACKLINKS_START}\n[Also generated](x.md)\n{BACKLINKS_END}\n"
        );

        assert_eq!(strip_generated_blocks(&source), "before\nafter\n");
    }

    #[test]
    fn fw_doc_nav_003_resolves_relative_markdown_targets_without_fragments() {
        let source = Path::new("docs/architecture/adr/0004.md");
        assert_eq!(
            resolve_markdown_link(source, "../../roadmap/template-release-plan.es.md#gate"),
            Some(PathBuf::from("docs/roadmap/template-release-plan.es.md"))
        );
        assert_eq!(resolve_markdown_link(source, "https://example.com/x.md"), None);
        assert_eq!(resolve_markdown_link(source, "#local-anchor"), None);
    }

    #[test]
    fn fw_doc_nav_004_docs_home_tracks_the_current_locale() {
        let document = Path::new("docs/architecture/repository-layout.es.md");
        assert_eq!(
            localized_docs_home(document, "es"),
            PathBuf::from("docs/README.es.md")
        );
        assert_eq!(
            localized_docs_home(document, "en"),
            PathBuf::from("docs/README.md")
        );
    }
}
