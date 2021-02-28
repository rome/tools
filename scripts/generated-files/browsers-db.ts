import {INTERNAL, modifyGeneratedFile, reporter, writeFile} from "../_utils";
import https = require("https");
import {version as currentVersion} from "@internal/browsers-db";
import {Consumer, consumeUnknown} from "@internal/consume";
import {markup} from "@internal/markup";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";

const browsersDbFolder = INTERNAL.append("browsers-db");

const packageJsonUrl = "https://raw.githubusercontent.com/Fyrd/caniuse/master/package.json";
const fulldataUrl = "https://raw.githubusercontent.com/Fyrd/caniuse/master/fulldata-json/data-2.0.json";
const regionUsageUrl = "https://raw.githubusercontent.com/Fyrd/caniuse/master/region-usage-json/<REGION>.json";

const REGIONS = new Set([
	"AD",
	"AE",
	"AF",
	"AG",
	"AI",
	"AL",
	"AM",
	"AO",
	"AR",
	"AS",
	"AT",
	"AU",
	"AW",
	"AX",
	"AZ",
	"BA",
	"BB",
	"BD",
	"BE",
	"BF",
	"BG",
	"BH",
	"BI",
	"BJ",
	"BM",
	"BN",
	"BO",
	"BR",
	"BS",
	"BT",
	"BW",
	"BY",
	"BZ",
	"CA",
	"CD",
	"CF",
	"CG",
	"CH",
	"CI",
	"CK",
	"CL",
	"CM",
	"CN",
	"CO",
	"CR",
	"CU",
	"CV",
	"CX",
	"CY",
	"CZ",
	"DE",
	"DJ",
	"DK",
	"DM",
	"DO",
	"DZ",
	"EC",
	"EE",
	"EG",
	"ER",
	"ES",
	"ET",
	"FI",
	"FJ",
	"FK",
	"FM",
	"FO",
	"FR",
	"GA",
	"GB",
	"GD",
	"GE",
	"GF",
	"GG",
	"GH",
	"GI",
	"GL",
	"GM",
	"GN",
	"GP",
	"GQ",
	"GR",
	"GT",
	"GU",
	"GW",
	"GY",
	"HK",
	"HN",
	"HR",
	"HT",
	"HU",
	"ID",
	"IE",
	"IL",
	"IM",
	"IN",
	"IQ",
	"IR",
	"IS",
	"IT",
	"JE",
	"JM",
	"JO",
	"JP",
	"KE",
	"KG",
	"KH",
	"KI",
	"KM",
	"KN",
	"KP",
	"KR",
	"KW",
	"KY",
	"KZ",
	"LA",
	"LB",
	"LC",
	"LI",
	"LK",
	"LR",
	"LS",
	"LT",
	"LU",
	"LV",
	"LY",
	"MA",
	"MC",
	"MD",
	"ME",
	"MG",
	"MH",
	"MK",
	"ML",
	"MM",
	"MN",
	"MO",
	"MP",
	"MQ",
	"MR",
	"MS",
	"MT",
	"MU",
	"MV",
	"MW",
	"MX",
	"MY",
	"MZ",
	"NA",
	"NC",
	"NE",
	"NF",
	"NG",
	"NI",
	"NL",
	"NO",
	"NP",
	"NR",
	"NU",
	"NZ",
	"OM",
	"PA",
	"PE",
	"PF",
	"PG",
	"PH",
	"PK",
	"PL",
	"PM",
	"PN",
	"PR",
	"PS",
	"PT",
	"PW",
	"PY",
	"QA",
	"RE",
	"RO",
	"RS",
	"RU",
	"RW",
	"SA",
	"SB",
	"SC",
	"SD",
	"SE",
	"SG",
	"SH",
	"SI",
	"SK",
	"SL",
	"SM",
	"SN",
	"SO",
	"SR",
	"ST",
	"SV",
	"SY",
	"SZ",
	"TC",
	"TD",
	"TG",
	"TH",
	"TJ",
	"TK",
	"TL",
	"TM",
	"TN",
	"TO",
	"TR",
	"TT",
	"TV",
	"TW",
	"TZ",
	"UA",
	"UG",
	"US",
	"UY",
	"UZ",
	"VA",
	"VC",
	"VE",
	"VG",
	"VI",
	"VN",
	"VU",
	"WF",
	"WS",
	"YE",
	"YT",
	"ZA",
	"ZM",
	"ZW",
	"alt-af",
	"alt-an",
	"alt-as",
	"alt-eu",
	"alt-na",
	"alt-oc",
	"alt-sa",
	"alt-ww",
]);

