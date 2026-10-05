# Evidence Index

This directory contains small, sanitized primary evidence. It does not contain full game assets, saves, object dumps, AES material, or cooked distributions.

- `runtime/v0.7-sanitized.log`: exact Complete Story/UE4SS lifecycle lines from the isolated v0.7 run.
- `object-dump/targeted-symbols.txt`: exact class/property/function names and source line numbers from the local dump.
- `assets/structural-summary.md`: derived registry, DLC experiment, and event-reference facts.
- `packages/SHA256SUMS.txt`: hashes of excluded local version ZIPs.
- `reproduction/required-assets.txt`: exact packages a legitimate owner must extract.

The local full object dump was about 202 MB and is excluded. Local JSON/UAsset exports are also excluded because they reproduce game-owned serialized data. The commands and identifiers here let another owner recreate the evidence.
