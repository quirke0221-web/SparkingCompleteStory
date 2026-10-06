# ADR 0003: Separation of Concerns in Asset Pipeline (Orchestrator vs. Pure Domain Mutation)

## Status
Superseded by [ADR 0009](0009-native-rust-cli-and-unified-build-sandbox.md) (Native Rust CLI and Unified Build Sandbox)

## Context
In the legacy build scripts, domain JSON manipulation (splicing character routes, mutating name maps, cloning data assets) was intermixed with process management, subprocess execution, and ad-hoc file verification in sprawling procedural scripts (`build_complete_story_assets.ps1`, `Extract-RequiredAssets.ps1`, `Build-IoStore.ps1`).

This lack of separation caused:
1. Difficulty testing asset mutation logic independently without launching heavy external CLI binaries.
2. Unclear error reporting when a step failed (e.g. distinguishing between a retoc CLI failure and a JSON structural invalidity).
3. Monolithic scripts that violated single-responsibility principles and file length ceilings.

## Decision
1. **Pipeline Partitioning:** The asset build architecture is strictly divided into two decoupled layers (located under `helpers/` per ADR 0008):
   * **Domain Transformation Layer (`helpers/Transform-CompleteStoryAssets.ps1`):** Pure in-memory JSON data transformation. It accepts input JSON paths, validates baseline structures, mutates the registries and character assets, and writes modified JSON to disk. It contains zero external CLI calls.
   * **Pipeline Orchestration Layer (`helpers/Build-CompleteStory.ps1`):** Master automation cmdlet coordinating external tools (`retoc`, `UAssetGUI`), invoking the transformation script, verifying intermediate artifacts, and packaging release archives.
2. **Ephemeral Staging Isolation:** All intermediate extraction, JSON conversion, and compilation steps must occur in `staging/` (which is excluded from Git via `.gitignore`).
3. **Single Build Entry Point:** Developers and AI agents execute the entire build via a single, parameterized cmdlet (`.\helpers\Build-CompleteStory.ps1`).

## Consequences
* **Positive:** Clean modular architecture. The domain mutation logic can be tested in milliseconds using standard JSON assertions.
* **Positive:** Scripts are concise, readable, and strictly maintainable (each well under 170 lines, far below the 300-line ceiling).
* **Positive:** Working directory remains clean; transient assets in `staging/` are isolated and never committed to Git.
