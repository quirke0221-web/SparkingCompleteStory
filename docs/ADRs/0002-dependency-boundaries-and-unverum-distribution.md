# ADR 0002: Dependency Boundaries & Delegation to Community Mod Management (Unverum)

## Status
Accepted

## Context
Early development attempted to invent custom packaging, manifest formats, and installation tooling from scratch:
1. An ad-hoc manifest schema was authored in `packaging/pakstore.json`.
2. Custom scripts were written to manage mod priority (`~mods\a\CompleteStory_9_P.pak`).
3. Development halted on a false "packaging blocker" (`PACKAGING_BLOCKER.md`) regarding how to distribute and inject the UTOC signature bypass DLLs (`dsound.dll`, `DBSparkingZeroUTOCBypass.asi`).

Audit of the Dragon Ball Sparking! ZERO modding ecosystem revealed that the community has already solved these problems through **Unverum**, **retoc**, **UAssetGUI**, and the **UTOC Signature Bypass**.

## Decision
1. **Zero Wheel-Reinvention:** Complete Story will not maintain custom mod manager manifests, priority sorting scripts, or custom package managers.
2. **Authoritative Tool Boundaries:**
   * Container extraction (`to-legacy`) and packing (`to-zen`): Delegated 100% to **retoc**.
   * Asset serialization and deserialization (`tojson` / `fromjson`): Delegated 100% to **UAssetGUI**.
   * Mod management, installation, profile configuration, and priority renaming: Delegated 100% to **Unverum**.
   * Signature bypass loading: Handled by Unverum's built-in UTOC bypass setup.
3. **Distribution Standards:** Release archives in `dist/` must contain strictly the game container assets (`CompleteStory_P.pak`, `CompleteStory_P.utoc`, `CompleteStory_P.ucas`) at the zip root.
4. **Strict Bypass Bundling Ban:** Mod release packages must **never** bundle third-party bypass or injection binaries (`dsound.dll`, `.asi`).

## Consequences
* **Positive:** Eliminates hundreds of lines of fragile, ad-hoc packaging scripts and custom manifest schemas.
* **Positive:** Unverum users get standard, conflict-free mod management and automatic load-ordering.
* **Positive:** Complete Story remains legally and ethically pristine by distributing only custom game mod data assets.
