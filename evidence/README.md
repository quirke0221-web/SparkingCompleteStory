# Evidence Index

This directory contains small, sanitized primary evidence. It does not contain full game assets, saves, object dumps, AES material, or cooked distributions.

- `runtime/v0.7-sanitized.log`: exact Complete Story/UE4SS lifecycle lines from the isolated v0.7 run.
- `runtime/v0.7-full-sanitized.log`: full retained v0.7 UE4SS log with local paths, account identifiers, and 256-bit values redacted.
- `runtime/UE4SS-settings-test.ini` and `runtime/v0.7-isolation-mods.txt`: exact safe test configuration and isolation record.
- `object-dump/targeted-symbols.txt`: exact class/property/function names and source line numbers from the local dump.
- `object-dump/episode-battle-symbols.txt`: broader targeted native/reflection output used during analysis.
- `assets/structural-summary.md`: derived registry, DLC experiment, and event-reference facts.
- `packages/SHA256SUMS.txt`: hashes of excluded local version ZIPs.
- `reproduction/required-assets.txt`: exact packages a legitimate owner must extract.

The local full object dump was about 202 MB and is excluded. Local JSON/UAsset exports are also excluded because they reproduce game-owned serialized data. The commands and identifiers here let another owner recreate the evidence.
