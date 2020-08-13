# Governance

This document outlines the governance model for Rome. This includes the contributor model, code review, merging, and the consequences and process for Code of Conduct violations.

## Member Roles

All members must follow the [Code of Conduct](CODE_OF_CONDUCT.md). Consequences for member violations are detailed in [Moderation](#moderation).

### Core Contributor

Core Contributors are those with a history of consistent contributions, including but not limited to pull requests, project management, or support. These privileges include:

- Push access to the [Rome GitHub org](https://github.com/romefrontend), this includes all repos
- Contributor status on the [Rome Discord server](https://discord.gg/rome)
- Ability to [vote](#voting) on project decisions

There are no expectations around activity once someone becomes a core contributor. Inactive contributors may have voting rights removed however will always retain their status. Inactivity requirements will be specified in a later governance change.

#### Induction

Core contributors may either be nominated by another core contributor or can be self nominated. This can be done via message in the [`#contributors` Discord channel](https://discord.com/channels/678763474494423051/678763474930761739) or by privately messaging a steward.

New core contributors will be decided based on a [vote](#voting) conducted by privately messaging a steward.

In the event of a rejection the nominated person will be privately given the requirements they have not met. Details of the discussion such as the names of those who objected will not be disclosed.

### Steward

Stewards have additional privileges over core contributors. Stewards control and maintain sensitive project assets, and act as tiebreakers in the event of disagreements. These additional privileges include:

- Access to the [@romefrontend Twitter account](https://twitter.com/romefrontend)
- Administration privileges on the [Rome GitHub org](https://github.com/romefrontend)
- Administration privileges on the [Rome Discord server](https://discord.gg/rome)
- Publish access to the [`rome` npm package](https://www.npmjs.com/package/rome)
- Domain registrar and DNS access to all `romefrontend.*` domains
- Administration access to the `romefrontend.dev` Netlify account
- Ability to initiate a [vote](#voting)
- Ability to veto [votes](#voting) and resolve voting deadlocks
- Define project direction and planning
- Ability to decide on moderation decisions
- Access to the `*@romefrontend.dev` email address

#### Induction

New stewards will be added based on a unanimous vote by the existing stewards. In the event that someone is unreachable then the decision will be deferred. Discussion and approval will be done in private. Stewards cannot be self-nominated. Inducted stewards must have already be a core contributor.

A [vote](#voting) will be conducted with core contributors to gauge general sentiment, however final decision will be reserved to the existing stewards.

### Owner

Certain parts of the codebase can be owned by one or more people. This process is informal and inclusion could be a result of substantial contribution or delegation by other members. It's the responsibility of a core contributor to identify the relevant owners and ensure there's an understanding when it comes to code review.

## Current Members

Members listed in alphabetical order.

### Stewards

- [Sebastian McKenzie @sebmck](https://github.com/sebmck)

### Core Contributors

- [Eduardo Lopes @EduardoLopes](https://github.com/EduardoLopes)
- [Emanuele Stoppa @ematipico](https://github.com/ematipico)
- [Florent Cailhol @ooflorent](https://github.com/ooflorent)
- [Jamie Kyle @jamiebuilds](https://github.com/jamiebuilds)
- [Kevin Kelbie @Kelbie](https://github.com/Kelbie)
- [Olivier Dusabimana @diokey](https://github.com/diokey)
- [Paul Bouchon @bitpshr](https://github.com/bitpshr)
- [Victor Hom @VictorHom](https://github.com/VictorHom)
- [Yasser Elassal @yassere](https://github.com/yassere)

## Project direction and planning

Project direction and planning is a shared responsibility amongst members. Stewards are responsible for dictating high level goals and scope of the project that should be adhered to.

## Voting

Certain project decisions require a vote. These include:

- Governance changes: simple majority (over 50%) conducted via GitHub PR approval.
- Core contributor membership: overwhelming majority (over 70%) conducted by privately messaging a steward. Funneling both assenting and dissenting votes directly through stewards allows for anonymity when discussing the merits of a potential contributor.

A steward may initiate a vote for any unlisted project decision. Core contributors can request a vote by contacting a steward.

### Rules

- Members may abstain from a vote.
- Members who do not vote within 7 days will automatically abstain.
- Stewards may reduce the 7 day automatic abstain for urgent decisions.
- Stewards reserve the right to veto approval with a publicly disclosed reason.

## Code review

We have a fairly liberal approach to code review and merging. We value quick iteration and low development friction. This comes with great responsibility. Reverting code is easy so landing code should be just as easy. Because of this, Rome will have discrete releases rather than rolling releases that are automatically published.

- If you are an owner of a particular area you are free to merge it without any review despite PR size.
- If after a PR is merged and there are comments or suggestions after the fact, allow yourself time to address them in a follow up PR. If you don't think you will be able to respond in a reasonable timeframe then create an issue to track.
- Ensure that PR summary is detailed listing steps you took to verify, the rationale, and relevant issues and people involved in any prior discussion.
- Ensure that PRs contain adequate tests and code comments for a future contributor to derive intent and modify your code safely.
- You are welcome to use the `rome` repo for your WIP branches. However you should prefix branches with your username. ie. `git branch sebmck/feature`. Branches not involved in an active PR will be regularly pruned.
- If you are adding a new feature then ensure that it has been discussed or approved on GitHub or Discord.
- If necessary, identify potential owners for PR review and approval.
- All code needs to go through Pull Requests (PR) and must pass status checks before being merged. If a PR is merged that breaks `main` due to the branch not being up to date, then it should either be reverted or a quick fix merged as a separate PR.
- If a PR is against code that you have previously committed and is either small changes, bug fixes, or refactors, then you're free to merge it without any review. However if you don't feel confident in your changes then you can wait for approval from another core contributor.

## Moderation

Outlined below is the process for Code of Conduct violation reviews.

### Reporting

Anyone may report a violation. Violations can be reported in the following ways:

- In private, via <conduct@romefrontend.dev> which is listed in the [Code of Conduct](./CODE_OF_CONDUCT.md). This email address is monitored by all stewards.
- In private, via email to one or more stewards.
- In private, via direct message to a project steward on Discord
- In public, via a GitHub comment (mentioning `@romefrontend/stewards`).
- In public, via the project Discord server.

### Who gets involved?

Each report will be assigned reviewers. These will initially be all project [stewards](#stewards).

In the event of any conflict of interest - ie. stewards who are personally connected to a situation, they must immediately recuse themselves.

At request of the reporter and if deemed appropriate by the reviewers, another neutral third-party may be involved in the review and decision process.

### Review

If a report doesn’t contain enough information, the reviewers will strive to obtain all relevant data before acting.

The reviewers will then review the incident and determine, to the best of their ability:

- What happened.
- Whether this event constitutes a Code of Conduct violation.
- Who, if anyone, was involved in the violation.
- Whether this is an ongoing situation.

The reviewers should aim to have a resolution agreed very rapidly; if not agreed within a week, they will inform the parties of the planned date.

### Resolution

Responses will be determined by the reviewers on the basis of the information gathered and of the potential consequences. It may include:

- taking no further action
- issuing a reprimand (private or public)
- asking for an apology (private or public)
- permanent ban from the GitHub org and Discord server
- revoked contributor status

## OpenCollective fund allocation

- Funds will be allocated for project-specific services such as domain registration and website hosting.
- Other usage of funds has yet to be decided.
- Expenses will be approved by project stewards.
