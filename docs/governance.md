# Governance

This document outlines the governance model for Rome. This includes the contributor model, code review, merging, and the consequences and process for code of conduct violations.

## Member Roles

All members must follow the [code of conduct](https://github.com/romejs/rome/blob/master/.github/CODE_OF_CONDUCT.md). Consequences for member violations are detailed in [Moderation](#moderation).

### Core Contributor

Core Contributors are those who have proved themselves as an active contributor by being involved in project management or by submitting high quality pull requests.

There are no expectations around activity. There may be a time where contributor privileges are restricted or removed for inactivity. However you will not lose org membership.

New contributors will be decided based on a general consensus by the existing core contributors.

### Admin

The amount of admins will be kept deliberately small. An admin has additional privileges over a core contributor including:

- Access to the [@romejsdev Twitter account](https://twitter.com/romejsdev)
- Administration privileges on the [Rome GitHub org](https://github.com/romejs)
- Administration privileges on the [Rome Discord server](https://github.com/romejs)
- Publish access to the [`rome` npm package](https://www.npmjs.com/package/rome)
- Domain registrar and DNS access to all `romejs.*` domains
- Administration access to the `romejs.dev` Netlify account
- Ability to decide on moderation decisions involving core contributors
- Access to the `*@romejs.dev` email address

The separation of Core Contributor and Admin allows for a minimum access of privileges.

New admins will be decided based on a general consensus by the existing admins.

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

- All code needs to go through pull requests and must pass status checks before being merged. If a PR is merged that breaks `master` due to the branch not being up to date, then it should either be reverted or a quick fix merged as a separate PR.
- If a PR is against code that you have previously committed and is either small changes, bug fixes, or refactors, then you're free to merge it without any review. However if you don't feel confident in your changes then wait for approval from another core contributor.
- If you are an owner of a particular area you are free to merge it without any review despite PR size.
- If an area involves multiple owners then it's up to .
- If after a PR is merged and there are comments or suggestions after the fact, allow yourself time to address them in a follow up PR. If you don't think you will be able to respond in a reasonable timeframe then create an issue to track.
- You are welcome to use the `rome` repo for your WIP branches. However you should prefix branches with your username. ie. `git branch sebmck/feature`. Branches not involved in an active PR will be regularly pruned.
- If you are adding a new feature then ensure that it has been discussed or approved on GitHub or Discord.

## Moderation

Users found to be in violation of the projects [code of conduct](https://github.com/romejs/rome/blob/master/.github/CODE_OF_CONDUCT.md) will be:

- Banned from the GitHub org and Discord server
- Have their contributor status revoked (if applicable)
- Have admin privileges revoked (if applicable)
- Action listed in [`moderation.md`](https://github.com/romejs/rome/blob/master/docs/governance.md). There may be some scenarios where discretion is required and some details omitted to protect individuals.

Code of conduct violations can be reported to <conduct@romejs.dev> which is listed in the [code of conduct](https://github.com/romejs/rome/blob/master/.github/CODE_OF_CONDUCT.md). This email address is monitored by admins.

Moderation decisions and code of conduct violations will be discussed amongst the core contributors in private. Exceptions are if a possible violation involves a core contributor or admin:

 - Core contributor: Adjudicated amongst admins and other core contributors.
 - Admin: Adjudicated amongst the remaining admins and core contributors.

## OpenCollective fund allocation

- Usage of funds has yet to be decided.

## Governance changes

- Future changes to this document will require approval from all core contributors.
