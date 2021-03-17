import {
	AndroidBrowser,
	BaiduBrowser,
	BlackberryBrowser,
	Browser,
	Chrome,
	ChromeAndroid,
	Edge,
	Firefox,
	FirefoxAndroid,
	KaiOSBrowser,
	Opera,
	OperaMini,
	OperaMobile,
	QQBrowser,
	Safari,
	SafariIOS,
	SamsungInternet,
	UCBrowserAndroid,
} from "@internal/browser-features/Browser";
import {data} from "@internal/browsers-db";
import {consumeUnknown} from "@internal/consume";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";

export type GetBrowserProps = {
	name: BrowserIds | string;
	version?: number;
};

type BrowserIds =
	| "android"
	| "baidu"
	| "bb"
	| "chrome"
	| "and_chr"
	| "edge"
	| "firefox"
	| "and_ff"
	| "kaios"
	| "opera"
	| "op_mini"
	| "op_mob"
	| "and_qq"
	| "safari"
	| "ios_saf"
	| "samsung"
	| "and_uc";

/**
 * Key should be the following format: `${BrowserIds}:${version: string}`
 */
const browserCache = new Map<string, Browser>();
let aliases: Map<string, BrowserIds>;

function loadAliases(): Map<string, BrowserIds> {
	const agents = consumeUnknown(data, DIAGNOSTIC_CATEGORIES.parse).get("agents");
	const abbr = new Map<string, BrowserIds>();
	for (const key in agents.asUnknownObject()) {
		abbr.set(
			agents.getPath([key, "a"]).asString().toLowerCase(),
			key as BrowserIds,
		);
		abbr.set(
			agents.getPath([key, "b"]).asString().toLowerCase(),
			key as BrowserIds,
		);
	}
	return abbr;
}

export function getBrowser({name, version}: GetBrowserProps): Browser {
	if (!aliases) {
		aliases = loadAliases();
	}

	const id = aliases.get(name.toLowerCase()) ?? name.toLowerCase();

	if (browserCache.has(`${id}:${version ?? "current"}`)) {
		return browserCache.get(`${id}:${version ?? "current"}`)!;
	}

	let browser: Browser;

	switch (id) {
		case "android": {
			browser = new AndroidBrowser({version});
			break;
		}
		case "baidu": {
			browser = new BaiduBrowser({version});
			break;
		}
		case "bb": {
			browser = new BlackberryBrowser({version});
			break;
		}
		case "chrome": {
			browser = new Chrome({version});
			break;
		}
		case "and_chr": {
			browser = new ChromeAndroid({version});
			break;
		}
		case "edge": {
			browser = new Edge({version});
			break;
		}
		case "firefox": {
			browser = new Firefox({version});
			break;
		}
		case "and_ff": {
			browser = new FirefoxAndroid({version});
			break;
		}
		case "kaios": {
			browser = new KaiOSBrowser({version});
			break;
		}
		case "opera": {
			browser = new Opera({version});
			break;
		}
		case "op_mini": {
			browser = new OperaMini({version});
			break;
		}
		case "op_mob": {
			browser = new OperaMobile({version});
			break;
		}
		case "and_qq": {
			browser = new QQBrowser({version});
			break;
		}
		case "safari": {
			browser = new Safari({version});
			break;
		}
		case "ios_saf": {
			browser = new SafariIOS({version});
			break;
		}
		case "samsung": {
			browser = new SamsungInternet({version});
			break;
		}
		case "and_uc": {
			browser = new UCBrowserAndroid({version});
			break;
		}
		default:
			throw new Error(`Unknown browser "${id}"`);
	}

	const v = version ?? browser.getCurrentVersion();

	browserCache.set(`${id}:${v}`, browser);
	if (v === browser.getCurrentVersion()) {
		browserCache.set(`${id}:current`, browser);
	}

	return browser;
}

let allBrowserNamesCache: string[];

export function getAllBrowserNames(): string[] {
	if (!allBrowserNamesCache) {
		allBrowserNamesCache = Object.keys(
			consumeUnknown(data, DIAGNOSTIC_CATEGORIES.parse).get("agents").asUnknownObject(),
		);
	}

	return allBrowserNamesCache;
}

interface AllBrowserUsage {
	id: string;
	version: number;
	usage: number;
}

let allBrowserUsagesCache: AllBrowserUsage[];

export function getAllBrowserUsages(region?: string): AllBrowserUsage[] {
	if (allBrowserUsagesCache) {
		return allBrowserUsagesCache;
	}

	const usages: AllBrowserUsage[] = [];

	for (const name of getAllBrowserNames()) {
		for (const version of getBrowser({name}).getVersions()) {
			const browser = getBrowser({name, version});
			usages.push({
				id: browser.getId(),
				version,
				usage: region
					? browser.getRegionUsage(region) ?? 0
					: browser.getGlobalUsage(),
			});
		}
	}

	allBrowserUsagesCache = usages;
	return usages;
}
