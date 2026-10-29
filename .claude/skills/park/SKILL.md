---
name: park
description: End a session by writing one "## Resume" comment on the epic (or the issue named) with gh, so the next session starts from it. Use when a lane lands, the topic changes, or the session is ending. "/park" parks on the epic labelled orchestrator; "/park 3" parks on issue #3. Never writes a file.
---

# Park

Write ONE comment on GitHub that lets a fresh session pick up exactly here. The newest `## Resume`
comment is the resume document; nothing goes in a file.

## Which issue

- `/park <n>`: issue #n.
- `/park` with no number: the open issue labelled `orchestrator`:
  `gh issue list --label orchestrator --state open --json number --jq '.[0].number'`.
  If there is none, ask; never guess.

## Gather (read narrowly)

```sh
git rev-parse --show-toplevel                       # the worktree
git rev-parse --abbrev-ref HEAD; git rev-parse --short HEAD
git status --short | head -n 20                     # anything uncommitted is "Not built"
tools/lane.sh list
gh pr list --state open --json number,title,headRefName
```

## Write the comment

Exactly this shape, short lines, no tooling talk. Write it to a temp file and post with
`gh issue comment <n> --body-file <file>`.

```markdown
## Resume

**Worktree:** <absolute path of the worktree the next session starts in>
**Branch:** <branch> @ <short sha> (<pushed | not pushed | PR #n open | merged>)

**Proven:** <what is done and how it was proven: tests, the gate, a CI run link>

**Not built:** <what was planned and is not done; uncommitted work, by path>

**Open question:** <the one decision that blocks progress, for John; "none" if none>

**Next session prompt:**
> Start in: <absolute path>
> <two to five sentences: what to read first, what to do, what done looks like>
```

## Rules

- One comment per park. Do not edit an older `## Resume`; the newest one wins.
- Findings that are not about resuming go in their own comment on the issue in hand, never a new
  issue.
- Print the comment's URL when done. That is the whole output.
