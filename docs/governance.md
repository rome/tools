# Governance

Rome is still an early project. This document serves as rough guidelines for merging code and decision making for core contributors. Core contributors are those who have been added to the Rome org and have triage and push privileges. This document is not intended to be strict and these rules will evolve over time.

These rules are loose and intend to give core contributors autonomy and discretion. Code can be easily reverted and we should value rapid change and refactoring that would otherwise be tedious and bureaucratic.

## What is a core contributor?

- Someone who has proved themselves as an active contributor by being involved in project management or by submitting high quality pull requests.
- There is no hard constraints. As the project evolves the bar for inclusion may be raised.
- New contributors will be decided based on a general consensus by the existing core contributors.

## Expectations

- Follow the [code of conduct](https://github.com/romejs/rome/blob/master/.github/CODE_OF_CONDUCT.md). Violations will result in your contributor status being revoked.
- Don't abuse your repo access to push malicious code, or to edit or delete other peoples comments outside of moderation.
- There are no expectations around activity. There may be a time where contributor privileges are restricted for inactivity however you will not lose org membership. Once you are added to the org you will be able t.

## Code review and merging

- All code needs to go through pull requests and must pass status checks before being merged. If a PR is merged that breaks `master` due to conflicts then submit a new PR to fix it.
- If a PR is against code that you have previously committed and is either small changes, bug fixes, or refactors, then you're free to merge it without any review. However if you don't feel confident in your changes then wait for approval from another core contributor.
- If you have shown yourself to be an owner of a particular area, whether it's by substantial contribution or prior discussion, you are free to merge it without any review despite PR size.
- Make sure you have a consensus on changes. If you are adding a new feature then ensure that it has been discussed or approved on GitHub or Discord.
- If code is merged without review and there are comments and suggestions after the fact, allow yourself time to address them in a follow up PR. If you don't think you will be able to respond in a reasonable timeframe then create an issue to track.
