use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const NAV_START: &str = "<!-- ferrumweave-nav:start -->";
pub const NAV_END: &str = "<!-- ferrumweave-nav:end -->";
pub const BACKLINKS_START: &str = "<!-- ferrumweave-backlinks:start -->";
pub const BACKLINKS_END: &str = "<!-- ferrumweave-backlinks:end -->";

const LOCALES: [&str; 9] = ["en", "de", "es", "fr", "it", "pt-BR", "ru", "zh-Hans", "ja"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocMetadata {
    pub doc_id: String,
    pub locale: String,
    pub translation_of: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Check,
    Render,
}

#[derive(Debug, Clone)]
struct Document {
    path: PathBuf,
    content: String,
    title: String,
    metadata: Option<DocMetadata>,
}

pub fn parse_metadata(markdown: &str) -> Option<DocMetadata> {
    let trimmed = markdown.trim_start();
    let body = trimmed.strip_prefix("<!--")?;
    let end = body.find("-->")?;
    let comment = &body[..end];

    let mut values = BTreeMap::new();
    for line in comment.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        values.insert(key.trim(), value.trim());
    }

    Some(DocMetadata {
        doc_id: values.get("doc-id")?.to_string(),
        locale: values.get("locale")?.to_string(),
        translation_of: values.get("translation-of").map(ToString::to_string),
    })
}

pub fn strip_generated_blocks(markdown: &str) -> String {
    let mut output = String::new();
    let mut skipping: Option<&str> = None;

    for line in markdown.lines() {
        if skipping.is_some() {
            if Some(line.trim()) == skipping {
                skipping = None;
            }
            continue;
        }

        match line.trim() {
            NAV_START => {
                skipping = Some(NAV_END);
            }
            BACKLINKS_START => {
                skipping = Some(BACKLINKS_END);
            }
            _ => {
                output.push_str(line);
                output.push('\n');
            }
        }
    }

    output
}

pub fn resolve_markdown_link(source: &Path, target: &str) -> Option<PathBuf> {
    let target = target.trim();
    if target.is_empty()
        || target.starts_with('#')
        || target.starts_with("http://")
        || target.starts_with("https://")
        || target.starts_with("mailto:")
        || target.starts_with("javascript:")
    {
        return None;
    }

    let target = target.split(['#', '?']).next()?;
    if !target.ends_with(".md") {
        return None;
    }

    let joined = if let Some(stripped) = target.strip_prefix('/') {
        PathBuf::from(stripped)
    } else {
        source.parent().unwrap_or_else(|| Path::new("")).join(target)
    };

    Some(normalize_path(&joined))
}

pub fn localized_docs_home(_document: &Path, locale: &str) -> PathBuf {
    if locale == "en" {
        PathBuf::from("docs/README.md")
    } else {
        PathBuf::from(format!("docs/README.{locale}.md"))
    }
}

pub fn process_repository(root: &Path, mode: Mode) -> Result<Vec<PathBuf>, String> {
    let paths = collect_markdown_paths(root)?;
    let mut documents = Vec::with_capacity(paths.len());

    for path in paths {
        let absolute = root.join(&path);
        let content = fs::read_to_string(&absolute)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
        documents.push(Document {
            title: first_heading(&content).unwrap_or_else(|| path.display().to_string()),
            metadata: parse_metadata(&content),
            path,
            content,
        });
    }

    validate_managed_documents(&documents)?;
    let incoming = incoming_links(&documents);
    let titles: BTreeMap<PathBuf, String> = documents
        .iter()
        .map(|document| (document.path.clone(), document.title.clone()))
        .collect();

    let mut stale = Vec::new();
    for document in documents.iter().filter(|document| document.metadata.is_some()) {
        let metadata = document.metadata.as_ref().expect("filtered metadata");
        let sources = incoming.get(&document.path).cloned().unwrap_or_default();
        let rendered = render_managed_document(document, metadata, &sources, &titles);
        if rendered == document.content {
            continue;
        }

        stale.push(document.path.clone());
        if mode == Mode::Render {
            fs::write(root.join(&document.path), rendered).map_err(|error| {
                format!("failed to write {}: {error}", document.path.display())
            })?;
        }
    }

    Ok(stale)
}

fn validate_managed_documents(documents: &[Document]) -> Result<(), String> {
    let mut identities = BTreeSet::new();
    for document in documents.iter().filter(|document| document.metadata.is_some()) {
        let metadata = document.metadata.as_ref().expect("filtered metadata");
        if !LOCALES.contains(&metadata.locale.as_str()) {
            return Err(format!(
                "{} declares unsupported locale {}",
                document.path.display(),
                metadata.locale
            ));
        }
        if !identities.insert((metadata.doc_id.clone(), metadata.locale.clone())) {
            return Err(format!(
                "duplicate doc-id/locale pair: {}/{}",
                metadata.doc_id, metadata.locale
            ));
        }
    }
    Ok(())
}

