# Governance

This document outlines the governance model for Rome. This includes the contributor model, code review, merging, and the consequences and process for Code of Conduct violations.

## Member Roles

All members must follow the [Code of Conduct](CODE_OF_CONDUCT.md). Consequences for member violations are detailed in [Moderation](#moderation).

### Core Contributor

Core Contributors are those who have a history of consistent contributions, whether this pull requests, project management, or support.

There are no expectations around activity. There may be a time where contributor privileges are restricted or removed for inactivity. Once core contributor status has been granted you will not lose org membership.

New contributors will be decided based on a unanimous vote by the existing core contributors.

### Admin

An admin has additional privileges over a core contributor. They serve to control and maintain sensitive project assets, and to act as tiebreakers in the event of disagreements. These additional privileges include:

- Access to the [@romejsdev Twitter account](https://twitter.com/romejsdev)
- Administration privileges on the [Rome GitHub org](https://github.com/romejs)
- Administration privileges on the [Rome Discord server](https://github.com/romejs)
- Publish access to the [`rome` npm package](https://www.npmjs.com/package/rome)
- Domain registrar and DNS access to all `romejs.*` domains
- Administration access to the `romejs.dev` Netlify account
- Ability to decide on moderation decisions involving core contributors
- Access to the `*@romejs.dev` email address

The separation of Core Contributor and Admin allows for a minimum access of privileges.

New admins will be decided based on a unanimous vote by the existing admins.

### Owners

Certain parts of the codebase can be owned by one or more people. This process is informal and inclusion could be a result of substantial contribution or delegation by other members. It's the responsibility of a core contributor to identify the relevant owners and ensure there's an understanding when it comes to code review.

## Current Members

Members listed in alphabetical order.

### Admins

- [Sebastian McKenzie @sebmck](https://github.com/sebmck)

### Core Contributors

- [Eduardo Lopes @EduardoLopes](https://github.com/EduardoLopes)
- [Florent Cailhol @ooflorent](https://github.com/ooflorent)
- [Jamie Kyle @jamiebuilds](https://github.com/jamiebuilds)
- [Kevin Kelbie](https://github.com/KevinKelbie)
- [Olivier Dusabimana @diokey](https://github.com/diokey)
- [Paul Bouchon @bitpshr](https://github.com/bitpshr)
- [Victor Hom @VictorHom](https://github.com/VictorHom)

## Code review and merging

- All code needs to go through Pull Requests (PR) and must pass status checks before being merged. If a PR is merged that breaks `master` due to the branch not being up to date, then it should either be reverted or a quick fix merged as a separate PR.
- If a PR is against code that you have previously committed and is either small changes, bug fixes, or refactors, then you're free to merge it without any review. However if you don't feel confident in your changes then you can wait for approval from another core contributor.
- If you are an owner of a particular area you are free to merge it without any review despite PR size.
- If after a PR is merged and there are comments or suggestions after the fact, allow yourself time to address them in a follow up PR. If you don't think you will be able to respond in a reasonable timeframe then create an issue to track.
- If necessary, identity potential owners for PR review and approval.
- Ensure that PR summary is detailed listing steps you took to verify, the rationale, and relevant issues and people involved in any prior discussion.
- Ensure that PRs contain adequate tests and code comments for a future contributor to derive intent and modify the code safely.
- You are welcome to use the `rome` repo for your WIP branches. However you should prefix branches with your username. ie. `git branch sebmck/feature`. Branches not involved in an active PR will be regularly pruned.
- If you are adding a new feature then ensure that it has been discussed or approved on GitHub or Discord.

## Moderation

Users found to be in violation of the projects [Code of Conduct](./CODE_OF_CONDUCT.md) will be:

- Banned from the GitHub org and Discord server
- Have their contributor status revoked (if applicable)
- Have admin privileges revoked (if applicable)
- Action listed in [`MODERATION.md`](./MODERATION.md). There may be some scenarios where discretion is required and some details omitted to protect and individual's privacy.

This is a one-strike policy.

Code of Conduct violations can be reported to <conduct@romejs.dev> which is listed in the [Code of Conduct](./CODE_OF_CONDUCT.md). This email address is monitored by admins. Alternatively email addresses for each admin is included in the Code of Conduct for private disclosements in the event of a report involving an admin.

Moderation decisions and Code of Conduct violations will be discussed amongst the core contributors in private. Exceptions are if a possible violation involves a core contributor or admin:

 - Core contributor: Adjudicated amongst admins and other core contributors.
 - Admin: Adjudicated amongst the remaining admins and core contributors.

## OpenCollective fund allocation

- Usage of funds has yet to be decided.

## Governance changes

- Future changes to this document will require approval from all core contributors.
