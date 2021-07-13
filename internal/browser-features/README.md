# `browser-feature`

Nice implementations for `browsers-db`.

## Usage
```ts
import {getBrowser} from "@internal/browser-features";
import {Chrome, Firefox} from "@internal/browser-features/Browser";

// Get an instance of a browser implementation

let browser = new Firefox();  // Defaults to current version (i.e. latest release at the current time).

browser = new Chrome(); // Get an instance of Chrome.

// Get firefox version 84
browser = new Firefox({
	version: 84
});

// Getting an incorrect version throws
browser = new Firefox({
	version: 567888678856
});
// => Error: Browser "firefox" does not have a version 567888678856


// Get a browser using it's id, name or abbreviation (also ignores case)
browser = getBrowser({
	name: "firefox", // Or "FF", "Firefox"
	version: 72
});
// Same as new Firefox({version: "72"});

// If the browser name doesn't exist it throws
browser = getBrowser({
	name: "TotallyNotABrowser",
});
// => Error: Unknown browser "TotallyNotABrowser"


// Actually using it (browser = firefox 84)

browser.getId(); // => "firefox"

browser.getName(); // => "Firefox" (human friendly name, for "and_ff" it's "Firefox for Android")

browser.getAbbreviation(); // => "FF"

browser.getVersion(); // => 84

browser.getType(); // => "desktop" (or "mobile")

browser.getCurrentVersion(); // => 85 (as of writing this)

browser.getDefaultPrefix(); // => "moz" (default browser prefix, not version dependant)

browser.getPrefix(); // => "moz" (for opera version <= 12.1 it's "o", later versions use "webkit")

browser.getGlobalUsage(); // => 2.3738 (global usage of version 84 as of writing)

browser.getRawReleaseDate(); // => 1607990400000 (ms, undefined if not yet released)

browser.getReleaseDate(); // => Date Tue Dec 15 2020 01:00:00 GMT+0100 (Central European Standard Time) (as a date object, undefined if not yet released)

browser.getVersions(); // => [2, 3, 3.5, 3.6, 4, ..., 85, 86, 87]

browser.cssFeatureRequiresPrefix("transforms2d"); // => false (for firefox 15 this would be true as it requires '-moz-' prefix)

browser.cssFeatureRequiresPrefixRaw("transforms2d"); // => "" (for firefox 15 this would be "y x")

browser.getRegionUsage("BE"); // => 2.4676 (for firefox 84, as of writing)
```

Be sure to also read https://github.com/rome/tools/tree/main/internal/browsers-db
