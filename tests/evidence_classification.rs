use std::fs;

fn contract_blocks(path: &str) -> Vec<String> {
    let text =
        fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"));
    text.split("[[contracts]]")
        .skip(1)
        .map(str::trim)
        .filter(|block| !block.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn contract_id(block: &str) -> &str {
    block
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("id = \"")
                .and_then(|rest| rest.strip_suffix('"'))
        })
        .unwrap_or("<missing-id>")
}

fn quoted_field<'a>(block: &'a str, key: &str) -> Option<&'a str> {
    block.lines().find_map(|line| {
        let (name, value) = line.trim().split_once('=')?;
        if name.trim() != key {
            return None;
        }

        value
            .trim()
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
    })
}

#[test]
fn historical_r02_and_r03_contracts_are_explicitly_oracle_proven() {
    for path in ["tests/r02/contracts.toml", "tests/r03/contracts.toml"] {
        let blocks = contract_blocks(path);
        assert!(!blocks.is_empty(), "{path} must contain contracts");

        for block in blocks {
            assert!(
                block.contains("evidence_level = \"oracle-proven\""),
                "{} in {path} must be classified as oracle-proven; historical rustc_codegen_clr success is characterization evidence, not FerrumWeave product-backend evidence",
                contract_id(&block)
            );
        }
    }
}

#[test]
fn r05_source_causal_contracts_name_the_ferrumweave_backend_evidence() {
    let path = "tests/r05/contracts.toml";
    let blocks = contract_blocks(path);
    let source_causal: Vec<_> = blocks
        .iter()
        .filter(|block| block.contains("evidence_level = \"source-causal-certified\""))
        .collect();

    assert!(
        !source_causal.is_empty(),
        "R05 must contain source-causal-certified contracts"
    );

    for block in source_causal {
        assert!(
            block.contains("backend_evidence = \"FerrumWeave-backend-proven\""),
            "{} must explicitly identify FerrumWeave-backend-proven evidence in addition to source-causal-certified evidence",
            contract_id(block)
        );
    }
}

#[test]
fn r05_managed_consumption_source_causality_census_is_complete() {
    const REQUIRED_CONTRACTS: [&str; 5] = [
        "FW-R05-DOTNET-009",
        "FW-R05-DOTNET-010",
        "FW-R05-DOTNET-011",
        "FW-R05-DOTNET-012",
        "FW-R05-DOTNET-013",
    ];

    let path = "tests/r05/contracts.toml";
    let blocks = contract_blocks(path);

    for required_id in REQUIRED_CONTRACTS {
        let block = blocks
            .iter()
            .find(|block| contract_id(block) == required_id)
            .unwrap_or_else(|| panic!("R05 source-causality census is missing {required_id}"));

        assert!(
            block.lines().any(|line| line.trim() == "implemented = true"),
            "{required_id} must remain implemented"
        );
        assert_eq!(
            quoted_field(block, "evidence_level"),
            Some("source-causal-certified"),
            "{required_id} must remain source-causal-certified"
        );

        let proof = quoted_field(block, "proof")
            .unwrap_or_else(|| panic!("{required_id} must keep replayable proof"));
        assert!(
            !proof.trim().is_empty() && !proof.starts_with("pending:"),
            "{required_id} must keep non-pending replayable proof"
        );
    }
}