function get(url: string): Promise<unknown> {
	return new Promise((resolve, reject) => {
		const req = https.get(
			url,
			(res) => {
				let buff = "";

				res.setEncoding("utf8");
				res.on(
					"data",
					(chunk) => {
						buff += chunk;
					},
				);

				res.on(
					"end",
					() => {
						try {
							resolve(JSON.parse(buff));
						} catch (err) {
							reject(err);
						}
					},
				);
			},
		);

		req.on(
			"error",
			(err) => {
				reject(err);
			},
		);
	});
}

export async function main() {
	const version = consumeUnknown(
		await get(packageJsonUrl),
		DIAGNOSTIC_CATEGORIES.parse,
	).get("version").asString();
	if (currentVersion !== version) {
		reporter.success(
			`[browsers-db] Update found! ${currentVersion} -> ${version}`,
		);
		await updateData();
		await updateRegions();
		await updateVersion(version);

		reporter.warn(
			markup`Don't forget to update the snapshots with <code>./rome test internal/codec-browsers/index.test.ts --update-snapshots</code>`,
		);
	} else {
		reporter.success(`[browsers-db] Already using latest version! ${version}`);
	}
}

async function updateVersion(version: string) {
	await modifyGeneratedFile(
		{
			path: browsersDbFolder.append("index.ts"),
			scriptName: "generated-files/browsers-db",
		},
		async () => {
			return {lines: [`export const version = "${version}"`], hash: version};
		},
	);
}

// See browsers-db/README.md#Format
interface Agent {
	b: string;
	a: string;
	p: string;
	t: string;
	vs: {
		v: number;
		g: number;
		r?: number;
		p?: string;
	}[];
	cv: number;
}

interface Feature {
	s: Map<string, Map<number, boolean>>;
	c: string[];
}

interface DataFormat {
	agents: Map<string, Agent>;
	categories: Map<string, string[]>;
	data: Map<string, Feature>;
}

interface RegionFormat {
	name: string;
	data: Usage;
}

type Usage = Map<string, Map<number, number>>;

type RegionsFormat = Map<string, RegionFormat>;

async function updateData() {
	const progress = reporter.progress({title: "Updating data"});

	const rawData = consumeUnknown(
		await get(fulldataUrl),
		DIAGNOSTIC_CATEGORIES.parse,
	);

	const data: DataFormat = {
		agents: generateDataAgents(rawData),
		categories: new Map<string, string[]>(
			Object.entries(rawData.get("cats").asAny()),
		),
		// Would take to many lines to convert it without 'any'
		data: generateDataData(rawData),
	};

	await writeFile(
		browsersDbFolder.append("data.json"),
		JSON.stringify(mapToObject(data)),
	);

	progress.end();
}

function generateDataAgents(rawData: Consumer) {
	let agents: DataFormat["agents"] = new Map<string, Agent>();

	for (const agent in rawData.get("agents").asUnknownObject()) {
		if (agent === "ie" || agent === "ie_mob") {
			continue;
		}

		const vs = generateDataAgentsVersions(
			rawData.get("agents").get(agent).get("version_list").asImplicitArray(),
		);
		const currentVersion = rawData.get("agents").get(agent).get(
			"current_version",
		).asString();

		agents.set(
			agent,
			{
				b: rawData.get("agents").get(agent).get("browser").asString(),
				a: rawData.get("agents").get(agent).get("abbr").asString(),
				p: rawData.get("agents").get(agent).get("prefix").asString(),
				t: rawData.get("agents").get(agent).get("type").asString(),
				vs,
				cv: isNaN(parseFloat(currentVersion))
					? vs[vs.length - 1].v
					: parseFloat(currentVersion), // Defaults to last version
			},
		);
	}
	return agents;
}

function generateDataAgentsVersions(rawVersions: Consumer[]): Agent["vs"] {
	const versions: Agent["vs"] = [];

	rawVersions.forEach((v) => {
		if (v.get("version").asString().includes("-")) {
			// Could be optimized but copying 3 times works
			// Converts versions like `12-20` into 2 versions 12 and 20
			versions.push({
				v: parseFloat(v.get("version").asString().split("-")[0]),
				g: v.get("global_usage").asNumber(),
				r: v.get("release_date").asNumberOrVoid(),
				p: v.get("prefix").asString().length === 0
					? undefined
					: v.get("prefix").asString(),
			});

			versions.push({
				v: parseFloat(v.get("version").asString().split("-")[1]),
				g: v.get("global_usage").asNumber(),
				r: v.get("release_date").asNumberOrVoid(),
				p: v.get("prefix").asString().length === 0
					? undefined
					: v.get("prefix").asString(),
			});
		} else {
			versions.push({
				v: isNaN(parseFloat(v.get("version").asString()))
					? 1
					: parseFloat(v.get("version").asString()),
				// String may be "all", replaced with 1
				g: v.get("global_usage").asNumber(),
				r: v.get("release_date").asNumberOrVoid(),
				p: v.get("prefix").asString().length === 0
					? undefined
					: v.get("prefix").asString(),
			});
		}
	});

	return versions;
}

