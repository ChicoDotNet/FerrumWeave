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