fn incoming_links(documents: &[Document]) -> BTreeMap<PathBuf, Vec<PathBuf>> {
    let known: BTreeSet<PathBuf> = documents.iter().map(|document| document.path.clone()).collect();
    let mut incoming: BTreeMap<PathBuf, BTreeSet<PathBuf>> = BTreeMap::new();

    for source in documents {
        let stripped = strip_generated_blocks(&source.content);
        for line in stripped.lines() {
            if is_language_selector_line(line) {
                continue;
            }
            for target in markdown_links(line) {
                let Some(resolved) = resolve_markdown_link(&source.path, target) else {
                    continue;
                };
                if resolved != source.path && known.contains(&resolved) {
                    incoming.entry(resolved).or_default().insert(source.path.clone());
                }
            }
        }
    }

    incoming
        .into_iter()
        .map(|(target, sources)| (target, sources.into_iter().collect()))
        .collect()
}

fn render_managed_document(
    document: &Document,
    metadata: &DocMetadata,
    sources: &[PathBuf],
    titles: &BTreeMap<PathBuf, String>,
) -> String {
    let stripped = strip_generated_blocks(&document.content);
    let mut base = collapse_blank_lines_around_metadata(&stripped);
    let nav = navigation_block(&document.path, &metadata.locale);
    insert_after_metadata(&mut base, &nav);

    let backlink_sources: Vec<&PathBuf> = sources
        .iter()
        .filter(|source| infer_locale_from_path(source) == metadata.locale)
        .collect();
    let backlinks = backlinks_block(
        &document.path,
        &metadata.locale,
        &backlink_sources,
        titles,
    );

    while base.ends_with("\n\n") {
        base.pop();
    }
    if !base.ends_with('\n') {
        base.push('\n');
    }
    base.push('\n');
    base.push_str(&backlinks);
    base
}

fn navigation_block(document: &Path, locale: &str) -> String {
    let home = localized_docs_home(document, locale);
    let project = localized_project_readme(locale);
    let parent = document.parent().unwrap_or_else(|| Path::new(""));
    let home_link = relative_path(parent, &home);
    let project_link = relative_path(parent, &project);
    let labels = labels(locale);

    format!(
        "{NAV_START}\n[← {}]({}) · [{}]({})\n{NAV_END}\n",
        labels.documentation,
        path_for_markdown(&home_link),
        labels.project_readme,
        path_for_markdown(&project_link)
    )
}

fn backlinks_block(
    document: &Path,
    locale: &str,
    sources: &[&PathBuf],
    titles: &BTreeMap<PathBuf, String>,
) -> String {
    let labels = labels(locale);
    let parent = document.parent().unwrap_or_else(|| Path::new(""));
    let mut block = format!("{BACKLINKS_START}\n## {}\n\n", labels.what_links_here);

    if sources.is_empty() {
        block.push_str(labels.no_backlinks);
        block.push('\n');
    } else {
        for source in sources {
            let link = relative_path(parent, source);
            let title = titles
                .get(*source)
                .map(String::as_str)
                .unwrap_or_else(|| source.to_str().unwrap_or("document"));
            block.push_str(&format!("- [{title}]({})\n", path_for_markdown(&link)));
        }
    }

    block.push_str(BACKLINKS_END);
    block.push('\n');
    block
}

fn insert_after_metadata(content: &mut String, block: &str) {
    let trimmed_start = content.trim_start_matches(['\n', '\r']);
    let leading_len = content.len() - trimmed_start.len();
    let insertion = if trimmed_start.starts_with("<!--") {
        trimmed_start
            .find("-->")
            .map(|index| leading_len + index + 3)
            .unwrap_or(0)
    } else {
        0
    };

    let tail = content[insertion..].trim_start_matches(['\n', '\r']);
    let head = content[..insertion].trim_end_matches(['\n', '\r']);
    *content = if head.is_empty() {
        format!("{block}\n{tail}")
    } else {
        format!("{head}\n\n{block}\n{tail}")
    };
}

fn collapse_blank_lines_around_metadata(content: &str) -> String {
    let mut result = content.to_owned();
    while result.ends_with("\n\n\n") {
        result.pop();
    }
    result
}

