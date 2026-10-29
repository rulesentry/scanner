#!/usr/bin/env bash
# lane.sh: one issue, one worktree, one PR. See CLAUDE.md "Lanes".
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  tools/lane.sh new <issue> "<title>"   cut ../scanner-lanes/<issue> on lane/<issue>, push, comment on the issue
  tools/lane.sh land                    (in a lane) gate, rebase on origin/main, push, open an auto-merge PR
  tools/lane.sh remove <issue>          remove the worktree and local branch once the PR has merged
  tools/lane.sh list                    list lanes with their last commit
USAGE
}

die() { echo "lane: $*" >&2; exit 1; }

main_root() { dirname "$(git rev-parse --path-format=absolute --git-common-dir)"; }
lanes_dir() { echo "$(dirname "$(main_root)")/scanner-lanes"; }
issue_ok()  { [[ "${1:-}" =~ ^[0-9]+$ ]] || die "issue must be a number, got '${1:-}'"; }

cmd_new() {
  local issue="${1:-}" title="${2:-}"
  issue_ok "$issue"
  [[ -n "$title" ]] || die "a title is required"
  local dir branch="lane/$issue"
  dir="$(lanes_dir)/$issue"
  [[ -e "$dir" ]] && die "$dir already exists"
  gh issue view "$issue" --json number >/dev/null || die "issue #$issue not found"
  git fetch --quiet origin
  mkdir -p "$(lanes_dir)"
  git worktree add -b "$branch" "$dir" origin/main
  git -C "$dir" push --set-upstream origin "$branch"
  gh issue comment "$issue" --body "Picked up in $branch"
  echo "lane: $title ready. Start the session in $dir"
}

current_issue() {
  local branch
  branch="$(git rev-parse --abbrev-ref HEAD)"
  [[ "$branch" =~ ^lane/([0-9]+)$ ]] || die "not on a lane/<issue> branch (on '$branch')"
  echo "${BASH_REMATCH[1]}"
}

run_gate() {
  local root="$1" ran=0
  if [[ -f "$root/scanner/Cargo.toml" ]]; then
    echo "gate: scanner/"
    (cd "$root/scanner" && cargo test --quiet && cargo clippy --quiet -- -D warnings) || return 1
    ran=1
  else
    echo "gate: scanner/ absent, skipped"
  fi
  if [[ -f "$root/web/package.json" ]]; then
    echo "gate: web/"
    (cd "$root/web" && npm run --silent lint && npm run --silent build) || return 1
    ran=1
  else
    echo "gate: web/ absent, skipped"
  fi
  [[ $ran -eq 1 ]] || echo "gate: nothing to build; docs and tooling only"
  return 0
}

cmd_land() {
  local issue root branch
  issue="$(current_issue)"
  root="$(git rev-parse --show-toplevel)"
  branch="lane/$issue"
  [[ "$root" != "$(main_root)" ]] || die "land runs in a lane worktree, not the main checkout"
  [[ -z "$(git status --porcelain --untracked-files=no)" ]] || die "uncommitted changes; commit them first"
  local logdir="$root/.lane-logs"
  mkdir -p "$logdir"

  git fetch --quiet origin
  git rebase origin/main || die "rebase failed; resolve it, then land again"
  [[ "$(git rev-list --count origin/main..HEAD)" -gt 0 ]] || die "nothing to land"

  if ! run_gate "$root" 2>&1 | tee "$logdir/gate.log"; then
    die "gate RED: not pushing (log: $logdir/gate.log)"
  fi

  git push --force-with-lease origin "$branch"

  if gh pr view "$branch" --json number >/dev/null 2>&1; then
    echo "lane: PR exists, updated by the push"
  else
    local subject
    subject="$(gh issue view "$issue" --json title --jq .title)"
    gh pr create --base main --head "$branch" \
      --title "#$issue $subject" \
      --body "$(printf 'Closes #%s\n\n## Commits\n\n%s\n' "$issue" \
        "$(git log --format='- %s' origin/main..HEAD)")"
  fi
  gh pr merge "$branch" --auto --squash || echo "lane: auto-merge not enabled; merge by hand once green"
}

cmd_remove() {
  local issue="${1:-}"
  issue_ok "$issue"
  local dir branch="lane/$issue"
  dir="$(lanes_dir)/$issue"
  [[ -d "$dir" ]] || die "no lane at $dir"
  local state
  state="$(gh pr view "$branch" --json state --jq .state 2>/dev/null || echo NONE)"
  [[ "$state" == "MERGED" ]] || die "PR for $branch is $state, not MERGED; refusing"
  git worktree remove "$dir"
  git branch -D "$branch"
  git push origin --delete "$branch" 2>/dev/null || true
  echo "lane: removed $dir"
}

cmd_list() {
  git worktree list --porcelain | awk '
    /^worktree /{wt=substr($0,10)} /^branch /{br=substr($0,8); sub("refs/heads/","",br)
      if (br ~ /^lane\//) print br "\t" wt}' |
  while IFS=$'\t' read -r br wt; do
    printf '%-12s %s  %s\n' "$br" "$wt" "$(git -C "$wt" log -1 --format='%h %s')"
  done
}

case "${1:-}" in
  new)    shift; cmd_new "$@" ;;
  land)   shift; cmd_land "$@" ;;
  remove) shift; cmd_remove "$@" ;;
  list)   shift; cmd_list "$@" ;;
  -h|--help|help|"") usage ;;
  *) usage >&2; exit 2 ;;
esac
