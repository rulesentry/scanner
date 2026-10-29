# Why the working agreement says what it says

[CLAUDE.md](../CLAUDE.md) states the rules. This page gives the reason for each, so that a rule is
changed by arguing with its reason, not by ignoring it.

## Golden rules

- **No secrets in git.** History is forever and this repository is public. A key committed and
  then deleted is still published; the only fix is to rotate it.
- **Warnings are errors.** A warning tolerated once becomes forty, and then the one that matters
  is invisible. Zero is the only count anyone notices changing.
- **Nothing new at the root without asking.** The root is what a visitor reads first. Every file
  there is a claim that it matters to the whole project.
- **One topic per session, then park.** A long session is a slow and confused one: everything said
  earlier is still in play. Parking writes down what the next session needs, so starting fresh
  costs one read.

## GitHub holds project state; the repo holds knowledge

- **State in two places drifts.** A status table in a markdown file and the issue it describes
  disagree within a week, and nobody knows which to believe. Issues hold where work stands; files
  hold what stays true.
- **Every issue belongs to an epic.** An issue without a parent is work nobody chose. The
  "Part of #n" line makes the plan readable from either end.
- **A finding is a comment, not a new issue.** A finding filed as an orphan is never decided: it
  sits in the list looking like a plan. Filing an issue is a decision to do the work.
- **Rulings are marked `## Ruling`.** A decision buried in a thread is re-argued. A fixed heading
  makes every ruling findable with one search.
- **The newest `## Resume` comment is the resume document.** One place to look, always the latest,
  never a file that someone forgot to update.

## Lanes

- **One issue, one worktree, one branch.** Several sessions work the repository at once. A
  worktree gives each its own files and index, so nobody's edit lands in somebody else's commit.
- **Every commit names its issue.** History then answers "why is this here" without archaeology.
- **The gate runs before the push.** The checks CI runs are cheap to run first; a push that CI
  turns red costs a runner and a reviewer's attention.
- **A red PR is closed and cut again.** A red PR fixed by trial pushes costs every other lane a
  rebase, and its history records the guessing instead of the change.
- **Short-lived branches.** A branch that lives a week conflicts with everything. If it cannot
  land in a session or two, the issue was scoped too large; split it.

## Agents

- **A lane agent stays in its worktree.** That is the whole of its isolation; outside it, its
  edits collide with other people's.
- **The orchestrator briefs and does not write lane code.** Someone has to hold the plan. An
  orchestrator that also codes loses the plan to the detail.
- **Minimal testing.** Unit tests and the gate prove the change. More ceremony slows every lane
  and proves the same thing.
- **Nothing about the tooling in issues or PRs.** The record is about the product. The attribution
  lines say who helped; the rest is noise to the next reader.

## Context budget

Every turn resends the whole conversation. Reading a whole file or a whole thread to find one
line makes every later turn slower. Read by range, read the newest comments, and ask a subagent
for its conclusion, not its evidence.
