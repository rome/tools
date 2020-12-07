---
title: "The Road to Rome: Fundraising and Project Goals"
description: TODO
author_name: Sebastian McKenzie
author_url: https://twitter.com/sebmck
author_avatar: /img/blog/sebmck-avatar.jpg
date: 2020-12-07
tags:
	- post
layout: layouts/funding.liquid
---

# The Road to Rome

## Introduction

{% include blog-post-info.liquid %}

I'm Sebastian McKenzie, the creator of [Babel](https://babeljs.io) and [Yarn](https://yarnpkg.com/). These tools have both inspired me to create Rome, a new project that aims to simplify and improve JavaScript and web development.

Rome consolidates dozens of separate tools into one. Rome can install your dependencies, check your code for errors, run your tests, bundle your code, and more, all via a single CLI. Rome will be able to replace Babel, ESLint, Prettier, Yarn, and webpack. [Learn more](/).

It's been three months since we announced our initial [beta release](/blog/2020/08/08/introducing-rome.html). Since then, we’ve received a tremendous amount of enthusiasm from the community. As that enthusiasm has grown, it’s become clear that Rome will require a full-time developer to be successful and deliver on our ambitious goals and release a stable v1.0.

**I need your help to make it a reality.**

## Funding

I have left my job so I can work independently and focus on what the community needs. This includes a [plugin system](#allow-users-to-extend-functionality-with-plugins), [more configuration](#add-more-configuration-and-have-less-opinions), and [dedicated integrations for existing tools](#integrate-with-existing-tools).

We have an initial goal of **$100,000**. This will allow myself to work independently on our first stable release. Additional funding would allow us to expand upon our release goals, fund future maintenance, and compensate other contributors.

If you're passionate about what we're building, or have otherwise benefited from my work, I would appreciate your financial support.

<noscript>
	<div class="toast error">
		<p>JavaScript is required to use our interactive form and view progress.</p>
		<p>We do not use any trackers. The only third-party library actively used is Sentry for client-side error monitoring. We will additionally <strong>only</strong> load the Stripe API if you make a contribution and click the final checkout button.</p>
	</div>
</noscript>

<div class="progress">
	<div class="progress-loading">Loading...</div>
	<div class="progress-fill">
		<div class="progress-text"></div>
	</div>
	<div class="progress-text progress-total"></div>
</div>

<div class="donate-bar">
	<div class="left">
		<a class="button primary" href="#contribute">Contribute</a>
		<p><a href="/funding/all-contributions"><strong><span class="donation-count">?</span> people</strong> have already contributed</a></p>
	</div>
	<div class="right">
		<a class="button" href="#goals">Goals</a>
		<a class="button" href="#questions-and-answers">FAQ</a>
	</div>
</div>

### Recent Contributions

<ul class="recent-contributions">
	<li>Loading...</li>
</ul>

### Contribute

Prices are in USD. Includes sales tax and international shipping. Refer to [Questions and Answers](#questions-and-answers) for more information.

<div class="tier">
	<h4>Custom</h4>
	<p>Want to donate under $10? Something else? Select your own amount!</p>
	<form class="custom-input">
		<div class="currency-input"><input type="number" step="any" placeholder="0" min="0"></div>
		<button hidden class="primary">Select</button>
	</form>
</div>

<div class="individual-tiers-container">
	Loading...
</div>

#### Business

These tiers include dedicated support, migration assistance, and website advertisement. I'll make sure Rome works well for you and your organization.

Migration support is where I personally help your organization adopt and use Rome. This could include porting configuration, integrating with CI, or even adding new features and configuration to Rome.

Interested in something else or have questions? Get in touch at [sebastian@rome.tools](mailto:sebastian@rome.tools)!

<div class="business-tiers-container">
	Loading...
</div>

## Goals

Funding will allow us to focus on usage and labor-intensive goals. We can make Rome easier to use and work for more people.

### Add more configuration and have less opinions

We have deliberately tried to keep configuration to a minimum. While this does produce a minimal API surface, it makes it almost impossible to easily migrate without losing functionality or changing conventions.

We should aim to reduce the functional differences between Rome and other tools by introducing additional configuration and supported languages. This could include:

- Code formatting options
- Ability to customize expected filenames and directories
- Support for other configuration languages such as YAML and TOML
- More CLI flags 
- Public JavaScript API
- Dynamic configuration (as opposed to static JSON-only configuration files)
- [Allow extending functionality with plugins](#allow-users-to-extend-functionality-with-plugins)

We have so far kept configuration light, as by reducing the amount of configuration options supported, we reduce maintenance cost and the potential for internal bugs.

While this makes it easier for us as maintainers, it makes it drastically more difficult for users. No matter how persuasive our arguments may be for why you should use hard tabs instead of spaces, they seem like artificial and arbitrary constraints and introduces excessive prerequisites for adoption.

Strong defaults and guided documentation for new users can provide the experience we ultimately want to offer, while removing our existing adoption restrictions.

### Integrate with existing tools

Rome attempts to replace many tools. However we should still strive to support scenarios where another tool is better situated or preferred. This can also help during a migration where Rome is used in conjunction with another tool. We can do this in a couple of ways:

**Integrating Rome as a first-class plugin in tools such as Babel, eslint, and webpack**

Rome could be exposed as a plugin for those tools to allow you to adopt the Rome compiler without having to adopt the bundler first. This would reduce adoption prerequisites and allow easier experimentation inside of existing setups.

**Seamlessly integrate other tools into Rome**

We can introduce compatibility layers to have ESLint, Babel, and other tools run inside of Rome itself. ESLint errors could be displayed alongside Rome linter errors with the same UI and output format Instantly you could benefit from Rome's file caching and parallelisation without needing a major migration.

### Assist in migrating from existing tools

It should be easy to migrate from other tools to Rome. First we need to ensure popular configuration options from other tools are supported. Then, offer automated tools to migrate basic setups without users needing to it manually.

This needs to be accompanied with dedicated documentation and guides that can explain the differences between the tools, why you might want to use one over the other, similar concepts, new terminology, and equivalent config options.

### Allow users to extend functionality with plugins

One of the fears with Rome is creating a monoculture where it's impossible to innovate and experiment with new ideas. While it's extremely optimistic to think we'll ever get into any sort of monopolistic position, not allowing extensions does stiffle innovation regardless of our market position by restricting the viability and adoption of new ideas.

Plugins allow us to avoid supporting functionality that we might not want, while still giving users a choice. It reduces our role as an arbiter and allows new languages, non-standard JavaScript features, code conventions, and ideas that interact with Rome to be viable, receive support, and proliferate.

We need to be extremely careful not to get into the position where Babel and Webpack are today, where they're heavily restricted by the usage of internal APIs. We need to be able to maintain our autonomy when it comes to making architectural changes. Balancing this with a powerful plugin API will be a challenge and will likely require several iterations.

### Release undocumented features

Rome currently does a lot more than linting. It's a major challenge today to market and explain Rome when so much of the project isn't officially supported. While we strive to make each individual component of Rome competitive on it's own, to some the biggest advantage and compelling reason for using Rome might be the reduction in dependencies.

We should focus on releasing and maturing basic versions of all core functionality. This would increase user confidence in our architecture and show that Rome is viable as the comprehensive replacement that we want to be.

### Provide accessible and comprehensive documentation

Documentation for developer tools is generally quite obtuse and relies a lot on prerequisite knowledge. This can make it intimidating and inaccessible for developers new to the ecosystem. Further complicating that is the broad scope of what Rome is trying to do.

We have tried to address some of this by making our documentation a single page. This makes it easy to search, and it can be read from top to bottom without needing to jump around to learn about different concepts. However as our supported features grow, it will be more difficult to use this structure without oversimplifcation and doesn't allow different paths for different demographics.

We need to invest in a more scalable approach for our documentation. We can offer dedicated sections that explain features like linting end-to-end without needing to introduce other components like the compiler and bundler that contain significantly more concepts and overwhelm the reader. Separate guides can be offered for new users and those already experienced with other tools to properly cater for multiple audiences.

### Regularly release new versions

One of the reasons Babel was successful is how quickly I was able to quickly fix bugs and release new versions. I would regularly have releases out within minutes of a bug report. This was critical during the early days when adoption was low. Being able to unblock users quickly would often make them more excited to use Babel even though they ran into a bug.

Similarly, we should try and replicate this by building out our release infrastructure to allow the rapid testing and release of versions. We need to maintain momentum as the scope of supported features grow.

We can achieve this with automated releases that can be manually triggered or deployed on a schedule. Automatic changelog generation would also take a lot of the manual work out of producing releases. Nightly releases would allow users to test experimental features and provide early feedback.

## Questions and Answers

### When will physical rewards be shipped?

We are tentatively aiming for the end of April 2021, however due to COVID delays or order volume this could be extended. We'll make sure to keep you updated via email.

### What is my email used for?

We use your email address to send information about your order such as order questions, shipping status and delays. We may also send a survey to decide on customization options for rewards.

Your email address will not be used for any other purpose or be displayed publicly.

### How is payment information stored?

Payment information is entered and stored via Stripe. We do not have access to full payment details. Your billing address is used if we need to calculate and pay sales tax in your jurisdiction.

### What do tier prices include?

Prices include processing fees, international shipping, and sales tax. This does mean the effective donation is reduced if you live in a country with import duty or high shipping cost.

You have the option to add an additional donation in the order review screen if you would like to cover those costs.

### Why do you need my usernames?

Usernames are used to allocate tier rewards. They are not required and you can optionally leave them empty to opt-out.

### What if I want a refund or need to change my order details?

Reply to your order receipt email, or contact me directly at [sebastian@rome.tools](mailto:sebastian@rome.tools) and I'll be happy to help!
