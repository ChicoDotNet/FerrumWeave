use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use ferrumweave_docgraph::{Mode, process_repository};

fn temp_repo() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("ferrumweave-docgraph-{}-{nonce}", std::process::id()))
}

#[test]
fn fw_doc_nav_005_render_then_check_is_idempotent() {
    let root = temp_repo();
    fs::create_dir_all(root.join("docs/architecture")).expect("create architecture directory");

    fs::write(root.join("README.es.md"), "# FerrumWeave\n").expect("write project readme");
    fs::write(root.join("docs/README.es.md"), "# Documentación\n").expect("write docs home");
    fs::write(
        root.join("docs/source.es.md"),
        "# Fuente\n\n[Estructura](architecture/repository-layout.es.md)\n",
    )
    .expect("write backlink source");
    fs::write(
        root.join("docs/architecture/repository-layout.es.md"),
        "<!--\ndoc-id: architecture.repository-layout\nlocale: es\ntranslation-of: docs/architecture/repository-layout.md\n-->\n\n# Estructura del repositorio\n\nContenido.\n",
    )
    .expect("write managed document");

    let changed = process_repository(&root, Mode::Render).expect("render docgraph");
    assert_eq!(
        changed,
        vec![PathBuf::from("docs/architecture/repository-layout.es.md")]
    );

    let rendered = fs::read_to_string(root.join("docs/architecture/repository-layout.es.md"))
        .expect("read rendered document");
    assert!(rendered.contains("[← Documentación](../README.es.md)"));
    assert!(rendered.contains("[README del proyecto](../../README.es.md)"));
    assert!(rendered.contains("## Qué enlaza aquí"));
    assert!(rendered.contains("[Fuente](../source.es.md)"));

    assert!(
        process_repository(&root, Mode::Check)
            .expect("check rendered graph")
            .is_empty()
    );

    fs::remove_dir_all(root).expect("remove temp repository");
}
