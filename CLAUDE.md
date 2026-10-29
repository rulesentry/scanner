# RuleSentry Scanner: working agreement

This file states the rules. Why each one exists is in
[docs/WORKING-AGREEMENT.md](docs/WORKING-AGREEMENT.md); read it there before changing or relaxing
a rule here. The product is specified in [docs/SPEC.md](docs/SPEC.md).

## Golden rules

1. **No secrets in git, ever.** `.env` is never committed. This repository is public.
2. **Warnings are errors.** `cargo clippy -- -D warnings`, a clean `npm run lint`. The count
   stays at zero.
3. **Never add files to the repository root without asking.** Code, config, scripts and docs go
   in the folder they belong to (see the layout in [README.md](README.md)).
4. **One topic per session, then park.** When the lane lands or the topic changes, write the
   `## Resume` comment (`/park`) and end the session. The next one starts with `/resume`.

## GitHub holds project state; the repo holds knowledge

1. **An epic is an issue labelled `epic`.** Every other issue is linked to its epic by a first
   line `Part of #n`, written when the issue is filed. If no epic fits, ask; never invent one.
2. **A finding is a comment on the issue in hand, never a new orphan.** It earns its own issue
   when someone decides to do it. Filing is a decision, not a note.
3. **A ruling is a comment that starts with `## Ruling`.** One decision per comment, with the date
   and the reason. Search for the heading before re-arguing anything.
4. **The newest `## Resume` comment on the epic is the resume document.** Where the work stands,
   what is proven, what is not built, the next prompt. No `STATUS.md`, no resume file, no status
   table in markdown.
5. **The epic carrying the orchestrator session is labelled `orchestrator`.** One at a time.
6. **Labels:** `epic`, `orchestrator`, `task`, `review`, `ruling-needed`. An issue waiting on a
   decision carries `ruling-needed` until a `## Ruling` comment answers it.

## Lanes: one issue, one worktree, one PR

```sh
tools/lane.sh new 2 "WASM scanner crate"   # ../scanner-lanes/2, branch lane/2, pushed;
                                           # comments "Picked up in lane/2" on #2
cd ../scanner-lanes/2                      # start the session HERE
git fetch && git rebase origin/main        # daily
tools/lane.sh land                         # gate, rebase, push, PR "#2 ...", auto-merge
tools/lane.sh list                         # every lane and its last commit
tools/lane.sh remove 2                     # once the PR has merged
```

- **Start the session in the worktree**, not in the main checkout. The main checkout is for
  reading and for `git pull --ff-only`; nothing is committed there.
- **Every commit subject carries `#<issue>`**: `feat(scanner): #2 — email and phone rules`.
  Subject at most 72 characters, detail in the body. Name the paths:
  `git commit -m "..." -- <paths>`.
- **The PR** is opened by `land`: its title starts with `#<issue>`, its body says
  `Closes #<issue>`, and auto-merge is enabled. Merging closes the issue.
- **Branches are short-lived.** Cut from `origin/main`, rebased daily, removed when merged. A lane
  that cannot land in a session or two was scoped wrong; split the issue.

### The gate

`tools/lane.sh land` runs it before pushing and **refuses to push on red**. It runs only the parts
that exist:

| Part | Checks |
|---|---|
| `scanner/` | `cargo test`, `cargo clippy -- -D warnings` |
| `web/` | `npm run lint`, `npm run build` |

CI (`.github/workflows/ci.yml`) runs the same checks plus `wasm-pack build`, and deploys `web/dist`
to GitHub Pages from `main`.

- **A red PR is closed and cut again**, never fixed on the branch by trial pushes.
- **The gate is never a substitute for CI.** Nothing local may let a required check pass.

## Agents

- **A lane agent works inside its worktree only.** It never edits or commits in the main checkout
  or in another lane.
- **The orchestrator briefs; it never writes lane code.** It holds the epic, writes the rulings,
  cuts the lanes and writes each lane's brief. Questions that need a decision go to John, not to
  a guess.
- **A brief names**: the issue, the worktree, the paths the lane may touch, the done-when, and
  what is out of scope.
- **Minimal testing**: unit tests while iterating (filtered), the gate once at the end. No
  screenshots or browser walks unless the issue asks for them.
- **Nothing about the model or the tooling** in an issue, a comment or a PR body, beyond the
  attribution lines.
- **An agent never reviews its own work with another agent.** Verification is a test that was red
  and is now green.
- Logs and scratch files go under the lane's own worktree (`.lane-logs/`, ignored), never a shared
  path.

## Context budget: a slow session is a full one

Every turn resends the whole conversation.

1. **Read by range, not whole.** `grep -n` first, then `sed -n 'a,bp'`. Never `cat` a file over
   about 150 lines, or any log. Long output goes to a file and is grepped.
2. **Issue reads are narrow.** `gh issue list --json number,title,labels`;
   `gh issue view <n> --comments | tail -n 60` for the newest comments. Open the full view only
   for the issue in hand.
3. **Subagents return conclusions, not evidence.** Bulky findings go to a file; the result is a
   short summary and the path.
4. **Write short, because the next session reads it.** A closing comment says what changed, the
   proof and what is not done, and links the CI run instead of pasting output.

## Skills

- `/park` writes the `## Resume` comment on the epic (or the issue in hand).
- `/resume` reads the newest `## Resume` comment and prints the next-session prompt.

Both live in `.claude/skills/`.
