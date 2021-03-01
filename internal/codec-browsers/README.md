# `codec-browsers`

Used to parse browser target syntax.

## Usage

```ts
import {resolveBrowsers} from "@internal/codec-browsers";

// Accepts a string, a list of strings or parser options

resolveBrowsers("firefox 84"); // Returns a list of browser implmentations; => [firefox:84]

resolveBrowsers("firefox 84", "firefox 85"); // => [firefox:84, firefox:85]

resolveBrowsers({
	input: "firefox 84",
	offsetPosition: ...,
	path: ...,
	includeSourceTextInDiagnostics: ...,
	integrity: ...,
	sourceText: ...,
});
// => [firefox:84]
```

For more information about browser implementations check https://github.com/rome/tools/blob/main/internal/browser-features/README.md

## Queries

For illustration purposes, browser implementations will be shown as `<browser>:<id>`.

### Combination

* Queries can be separated using a comma `,` or the `or` keyword.

	![or](https://user-images.githubusercontent.com/33844379/109436937-8001a400-7a22-11eb-9f62-4b22ebc20f21.png)

	`firefox 84, firefox 85` and `firefox 84 or firefox 85` are the same and result to `[firefox:84, firefox:85]`.


* Queries can be combined with the `and` keyword.

	![and](https://user-images.githubusercontent.com/33844379/109436933-79732c80-7a22-11eb-9593-8f27607b0fa7.png)

	`> 5% and firefox all` would result to all firefox versions that are used by more than 5%.


* Queries can be inverted (removed instead of added) with the `not` keyword.

	![not](https://user-images.githubusercontent.com/33844379/109436925-737d4b80-7a22-11eb-88b9-4d5cbcd48556.png)

	`> 5%, not chrome all` would result in all browser versions used by more than 5% but not chrome.

### Syntax

* `modern`, `default`: Rome's modern (default) browser targets (`last 2 versions, not dead`).
* `firefox 84`: Firefox (desktop) version 84.
	* `chrome > 80`: Chrome versions bigger than 80 (also available with `>=`, `<` and `<=`).
	* `safari 10-14`: Safari versions between 10 and 14.
* `cover 99.5%`: Most used browser versions that cover 99.5%.
	* `cover 99.5% in BE`: Most used browser versions that cover 99.5% in Belgium.
* `current versions`: Current version of all browsers.
	* `current opera versions`: Current Opera version.
* `dead`: Browsers without official support or updates for 24 months (`bb <= 10, op_mob <= 12.1, samsung 4` as of writing).
* `last 2 versions`: Last 2 versions of all browsers.
	* `last 2 firefox versions`: Last 2 Firefox versions.
	* `last 2 major versions`: Last 2 major versions of all browsers (includes all minor versions).
	* `last 2 firefox major versions`: Last 2 Firefox major versions (includes all minor versions).
* `last 2 years`: All browser versions released in the last 2 years (also available with `days` and `months`).
* `> 5%`: Browser versions used by more than 5% in global usage statistics (also available with `>=`, `<` and `<=`).
	* `> 5% in BE`: Browser versions used by more than 5% in Belgium usage statistics (also available with `>=`, `<` and `<=`).
* `since 2020`: Browser versions released since 2020 (also available with month `2020-02` and day `2020-02-15`).
* `unreleased versions`: Unreleased versions of all browsers.
	* `unreleased firefox versions`: Unreleased Firefox versions.

## Browsers

⚠️ Rome does NOT support Internet Explorer nor Edge versions based on Internet Explorer.

For a full list of all supported browsers check out https://github.com/rome/tools/blob/main/internal/browsers-db/README.md#browsers

## Regions

For a list of all available regions check out https://github.com/rome/tools/blob/main/internal/browsers-db/README.md#regions
