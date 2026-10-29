# RuleSentry Scanner

RuleSentry Scanner is a small in-browser tool that finds sensitive text (email addresses, phone
numbers, card numbers, IBANs) and redacts it. You paste text on the left, choose which rules
apply, and copy the redacted result from the right.

The scanning engine is a Rust crate compiled to WebAssembly. The page loads it once and calls it
on every edit, so the scan runs on your machine at typing speed, with no account and no upload.

## Nothing leaves this page

The text you paste is never sent anywhere. There is no server, no analytics and no network call
after the page has loaded. The WebAssembly module runs inside the browser tab, and closing the tab
discards everything.

## How this repo is worked

The work is planned in GitHub issues and carried out in short-lived lanes, one worktree and one
pull request per issue, by people and AI agents under the same rules. Those rules are in
[CLAUDE.md](CLAUDE.md); the reasons behind them are in
[docs/WORKING-AGREEMENT.md](docs/WORKING-AGREEMENT.md); the product is specified in
[docs/SPEC.md](docs/SPEC.md).

## Layout

| Folder | What | Stack |
|---|---|---|
| `scanner/` | The scanning engine, compiled to WebAssembly | Rust, wasm-bindgen, wasm-pack |
| `web/` | The page | Vite, TypeScript |
| `docs/` | Spec and working agreement | Markdown |
| `tools/` | Developer tools (`lane.sh`) | Bash |
| `.github/` | CI and the Pages deploy | GitHub Actions |
| `.claude/` | Project skills for agent sessions | Markdown |

## Running it (not built yet)

```sh
cd scanner && cargo test        # the engine's unit tests
cd web && npm install && npm run dev
```

## License

MIT, see [LICENSE](LICENSE).
