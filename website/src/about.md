---
title: About
layout: layouts/page.njk
---

# About

## History

## Team

<ul class="team-list">
	<li>
		<img src="https://github.com/EduardoLopes.png?s=176">
		<p><a href="https://github.com/EduardoLopes">Eduardo Lopes</a></p>
	</li>
	<li>
		<img src="https://github.com/ematipico.png?s=176">
		<p><a href="https://github.com/ematipico">Emanuele Stoppa</a></p>
	</li>
	<li>
		<img src="https://github.com/ooflorent.png?s=176">
		<p><a href="https://github.com/ooflorent">Florent Cailhol</a></p>
	</li>
	<li>
		<img src="https://github.com/jamiebuilds.png?s=176">
		<p><a href="https://github.com/jamiebuilds">Jamie Kyle</a></p>
	</li>
	<li>
		<img src="https://github.com/Kelbie.png?s=176">
		<p><a href="https://github.com/Kelbie">Kevin Kelbie</a></p>
	</li>
	<li>
		<img src="https://github.com/diokey.png?s=176">
		<p><a href="https://github.com/diokey">Olivier Dusabimana</a></p>
	</li>
	<li>
		<img src="https://github.com/bitpshr.png?s=176">
		<p><a href="https://github.com/bitpshr">Paul Bouchon</a></p>
	</li>
	<li>
		<img src="https://github.com/sebmck.png?s=176">
		<p><a href="https://github.com/sebmck">Sebastian McKenzie</a></p>
	</li>
	<li>
		<img src="https://github.com/VictorHom.png?s=176">
		<p><a href="https://github.com/VictorHom">Victor Hom</a></p>
	</li>
	<li>
		<img src="https://github.com/yassere.png?s=176">
		<p><a href="https://github.com/yassere">Yasser Elassal</a></p>
	</li>
</ul>


## Philosophy

This list includes general ethos the project should abide by. This list is not comprehensive. Some of these are obvious but are stated for completeness.

### Project Management

- **Set clear expectations.** Make project intent and decisions known well in advance. Nothing should be a surprise.
- **Transparency.** No back-channel project management. Project conversation and decisions will take place only on public forums such as GitHub, the Rome Discord, and Twitter. The only exception to this is moderation decisions which will be strictly done in private.

### Technical

- **No external dependencies.** This allows us to develop faster and provide a more cohesive experience by integrating internal libraries more tightly and sharing concepts and abstractions. There always exist opportunities to have a better experience by having something purpose-built.
- **Errors should suggest fixes and hints where possible.** These should be inferred and filtered from usage to reduce surfacing irrelevant and unhelpful messages.
- **Unique and specific error messages.** No generic error messages. This not only helps users understand what went wrong, but should provide maintainers with a unique call site and the necessary information to debug.
- **Minimize API.** Question the existence of all options and flags. Are they necessary? Can they be combined? How can we reduce code branching?
- **Reduce jargon.** Don't assume that users will understand specific terminology. Strive to provide clear meaning for experts and beginners. For example, use "character" where you would traditionally use "token" when producing parser errors.
- **Utilize verbosity when naming commands and flags.** No unnecessary and confusing abbreviations.
- **Use inclusive terminology.** Use gender-neutral pronouns. No ableist slurs. No usage of terms that could be considered insensitive.
- **Build for generic clients.** Don't assume that output will only be consumed by a terminal and using ANSI codes. Use abstractions that could be generalized for viewing in an IDE, browser, or other environments.
- **Use strong types.** Don't use loose types such as `any`. Where possible, refine and validate input. Aim for sound types.
- **Terminal output should be unambiguous.** When designing terminal output, don't purely rely on formatting cues such as color. Always use a combination of formatting, symbols, and spacing. If all ANSI codes are stripped, all the output should still be understood.
