---
name: resume
description: Start a session where the last one parked. Reads the newest "## Resume" comment on the epic (or the issue named) with gh and prints its next-session prompt, the directory to start in, and whether that worktree and branch still exist. "/resume" reads the open epic labelled orchestrator; "/resume 3" reads issue #3. Read-only.
---

# Resume

Read the newest `## Resume` comment and act on its prompt. Read-only: this skill changes nothing.

## Which issue

- `/resume <n>`: issue #n.
- `/resume` with no number: the open issue labelled `orchestrator`:
  `gh issue list --label orchestrator --state open --json number --jq '.[0].number'`.
  If there is none, list the open epics (`gh issue list --label epic --json number,title`) and ask.

## Read narrowly

Do not dump the whole thread. Pull only the newest resume comment:

```sh
gh issue view <n> --json comments \
  --jq '[.comments[] | select(.body | startswith("## Resume"))] | last | .body'
```

If that is empty, fall back to `gh issue view <n> --comments | tail -n 60`.

## Check the ground

From the comment, take the `Start in:` path and the `Branch: <branch> @ <sha>` line, then:

```sh
test -d <path> && git -C <path> rev-parse --abbrev-ref HEAD --short HEAD
gh pr list --head <branch> --state all --json number,state
```

## Report

Print, in this order and nothing more:

1. The issue number and title.
2. **Start in:** the path, and whether it exists. If the session is not already there, say so:
   the session must start in that directory, not `cd` into it part-way.
3. **Branch:** recorded sha vs current sha; PR state.
4. **Open question**, if any. If it is not answered by a later `## Ruling` comment, stop and ask
   John before doing anything else.
5. The **Next session prompt**, verbatim.
