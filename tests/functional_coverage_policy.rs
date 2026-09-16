use std::collections::BTreeSet;
use std::fs;

fn contract_blocks(text: &str) -> Vec<&str> {
    text.split("[[contracts]]")
        .skip(1)
        .map(str::trim)
        .filter(|block| !block.is_empty())
        .collect()
}

fn field_value<'a>(block: &'a str, key: &str) -> Option<&'a str> {
    block.lines().find_map(|line| {
        let (name, value) = line.trim().split_once('=')?;
        if name.trim() == key {
            Some(value.trim())
        } else {
            None
        }
    })
}

fn quoted_field<'a>(block: &'a str, key: &str) -> Option<&'a str> {
    field_value(block, key)?
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
}

fn contract_id(block: &str) -> &str {
    quoted_field(block, "id").unwrap_or("<missing-id>")
}

fn rust_test_names(source: &str) -> BTreeSet<&str> {
    let mut names = BTreeSet::new();
    let mut expects_test_fn = false;

    for line in source.lines() {
        let trimmed = line.trim();

        if trimmed == "#[test]" {
            expects_test_fn = true;
            continue;
        }

        if !expects_test_fn {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("fn ") {
            if let Some((name, _)) = rest.split_once('(') {
                names.insert(name.trim());
            }
            expects_test_fn = false;
            continue;
        }

        if !trimmed.is_empty() && !trimmed.starts_with("#[") {
            expects_test_fn = false;
        }
    }

    names
}

#[test]
fn functional_contract_ledger_is_mapped_to_rust_tests_at_or_above_policy_minimum() {
    let manifest_path = "tests/functional/contracts.toml";
    let tests_path = "tests/functional.rs";

    let manifest = fs::read_to_string(manifest_path)
        .unwrap_or_else(|error| panic!("failed to read {manifest_path}: {error}"));
    let functional_tests = fs::read_to_string(tests_path)
        .unwrap_or_else(|error| panic!("failed to read {tests_path}: {error}"));

    let contracts = contract_blocks(&manifest);
    assert!(
        !contracts.is_empty(),
        "functional contract ledger must not be empty"
    );

    let minimum_percent = manifest
        .lines()
        .find_map(|line| {
            let (name, value) = line.trim().split_once('=')?;
            if name.trim() == "minimum_percent" {
                value.trim().parse::<f64>().ok()
            } else {
                None
            }
        })
        .expect("functional contract ledger must declare a numeric minimum_percent");

    let available_tests = rust_test_names(&functional_tests);
    assert!(
        !available_tests.is_empty(),
        "{tests_path} must contain discoverable #[test] functions"
    );

    let mut covered = 0usize;

    for block in &contracts {
        let id = contract_id(block);
        let implemented = match field_value(block, "implemented") {
            Some("true") => true,
            Some("false") | None => false,
            Some(value) => panic!("{id} has invalid implemented value {value:?}"),
        };

        if !implemented {
            continue;
        }

        let test_name = quoted_field(block, "test")
            .filter(|name| !name.trim().is_empty())
            .unwrap_or_else(|| panic!("{id} must map to a non-empty Rust test name"));

        assert!(
            available_tests.contains(test_name),
            "{id} maps to Rust test {test_name:?}, but {tests_path} does not declare it"
        );

        covered += 1;
    }

    let percent = covered as f64 * 100.0 / contracts.len() as f64;
    assert!(
        percent >= minimum_percent,
        "functional coverage is {covered}/{} = {percent:.2}%, below the {minimum_percent:.2}% gate",
        contracts.len()
    );

    // ADAPT from the Python harness: this test owns ledger/mapping policy, while
    // the same authoritative `cargo test --workspace --tests` invocation runs
    // every mapped test in `tests/functional.rs`. A mapped test failure therefore
    // still fails the Rust certification run without recursively spawning Cargo.
}
