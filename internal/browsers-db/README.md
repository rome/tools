# `browsers-db`

Contains data about the most popular modern browsers.

Stripped down data CC BY 4.0 https://caniuse.com

## Format
data
```ts
{
	agents: {
		[key: string]: { // browser id
			b: string, // browser
			a: string, // abbreviation
			p: string, // prefix
			t: string, // type
			vs: { // versions
				v: string, // version
				g: number, // global usage
				r: number, // release date
				p?: string, // prefix
			}[],
			cv: string, // current version
		}
	},
	categories: {
		[key: string]: string[]
	},
	data: {
		[key: string]: { // feature id
			s: { // stats
				[key: string]: { // browser id
					[key: string]: boolean // browser version: needs prefix
				}
			},
			c: string[] // categories
		}
	}
}
```

regions
```ts
{
	[key: string]: { // region id
		name: string,
		data: {
			[key: string]: { // browser id
				[key: string]: number // browser version: browser usage
			}
		}
	}
}
```