fn collect_markdown_paths(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    collect_markdown_paths_inner(root, root, &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn collect_markdown_paths_inner(
    root: &Path,
    current: &Path,
    paths: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let entries = fs::read_dir(current)
        .map_err(|error| format!("failed to read directory {}: {error}", current.display()))?;

    for entry in entries {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path.is_dir() {
            if matches!(name.as_ref(), ".git" | "target" | "node_modules") {
                continue;
            }
            collect_markdown_paths_inner(root, &path, paths)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            paths.push(
                path.strip_prefix(root)
                    .map_err(|error| error.to_string())?
                    .to_path_buf(),
            );
        }
    }
    Ok(())
}

fn first_heading(markdown: &str) -> Option<String> {
    markdown
        .lines()
        .find_map(|line| line.strip_prefix("# ").map(str::trim).map(ToString::to_string))
}

fn markdown_links(line: &str) -> Vec<&str> {
    let mut targets = Vec::new();
    let mut rest = line;
    while let Some(open) = rest.find("](") {
        let after = &rest[open + 2..];
        let Some(close) = after.find(')') else {
            break;
        };
        let target = after[..close].trim();
        let target = target.split_whitespace().next().unwrap_or("");
        if !target.is_empty() {
            targets.push(target.trim_matches(['<', '>']));
        }
        rest = &after[close + 1..];
    }
    targets
}

fn is_language_selector_line(line: &str) -> bool {
    let matches = ["English", "Deutsch", "Español", "Français", "Italiano", "Português", "Русский", "简体中文", "日本語"]
        .iter()
        .filter(|name| line.contains(*name))
        .count();
    matches >= 4
}

fn infer_locale_from_path(path: &Path) -> String {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return "en".to_string();
    };
    for locale in LOCALES.into_iter().filter(|locale| *locale != "en") {
        if name.ends_with(&format!(".{locale}.md")) {
            return locale.to_string();
        }
    }
    "en".to_string()
}

fn localized_project_readme(locale: &str) -> PathBuf {
    if locale == "en" {
        PathBuf::from("README.md")
    } else {
        PathBuf::from(format!("README.{locale}.md"))
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
            Component::RootDir | Component::Prefix(_) => {}
        }
    }
    normalized
}

fn relative_path(from: &Path, to: &Path) -> PathBuf {
    let from_components: Vec<_> = from.components().collect();
    let to_components: Vec<_> = to.components().collect();
    let shared = from_components
        .iter()
        .zip(&to_components)
        .take_while(|(left, right)| left == right)
        .count();

    let mut result = PathBuf::new();
    for _ in shared..from_components.len() {
        result.push("..");
    }
    for component in &to_components[shared..] {
        result.push(component.as_os_str());
    }
    if result.as_os_str().is_empty() {
        result.push(".");
    }
    result
}

fn path_for_markdown(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

struct Labels {
    documentation: &'static str,
    project_readme: &'static str,
    what_links_here: &'static str,
    no_backlinks: &'static str,
}

fn labels(locale: &str) -> Labels {
    match locale {
        "de" => Labels { documentation: "Dokumentation", project_readme: "Projekt-README", what_links_here: "Was hierher verlinkt", no_backlinks: "Noch keine dokumentierten eingehenden Links." },
        "es" => Labels { documentation: "Documentación", project_readme: "README del proyecto", what_links_here: "Qué enlaza aquí", no_backlinks: "Todavía no hay enlaces documentales entrantes." },
        "fr" => Labels { documentation: "Documentation", project_readme: "README du projet", what_links_here: "Pages liées ici", no_backlinks: "Aucun lien documentaire entrant pour le moment." },
        "it" => Labels { documentation: "Documentazione", project_readme: "README del progetto", what_links_here: "Cosa punta qui", no_backlinks: "Nessun collegamento documentale in entrata per ora." },
        "pt-BR" => Labels { documentation: "Documentação", project_readme: "README do projeto", what_links_here: "O que aponta para cá", no_backlinks: "Ainda não há links de documentação apontando para cá." },
        "ru" => Labels { documentation: "Документация", project_readme: "README проекта", what_links_here: "Что ссылается сюда", no_backlinks: "Пока нет входящих ссылок из документации." },
        "zh-Hans" => Labels { documentation: "文档", project_readme: "项目 README", what_links_here: "链入页面", no_backlinks: "目前还没有文档链接到这里。" },
        "ja" => Labels { documentation: "ドキュメント", project_readme: "プロジェクト README", what_links_here: "ここへのリンク", no_backlinks: "現在、このページへのドキュメント内リンクはありません。" },
        _ => Labels { documentation: "Documentation", project_readme: "Project README", what_links_here: "What links here", no_backlinks: "No incoming documentation links yet." },
    }
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

    #[test]
    fn markdown_link_parser_finds_multiple_targets() {
        assert_eq!(
            markdown_links("[one](a.md) and [two](../b.md#x)"),
            vec!["a.md", "../b.md#x"]
        );
    }

    #[test]
    fn relative_paths_are_repository_portable() {
        assert_eq!(
            path_for_markdown(&relative_path(
                Path::new("docs/architecture/adr"),
                Path::new("docs/README.es.md")
            )),
            "../../README.es.md"
        );
    }
}
