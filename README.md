# lib-rand-c

Native random implementation (C entropy) for the `rand` ports.

Part of the `handl-lang` org split (ADR-0128). History is a `git subtree
split` of `kit-canonical/handl-packages/rand-c/`.

## Layout

```text
src/        the .hnl sources
pkg.hnl     dependencies, when any: `{ deps: { <name>: "branch:<ref>" } }`
```

Dependencies: none

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```
