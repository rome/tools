import {Consumer, consumeUnknown} from "@internal/consume";
import {data, regions} from "@internal/browsers-db";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";

export interface BrowserProps {
	id: string;
	version?: number;
}

export type BrowserTypes = "desktop" | "mobile";

export abstract class Browser {
	private readonly id: string;
	private readonly version: number;

	private readonly cssFeatureCache = new Map<string, boolean>();

	protected constructor({id, version}: BrowserProps) {
		this.id = id;
		if (version && !this.getVersions().includes(version)) {
			throw new Error(`Browser "${id}" does not have a version ${version}`);
		}
		this.version = version ?? this.getCurrentVersion();
	}

	protected getDataConsumer(): Consumer {
		return consumeUnknown(data, DIAGNOSTIC_CATEGORIES.parse);
	}

	protected getAgentConsumer(): Consumer {
		return this.getDataConsumer().getPath(["agents", this.getId()]);
	}

	protected getVersionConsumer(): Consumer {
		return this.getAgentConsumer().get("vs").asImplicitArray().find((value) =>
			value.get("v").asNumber() === this.getVersion()
		)!;
	}

	public getId(): string {
		return this.id;
	}

	public getVersion(): number {
		return this.version ?? this.getCurrentVersion();
	}

	public getName(): string {
		return this.getAgentConsumer().get("b").asString();
	}

	public getAbbreviation(): string {
		return this.getAgentConsumer().get("a").asString();
	}

	public getType(): BrowserTypes {
		return this.getAgentConsumer().get("t").asString() as BrowserTypes;
	}

	public getCurrentVersion(): number {
		return this.getAgentConsumer().get("cv").asNumber();
	}

	public getDefaultPrefix(): string {
		return this.getAgentConsumer().get("p").asString();
	}

	public getPrefix(): string {
		return (
			this.getVersionConsumer().get("p").asStringOrVoid() ??
			this.getDefaultPrefix()
		);
	}

	public getGlobalUsage(): number {
		return this.getVersionConsumer().get("g").asNumber();
	}

	/**
	 * Returns release date in milliseconds
	 */
	public getRawReleaseDate(): number | undefined {
		return this.getVersionConsumer().get("r").asNumberOrVoid()
			? this.getVersionConsumer().get("r").asNumber() * 1_000
			: undefined;
	}

	public getReleaseDate(): Date | undefined {
		return this.getRawReleaseDate() !== undefined
			? new Date(this.getRawReleaseDate()!)
			: undefined;
	}

	public isReleased(): boolean {
		return this.getRawReleaseDate() != null;
	}

	public getVersions(): number[] {
		return this.getAgentConsumer().get("vs").asImplicitArray().map((value) =>
			value.get("v").asNumber()
		);
	}

	/**
	 * Whether the css feature requires a browser prefix
	 *
	 * @param feature check internal/browsers-db/README.md for more info
	 */
	public cssFeatureRequiresPrefix(feature: string): boolean {
		if (this.cssFeatureCache.has(feature)) {
			return this.cssFeatureCache.get(feature)!;
		}

		const featureConsumer = this.getDataConsumer().getPath([
			"data",
			feature,
			"s",
		]);

		let value = false;

		if (
			featureConsumer.has(this.getId()) &&
			featureConsumer.get(this.getId()).has(this.getVersion().toString())
		) {
			value = featureConsumer.getPath([
				this.getId(),
				this.getVersion().toString(),
			]).asBoolean(false);
		}

		this.cssFeatureCache.set(feature, value);
		return value;
	}

	/**
	 * Get the region usage of the browser
	 *
	 * @param region check internal/browsers-db/README.md for more info
	 */
	public getRegionUsage(region: string): number | undefined {
		return consumeUnknown(regions, DIAGNOSTIC_CATEGORIES.parse).getPath([
			region,
			"data",
			this.getId(),
			this.getVersion().toString(),
		]).asNumberOrVoid();
	}
}

export class AndroidBrowser extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "android",
			version: props?.version,
		});
	}
}

export class BaiduBrowser extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "baidu",
			version: props?.version,
		});
	}
}

export class BlackberryBrowser extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "bb",
			version: props?.version,
		});
	}
}

export class Chrome extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "chrome",
			version: props?.version,
		});
	}
}

export class ChromeAndroid extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "and_chr",
			version: props?.version,
		});
	}
}

export class Edge extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "edge",
			version: props?.version,
		});
	}
}

export class Firefox extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "firefox",
			version: props?.version,
		});
	}
}

export class FirefoxAndroid extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "and_ff",
			version: props?.version,
		});
	}
}

export class KaiOSBrowser extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "kaios",
			version: props?.version,
		});
	}
}

export class Opera extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "opera",
			version: props?.version,
		});
	}
}

export class OperaMini extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "op_mini",
			version: props?.version,
		});
	}
}

export class OperaMobile extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "op_mob",
			version: props?.version,
		});
	}
}

export class QQBrowser extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "and_qq",
			version: props?.version,
		});
	}
}

export class Safari extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "safari",
			version: props?.version,
		});
	}
}

export class SafariIOS extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "ios_saf",
			version: props?.version,
		});
	}
}

export class SamsungInternet extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "samsung",
			version: props?.version,
		});
	}
}

export class UCBrowserAndroid extends Browser {
	constructor(props?: Pick<BrowserProps, "version">) {
		super({
			id: "and_uc",
			version: props?.version,
		});
	}
}
