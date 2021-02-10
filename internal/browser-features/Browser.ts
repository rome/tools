import {Consumer, consumeUnknown} from "@internal/consume";
import {data, regions} from "@internal/browsers-db";

export interface BrowserProps {
	id: string;
	version?: string;
}

export type BrowserTypes = "desktop" | "mobile";

export abstract class Browser {
	private readonly id: string;
	private readonly version: string | undefined;

	private readonly cssFeatureCache = new Map<string, boolean>();

	protected constructor({id, version}: BrowserProps) {
		this.id = id;
		if (version && !this.getVersions().includes(version)) {
			throw new Error(`Browser "${id}" does not have a version "${version}"`);
		}
		this.version = version;
	}

	protected getDataConsumer(): Consumer {
		return consumeUnknown(data, "parse");
	}

	protected getAgentConsumer(): Consumer {
		return this.getDataConsumer().get("agents").get(this.getId());
	}

	protected getVersionConsumer(): Consumer {
		return this.getAgentConsumer().get("vs").asImplicitArray().find((value) =>
			value.get("v").asString() === this.getVersion()
		)!;
	}

	public getId(): string {
		return this.id;
	}

	public getVersion(): string {
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

	public getCurrentVersion(): string {
		return this.getAgentConsumer().get("cv").asString();
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

	public getVersions(): string[] {
		return this.getAgentConsumer().get("vs").asImplicitArray().map((value) =>
			value.get("v").asString()
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

		const value = this.getDataConsumer().get("data").get(feature).get("s").get(
			this.getId(),
		).get(this.getVersion()).asBoolean(false);
		this.cssFeatureCache.set(feature, value);
		return value;
	}

	/**
	 * Get the region usage of the browser
	 *
	 * @param region check internal/browsers-db/README.md for more info
	 */
	public getRegionUsage(region: string): number | undefined {
		return consumeUnknown(regions, "parse").get(region).get("data").get(
			this.getId(),
		).get(this.getVersion()).asNumberOrVoid();
	}
}
