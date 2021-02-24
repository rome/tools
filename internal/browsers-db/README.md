# `browsers-db`

Contains data about the most popular modern browsers.

Stripped down data CC BY 4.0 https://caniuse.com

## Usage

```ts
import {version, data, regions} from "@internal/browsers-db";
import {consumeUnknown} from "@internal/consume";

version; // Current version of the data from caniuse

data; // Data (check below)
consumeUnknown(data).get(...).as...(); // Recommended to wrap with Consumer

regions; // Region usage (check below)
consumeUnknown(regions).get(...).as...(); // Recommended to wrap with Consumer
```

## Format
### data
```ts
{
	agents: {
		[key: string]: { // browser id
			b: string, // browser
			a: string, // abbreviation
			p: string, // prefix
			t: string, // type
			vs: { // versions
				v: number, // version
				g: number, // global usage
				r?: number, // release date
				p?: string, // prefix
			}[],
			cv: number, // current version
		}
	},
	categories: {
		[key: string]: string[]
	},
	data: {
		[key: string]: { // feature id
			s: { // stats
				[key: string]: { // browser id
					[key: number]: boolean // browser version: needs prefix
				}
			},
			c: string[] // categories
		}
	}
}
```

Currently, only contains data for css features.
Doesn't have IE and Edge pre Webkit data.

Feature Ids are the same as the filenames of https://github.com/Fyrd/caniuse/tree/master/features-json (without `.json`).

Categories include (will not stay up to date, check `data.categories`).
```json
"CSS":[
	 "CSS",
	 "CSS2",
	 "CSS3"
],
"HTML5":[
	 "Canvas",
	 "HTML5"
],
"JS":[
	 "JS"
],
"JS API":[
	 "JS API"
],
"Other":[
	 "Other",
	 "DOM",
	 "PNG"
],
"Security":[
	 "Security"
],
"SVG":[
	 "SVG"
]
```

### regions
```ts
{
	[key: string]: { // region id
		name: string, // full region name
		data: {
			[key: string]: { // browser id
				[key: number]: number // browser version: browser usage
			}
		}
	}
}
```

If the region usage for the browsers version is unknown or zero, it's set as undefined.

Same categories as above.

Available regions:

`AD`, `AE`, `AF`, `AG`, `AI`, `AL`, `AM`, `AO`, `AR`, `AS`, `AT`, `AU`, `AW`, `AX`, `AZ`, `BA`, `BB`, `BD`, `BE`, `BF`,
`BG`, `BH`, `BI`, `BJ`, `BM`, `BN`, `BO`, `BR`, `BS`, `BT`, `BW`, `BY`, `BZ`, `CA`, `CD`, `CF`, `CG`, `CH`, `CI`, `CK`,
`CL`, `CM`, `CN`, `CO`, `CR`, `CU`, `CV`, `CX`, `CY`, `CZ`, `DE`, `DJ`, `DK`, `DM`, `DO`, `DZ`, `EC`, `EE`, `EG`, `ER`,
`ES`, `ET`, `FI`, `FJ`, `FK`, `FM`, `FO`, `FR`, `GA`, `GB`, `GD`, `GE`, `GF`, `GG`, `GH`, `GI`, `GL`, `GM`, `GN`, `GP`,
`GQ`, `GR`, `GT`, `GU`, `GW`, `GY`, `HK`, `HN`, `HR`, `HT`, `HU`, `ID`, `IE`, `IL`, `IM`, `IN`, `IQ`, `IR`, `IS`, `IT`,
`JE`, `JM`, `JO`, `JP`, `KE`, `KG`, `KH`, `KI`, `KM`, `KN`, `KP`, `KR`, `KW`, `KY`, `KZ`, `LA`, `LB`, `LC`, `LI`, `LK`,
`LR`, `LS`, `LT`, `LU`, `LV`, `LY`, `MA`, `MC`, `MD`, `ME`, `MG`, `MH`, `MK`, `ML`, `MM`, `MN`, `MO`, `MP`, `MQ`, `MR`,
`MS`, `MT`, `MU`, `MV`, `MW`, `MX`, `MY`, `MZ`, `NA`, `NC`, `NE`, `NF`, `NG`, `NI`, `NL`, `NO`, `NP`, `NR`, `NU`, `NZ`,
`OM`, `PA`, `PE`, `PF`, `PG`, `PH`, `PK`, `PL`, `PM`, `PN`, `PR`, `PS`, `PT`, `PW`, `PY`, `QA`, `RE`, `RO`, `RS`, `RU`,
`RW`, `SA`, `SB`, `SC`, `SD`, `SE`, `SG`, `SH`, `SI`, `SK`, `SL`, `SM`, `SN`, `SO`, `SR`, `ST`, `SV`, `SY`, `SZ`, `TC`,
`TD`, `TG`, `TH`, `TJ`, `TK`, `TL`, `TM`, `TN`, `TO`, `TR`, `TT`, `TV`, `TW`, `TZ`, `UA`, `UG`, `US`, `UY`, `UZ`, `VA`,
`VC`, `VE`, `VG`, `VI`, `VN`, `VU`, `WF`, `WS`, `YE`, `YT`, `ZA`, `ZM`, `ZW`, `alt-af`, `alt-an`, `alt-as`, `alt-eu`,
`alt-na`, `alt-oc`, `alt-sa`, `alt-ww`.
