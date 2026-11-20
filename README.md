# lib-rand-c

Native random implementation (C entropy) for the `rand` ports.

## Layout

```text
src/index.hnl   the .hnl sources
src/entropy.hnl the C-entropy backend
pkg.hnl         package manifest — modules, deps, description, license
ci.sh           bupha entry → scripts/hnl-gate.sh
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
