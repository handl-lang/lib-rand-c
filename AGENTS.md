# lib-rand-c

Native random implementation (C entropy) for the `rand` ports. Pure
`.hnl`.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the rand surface |
| `src/entropy.hnl` | the C-entropy backend |
| `pkg.hnl` | manifest — `modules: ["rand-c"]` |
| `scripts/hnl-gate.sh`, `ci.sh` | the gate / bupha entry |

## Contract

- Implements the `rand` contract (`lib-rand`) over C entropy sources.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
