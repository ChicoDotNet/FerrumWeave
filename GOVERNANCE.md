# Governance

FerrumWeave is currently a pre-alpha project under a **founding maintainer model**.

This document describes how decisions are made now and the direction in which governance should evolve if the project earns a broader community.

## Principles

FerrumWeave governance should optimize for:

- technical correctness over authority;
- evidence over status;
- transparent decisions over private architecture;
- interoperability over ecosystem tribalism;
- small reversible increments over speculative rewrites;
- upstream collaboration over permanent private forks;
- contributor access based on demonstrated stewardship, not employer affiliation;
- the ability to become neutrally governed if the project grows beyond its founder.

## Current model

During architectural discovery, the repository owner acts as founding maintainer and final steward of merge decisions.

That authority exists to keep the project coherent while the technical contracts are still being discovered. It is not intended to become permanent personal control if FerrumWeave develops a healthy multi-maintainer community.

Maintainers are expected to explain material technical decisions publicly whenever security, privacy, or legal constraints do not require confidentiality.

## Decision levels

### Local and reversible

Examples:

- typo fixes;
- focused tests;
- internal refactors with unchanged contracts;
- documentation improvements;
- local tooling improvements.

These can normally proceed through ordinary pull-request review.

### Cross-cutting technical decisions

Examples:

- Rust ↔ CLR type mappings;
- ownership and managed-reference semantics;
- exceptions and `Result`;
- async mapping;
- public project-system behavior;
- metadata shape;
- compatibility promises;
- long-lived upstream divergence.

These should begin with a design issue, RFC-style discussion, or architecture decision record before a large implementation is merged.

### Project-policy decisions

Examples:

- licenses;
- contribution certification;
- Code of Conduct;
- security policy;
- governance model;
- trademarks and project assets;
- transfer to a neutral foundation.

These require explicit maintainer approval and should be documented in the repository.

## Maintainers

Maintainer status should be earned through repeated, constructive stewardship such as:

- technically sound contributions;
- high-quality review;
- care for compatibility and users;
- constructive cross-ecosystem collaboration;
- responsible handling of security and community concerns;
- willingness to maintain work after it merges.

As the project grows, maintainer responsibilities and areas of ownership should be documented explicitly rather than inferred from GitHub permissions.

## Consensus and disagreement

FerrumWeave should seek consensus, but consensus does not mean unanimity.

When reasonable people disagree:

1. state the contract or decision clearly;
2. identify the Rust-facing and .NET-facing consequences;
3. gather executable evidence where possible;
4. prefer the option that preserves future choices when evidence is incomplete;
5. record the decision and its rationale;
6. revisit it when materially new evidence appears.

Technical decisions are not loyalty tests to either ecosystem.

## Upstream relationships

FerrumWeave expects meaningful interaction with Rust and .NET upstream projects.

The project should avoid creating governance incentives that require keeping patches private or maintaining unnecessary forks. Where an upstream contribution is practical and beneficial, upstreaming is preferred.

FerrumWeave does not currently claim endorsement, affiliation, or governance authority from the Rust Project, Rust Foundation, Microsoft, .NET Foundation, Linux Foundation, or any other external organization.

## Path to neutral governance

A successful FerrumWeave should be capable of outgrowing its founder.

Signals that would justify evolving governance include:

- multiple sustained maintainers;
- meaningful adoption outside the founding organization/account;
- recurring contributions from multiple companies or communities;
- ecosystem dependencies that benefit from neutral stewardship;
- trademark, funding, or infrastructure needs that are better handled institutionally.

At that stage, maintainers should evaluate a more formal open-governance model and, if useful, a neutral foundation home.

Transfer of repositories, trademarks, domains, funds, or other community assets should be treated as a project-policy decision and documented transparently.

## No purchased authority

Sponsorship, employment, vendor relationships, or funding do not automatically grant technical or governance authority.

Financial support may enable work. It does not replace review, evidence, compatibility obligations, or community standards.

## Amendments

Changes to this governance document should be proposed through a pull request with a clear explanation of the problem being solved and the practical effect of the change.
