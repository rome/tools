import {INTERNAL, modifyGeneratedFile} from "../_utils";
import https = require("https");
import {version as currentVersion} from "@internal/browsers-db";

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

function get<T>(url: string): Promise<T> {
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
	const version = (await get<{
		version: string;
	}>(packageJsonUrl)).version;
	console.log("Checking for update...");
	if (currentVersion !== version) {
		console.log(`Update found! ${currentVersion} -> ${version}`);
		await updateVersion(version);

		console.log("Updating data...");
		await updateData();
		console.log("Updating data... done!");

		console.log("Updating regions...");
		await updateRegions();
		console.log("Updating regions... done!");
	} else {
		console.log(`Already using latest version! ${version}`);
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
interface agent {
	b: string;
	a: string;
	p: string;
	t: string;
	vs: {
		v: string;
		g: number;
		r: number;
		p?: string;
	}[];
	cv: string;
}

interface feature {
	s: {
		[key: string]: {
			[key: string]: boolean;
		};
	};
	c: string[];
}

interface dataFormat {
	agents: {
		[key: string]: agent;
	};
	categories: {
		[key: string]: string[];
	};
	data: {
		[key: string]: feature;
	};
}

interface regionFormat {
	name: string;
	data: usage;
}

interface usage {
	[key: string]: {
		[key: string]: number;
	};
}

interface regionsFormat {
	[key: string]: regionFormat;
}

async function updateData() {
	const rawData = await get<{
		agents: {
			[key: string]: {
				browser: string;
				abbr: string;
				prefix: string;
				type: string;
				version_list: {
					version: string;
					global_usage: number;
					release_date: number;
					prefix?: string;
				}[];
				current_version: string;
			};
		};
		cats: {
			[key: string]: string[];
		};
		data: {
			[key: string]: {
				stats: {
					[key: string]: {
						[key: string]: string;
					};
				};
				categories: string[];
			};
		};
	}>(fulldataUrl);

	const data = {
		agents: (() => {
			let temp = (<dataFormat["agents"]>{});

			for (const agent in rawData.agents) {
				if (agent === "ie" || agent === "ie_mob") {
					continue;
				}

				temp[agent] = {
					b: rawData.agents[agent].browser,
					a: rawData.agents[agent].abbr,
					p: rawData.agents[agent].prefix,
					t: rawData.agents[agent].type,
					vs: rawData.agents[agent].version_list.filter((v) =>
						v.prefix! !== "ms"
					).map((v) => ({
						v: v.version,
						g: v.global_usage,
						r: v.release_date,
						p: v.prefix!.length === 0 ? undefined : v.prefix,
					})),
					cv: rawData.agents[agent].current_version,
				};
			}
			return temp;
		})(),
		categories: rawData.cats,
		data: (() => {
			let temp = (<dataFormat["data"]>{});

			for (const feature in rawData.data) {
				if (
					!rawData.cats.CSS.some((c) =>
						rawData.data[feature].categories.includes(c)
					)
				) {
					continue;
				} // Skip non CSS features for the time being

				let tempFeature = (<feature>{
					s: {},
				});

				tempFeature.c = rawData.data[feature].categories;

				for (const agent in rawData.data[feature].stats) {
					if (agent === "ie" || agent === "ie_mob") {
						continue;
					}

					tempFeature.s[agent] = {};

					for (const v in rawData.data[feature].stats[agent]) {
						if (rawData.data[feature].stats[agent][v].includes("x")) {
							tempFeature.s[agent][v] = true;
						}
					}
				}

				temp[feature] = tempFeature;
			}
			return temp;
		})(),
	};

	await modifyGeneratedFile(
		{
			path: browsersDbFolder.append("data.ts"),
			scriptName: "generated-files/browsers-db",
		},
		async () => {
			return {
				lines: [`export default ${JSON.stringify(data)}`],
				hash: JSON.stringify(data),
			};
		},
	);
}

async function updateRegions() {
	let regionsUsage = (<regionsFormat>{});

	for (const region of REGIONS) {
		// Can't use .forEach because of await
		console.log(`Updating region "${region}"...`);

		const rawRegionUsage = await get<{
			name: string;
			data: {
				[key: string]: {
					[key: string]: number;
				};
			};
		}>(regionUsageUrl.replace("<REGION>", region));

		regionsUsage[region] = {
			name: rawRegionUsage.name,
			data: (() => {
				let temp = (<usage>{});

				for (const agent in rawRegionUsage.data) {
					if (agent === "ie" || agent === "ie_mob") {
						continue;
					}

					temp[agent] = {};

					for (const v in rawRegionUsage.data[agent]) {
						if (
							rawRegionUsage.data[agent][v] &&
							rawRegionUsage.data[agent][v] > 0
						) {
							temp[agent][v] = rawRegionUsage.data[agent][v];
						}
					}
				}
				return temp;
			})(),
		};
	}

	await modifyGeneratedFile(
		{
			path: browsersDbFolder.append("regions.ts"),
			scriptName: "generated-files/browsers-db",
		},
		async () => {
			return {
				lines: [`export default ${JSON.stringify(regionsUsage)}`],
				hash: JSON.stringify(regionsUsage),
			};
		},
	);
}
