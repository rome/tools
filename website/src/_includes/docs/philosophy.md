## Philosophy

This list includes general ethos the project should abide by. This list is not comprehensive. Some of these are obvious but are stated for completeness.

### Project Management

- **Set clear expectations.** Make project intent and decisions known well in advance. Nothing should be a surprise.
- **Clear messaging of decisions.** The team might evaluate options and make decisions using private channels. While the team will try to keep discussions
  using public channels like GitHub Discussions or Discord, frequent private check-in are the norm, due to the nature of the private company.
  When decisions occur via private channels, the team has to commit to communicate these decisions using the public channels.

### Releases, versioning and breaking changes

- **Rome uses semantic versioning for mostly all packages**. More precisely, [semver v2](https://semver.org/spec/v2.0.0.html).
- **The VSCode extension uses semantic versioning**. More precisely, [semver v1](https://semver.org/spec/v1.0.0.html). The market doesn't support pre-releases.
Even numbers are meant for stable releases, odd numbers for pre-releases.
- **Rome commits to a monthly release**, with _announced features_. 
- **Rome commits to patch releases**, if the bugs are blockers and no workarounds can't be suggested.
- **Features that might slip in patch releases won't be announced**. This can happen, patch releases
are published and some new features are part of the code.
- **Features that are not announced are considered unstable and undocumented**. Users can use them and provide feedback to help developments,
but the absence of documentation should be the norm in these cases.
- **Rome may release breaking changes very often**. Due to the nature of the tool (many tools in one),
the team needs to break things easily without fear.
- **Provide migration paths for breaking changes**. Provide *automated* migration paths when possible,
or document them in the changelog.
- **Things that change must be deprecated first**, where it's possible. 
This allows a smoother path for migration to new versions.

Following, a list of breaking changes. This list is used by the team to assess the nature
of their changes.

**What's considered a _breaking change_**:

- changes to CLI arguments and commands;
- changes to APIs change their name or signature. Exception for those APIs that only add new options;
- changes to Rome's defaults, of any kind;
- changes to the recommended lint rules;
- changes to the shape of diagnostics **if they are emitted as objects to clients**, e.g. Js APIs;

**What's _not_ considered a breaking change**:
- lint rules that are promoted form the `nursery` group to a stable group (but not recommended);
- changes to how code is formatted;



### Technical

- **Errors should suggest fixes and hints where possible.** These should be inferred and filtered from usage to reduce surfacing irrelevant and unhelpful messages.
- **Unique and specific error messages.** No generic error messages. This not only helps users understand what went wrong, but should provide maintainers with a unique call site and the necessary information to debug.
- **Optimise API.** Question the existence of all options and flags. Are they necessary? Can they be combined? How can we reduce code branching?
- **Reduce jargon.** Don't assume that users will understand specific terminology. Strive to provide clear meaning for experts and beginners. For example, use "character" where you would traditionally use "token" when producing parser errors.
- **Utilize verbosity when naming commands and flags.** No unnecessary and confusing abbreviations.
- **Use inclusive terminology.** Use gender-neutral pronouns. No ableist slurs. No usage of terms that could be considered insensitive.
- **Build for generic clients.** Don't assume that output will only be consumed by a terminal and using ANSI codes. Use abstractions that could be generalized for viewing in an IDE, browser, or other environments.
- **Terminal output should be unambiguous.** When designing terminal output, don't purely rely on formatting cues such as color. Always use a combination of formatting, symbols, and spacing. If all ANSI codes are stripped, all the output should still be understood.

