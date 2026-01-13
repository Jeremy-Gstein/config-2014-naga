_default:
  @just --list

# - No Release: git and crates.io -
# just no-release [skip ci/github actions]
no-release message:
  git add .
  git commit -am "{{message}}"
  git push

# - Patch: git and crates.io -
# just patch "small bug fix" [0.0.* release]
patch message:
  git add .
  git commit -am "fix: {{message}} [release]"
  git push

# - Minor Release: git and crates.io -
# just minor "new: features" [0.*.0 release]
minor message:
  git add .
  git commit -am "feat: {{message}} [release]"
  git push

# - Major Release: git and crates.io -
# just major "new: features" [*.0.0 release]
major message:
  git add .
  git commit -am "feat: {{message}} BREAKING CHANGE [release]"
  git push
