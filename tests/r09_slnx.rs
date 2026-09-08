use std::fs;
use std::path::PathBuf;

#[test]
fn mixed_solution_declares_all_four_required_project_language_families() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let solution = repo.join("tests/fixtures/r09/Enterprise.slnx");
    let slnx = fs::read_to_string(&solution).unwrap_or_else(|error| {
        panic!(
            "R09 requires the canonical mixed-language solution at {}: {error}",
            solution.display()
        )
    });

    for project in [
        "Legacy/Legacy.vbproj",
        "Domain/Domain.csproj",
        "Analytics/Analytics.fsproj",
        "RiskEngine/RiskEngine.rsproj",
    ] {
        assert!(
            slnx.contains(project),
            "R09 mixed solution must include {project}"
        );
    }
}