function generateDataData(rawData: Consumer) {
	let dataData: DataFormat["data"] = new Map<string, Feature>();

	for (const feature in rawData.get("data").asUnknownObject()) {
		// Skip non CSS features for the time being
		if (
			!rawData.get("cats").get("CSS").asMappedArray((c) => c.asString()).some((
				c,
			) =>
				rawData.get("data").get(feature).get("categories").asMappedArray((c) =>
					c.asString()
				).includes(c)
			)
		) {
			continue;
		}

		const stats = new Map<string, Map<number, boolean>>();

		for (const agent in rawData.get("data").get(feature).get("stats").asUnknownObject()) {
			if (agent === "ie" || agent === "ie_mob") {
				continue;
			}

			const featureAgents = new Map<number, boolean>();

			for (const v in rawData.get("data").get(feature).get("stats").get(agent).asUnknownObject()) {
				if (
					rawData.get("data").get(feature).get("stats").get(agent).get(v).asString().includes(
						"x",
					)
				) {
					// Could be optimized but copying 3 times works
					// Converts versions like `12-20` into 2 versions 12 and 20
					if (v.includes("-")) {
						featureAgents.set(parseFloat(v.split("-")[0]), true);
						featureAgents.set(parseFloat(v.split("-")[1]), true);
					} else {
						featureAgents.set(parseFloat(v), true);
					}
				}
			}

			if (featureAgents.size !== 0) {
				stats.set(agent, featureAgents);
			}
		}

		dataData.set(
			feature,
			{
				s: stats,
				c: rawData.get("data").get(feature).get("categories").asMappedArray((c) =>
					c.asString()
				),
			},
		);
	}
	return dataData;
}

async function updateRegions() {
	const progress = reporter.progress({title: "Updating regions"});
	progress.setTotal(REGIONS.size);

	let regionsUsage: RegionsFormat = new Map<string, RegionFormat>();

	for (const region of REGIONS) {
		// Can't use .forEach because of await
		progress.setText(`Updating region "${region}"`);

		const rawRegionUsage = consumeUnknown(
			await get(regionUsageUrl.replace("<REGION>", region)),
			DIAGNOSTIC_CATEGORIES.parse,
		);

		regionsUsage.set(
			region,
			{
				name: rawRegionUsage.get("name").asString(),
				data: generateRegionsData(rawRegionUsage),
			},
		);

		progress.tick();
	}

	await writeFile(
		browsersDbFolder.append("regions.json"),
		JSON.stringify(mapToObject(regionsUsage)),
	);

	progress.end();
}

function generateRegionsData(rawRegionUsage: Consumer) {
	const usage: Usage = new Map<string, Map<number, number>>();

	for (const agent in rawRegionUsage.get("data").asUnknownObject()) {
		if (agent === "ie" || agent === "ie_mob") {
			continue;
		}

		const usageAgent = new Map<number, number>();

		for (const v in rawRegionUsage.get("data").get(agent).asUnknownObject()) {
			if (
				rawRegionUsage.get("data").get(agent).get(v).asNumberOrVoid() != null &&
				rawRegionUsage.get("data").get(agent).get(v).asNumber() > 0
			) {
				// Could be optimized but copying 3 times works
				// Converts versions like `12-20` into 2 versions 12 and 20
				if (v.includes("-")) {
					usageAgent.set(
						parseFloat(v.split("-")[0]),
						rawRegionUsage.get("data").get(agent).get(v).asNumber(),
					);
					usageAgent.set(
						parseFloat(v.split("-")[1]),
						rawRegionUsage.get("data").get(agent).get(v).asNumber(),
					);
				} else {
					usageAgent.set(
						parseFloat(v),
						rawRegionUsage.get("data").get(agent).get(v).asNumber(),
					);
				}
			}
		}

		usage.set(agent, usageAgent);
	}
	return usage;
}

// Needed to convert to JSON
// rome-ignore lint/ts/noExplicitAny: the object could be anything and is just used to be converted to a JSON string
function mapToObject(object: any): object {
	if (object instanceof Map) {
		let result: any = {};
		object.forEach((value, key) => {
			result[key] = mapToObject(value);
		});
		return result;
	}
	if (Array.isArray(object)) {
		return object;
	}
	if (typeof object === "object") {
		let result: any = {};
		for (const key in object) {
			result[key] = mapToObject(object[key]);
		}
		return result;
	}
	return object;
}
