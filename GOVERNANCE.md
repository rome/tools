# Governance

This document outlines the governance model for Rome. This includes the contributor model, code review, merging, and the consequences and process for Code of Conduct violations.

## Member Roles

All members must follow the [Code of Conduct](CODE_OF_CONDUCT.md). Consequences for member violations are detailed in [Moderation](#moderation).

### Core Contributor

Core Contributors are those with a history of consistent contributions, including but not limited to pull requests, project management, or support. These privileges include:

- Push access to the [Rome GitHub org](https://github.com/romejs), this includes all repos
- Contributor status on the [Rome Discord server](https://github.com/romejs)
- Ability to [vote](#voting) on project decisions

There are no expectations around activity once someone becomes a core contributor. Inactive contributors may have voting rights removed however will always retain their status. Inactivity requirements will be specified in a later governance change.

#### Induction

Core contributors may either be nominated by another core contributor or can be self nominated. This can be done via message in the [`#contributors` Discord channel](https://discord.com/channels/678763474494423051/678763474930761739) or by privately messaging a director.

New core contributors will be decided based on a [vote](#voting) conducted in the [`#contributors-private` Discord channel](https://discord.com/channels/678763474494423051/712849311985041420).

In the event of a rejection the nominated person will be privately given the requirements they have not met. Details of the discussion such as the names of those who objected will not be disclosed.

### Director

Directors have additional privileges over core contributors. Directors control and maintain sensitive project assets, and act as tiebreakers in the event of disagreements. These additional privileges include:

- Access to the [@romejsdev Twitter account](https://twitter.com/romejsdev)
- Administration privileges on the [Rome GitHub org](https://github.com/romejs)
- Administration privileges on the [Rome Discord server](https://github.com/romejs)
- Publish access to the [`rome` npm package](https://www.npmjs.com/package/rome)
- Domain registrar and DNS access to all `romejs.*` domains
- Administration access to the `romejs.dev` Netlify account
- Ability to initiate a [vote](#voting)
- Ability to veto [votes](#voting) and resolve voting deadlocks
- Define project direction and planning
- Ability to decide on moderation decisions
- Access to the `*@romejs.dev` email address

#### Induction

New directors will be added based on a unanimous vote by the existing directors. In the event that someone is unreachable then the decision will be deferred. Discussion and approval will be done in private. Directors cannot be self-nominated. Inducted directors must have already be a core contributor.

A [vote](#voting) will be conducted with core contributors to gauge general sentiment, however final decision will be reserved to the existing directors.

### Owner

Certain parts of the codebase can be owned by one or more people. This process is informal and inclusion could be a result of substantial contribution or delegation by other members. It's the responsibility of a core contributor to identify the relevant owners and ensure there's an understanding when it comes to code review.

## Current Members

Members listed in alphabetical order.

### Directors

- [Sebastian McKenzie @sebmck](https://github.com/sebmck)

### Core Contributors

- [Eduardo Lopes @EduardoLopes](https://github.com/EduardoLopes)
- [Florent Cailhol @ooflorent](https://github.com/ooflorent)
- [Jamie Kyle @jamiebuilds](https://github.com/jamiebuilds)
- [Kevin Kelbie @Kelbie](https://github.com/Kelbie)
- [Olivier Dusabimana @diokey](https://github.com/diokey)
- [Paul Bouchon @bitpshr](https://github.com/bitpshr)
- [Victor Hom @VictorHom](https://github.com/VictorHom)

## Project direction and planning

Project direction and planning is a shared responsibility amongst members. Directors are responsible for dictating high level goals and scope of the project that should be adhered to.

## Voting

Certain project decisions require a vote. These include:

- Governance changes: simple majority (over 50%) conducted via GitHub PR approval.
- Core contributor membership: overwhelming majority (over 70%) conducted in the `#contributors-private` Discord channel.

A director may initiate a vote for any unlisted project decision. Core contributors can request a vote by contacting a director.

### Rules

- Members may abstain from a vote.
- Members who do not vote within 7 days will automatically abstain.
- Directors may reduce the 7 day automatic abstain for urgent decisions.
- Directors reserve the right to veto approval with a publicly disclosed reason.

## Code review

We have a fairly liberal approach to code review and merging. We value quick iteration and low development friction. This comes with great responsibility. Reverting code is easy so landing code should be just as easy. Because of this, Rome will have discrete releases rather than rolling releases that are automatically published.

- If you are an owner of a particular area you are free to merge it without any review despite PR size.
- If after a PR is merged and there are comments or suggestions after the fact, allow yourself time to address them in a follow up PR. If you don't think you will be able to respond in a reasonable timeframe then create an issue to track.
- Ensure that PR summary is detailed listing steps you took to verify, the rationale, and relevant issues and people involved in any prior discussion.
- Ensure that PRs contain adequate tests and code comments for a future contributor to derive intent and modify your code safely.
- You are welcome to use the `rome` repo for your WIP branches. However you should prefix branches with your username. ie. `git branch sebmck/feature`. Branches not involved in an active PR will be regularly pruned.
- If you are adding a new feature then ensure that it has been discussed or approved on GitHub or Discord.
- If necessary, identify potential owners for PR review and approval.
- All code needs to go through Pull Requests (PR) and must pass status checks before being merged. If a PR is merged that breaks `master` due to the branch not being up to date, then it should either be reverted or a quick fix merged as a separate PR.
- If a PR is against code that you have previously committed and is either small changes, bug fixes, or refactors, then you're free to merge it without any review. However if you don't feel confident in your changes then you can wait for approval from another core contributor.

## Moderation

Users found to be in violation of the projects [Code of Conduct](./CODE_OF_CONDUCT.md) will be:

- Banned from the GitHub org and Discord server
- Have their contributor status revoked (if applicable)
- Action listed in [`MODERATION.md`](./MODERATION.md). There may be some scenarios where discretion is required and some details omitted to protect an individual's privacy.

This is a one-strike policy.

Code of Conduct violations can be reported to <conduct@romejs.dev> which is listed in the [Code of Conduct](./CODE_OF_CONDUCT.md). This email address is monitored by directors. Alternatively email addresses for each directror is included in the Code of Conduct for private disclosements in the event of a report involving a directror.

Moderation decisions and Code of Conduct violations will be reviewed by the directors in private. If a violation involves a directror then it will be decided amongst the remaining directors. Review can be delegated to core contributors.

## OpenCollective fund allocation

- Funds will be allocated for project-specific services such as domain registration and website hosting.
- Other usage of funds has yet to be decided.
- Expenses will be approved by project directors.
