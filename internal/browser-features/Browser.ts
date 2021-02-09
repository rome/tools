import {Consumer, consumeUnknown} from "@internal/consume";
import {data, regions} from "@internal/browsers-db";

export interface BrowserProps {
	id: string,
	version?: string,
}

export abstract class Browser {
	private readonly id: string;
	private readonly version?: string;

	protected constructor({id, version}: BrowserProps) {
		this.id = id;
		this.version = version;
	}

	protected getDataConsumer(): Consumer {
		return consumeUnknown(data, "parse");
	}

	protected getAgentConsumer(): Consumer {
		return this.getDataConsumer().get("agents").get(this.getId());
	}

	protected getVersionConsumer(): Consumer {
		return this.getAgentConsumer().get("vs").asImplicitArray().find(value => value.get("v").asString() === this.getVersion())!;
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

	public getType(): string {
		return this.getAgentConsumer().get("t").asString();
	}

	public getCurrentVersion(): string {
		return this.getAgentConsumer().get("cv").asString();
	}

	public getDefaultPrefix(): string {
		return this.getAgentConsumer().get("p").asString();
	}

	public getPrefix(): string {
		return this.getVersionConsumer().get("p").asStringOrVoid() ?? this.getDefaultPrefix();
	}

	public getGlobalUsage(): number {
		return this.getVersionConsumer().get("g").asNumber();
	}

	public getRawReleaseDate(): number|undefined {
		return this.getVersionConsumer().get("r").asNumberOrVoid();
	}

	public getReleaseDate(): Date|undefined {
		return this.getRawReleaseDate() !== undefined ? new Date(this.getRawReleaseDate()!) : undefined;
	}

	public getVersions(): string[] {
		return this.getAgentConsumer().get("vs").asImplicitArray().map(value => value.get("v").asString());
	}

	/**
	 * Whether the css feature requires a browser prefix
	 *
	 * @param feature check internal/browsers-db/README.md for more info
	 */
	public cssFeatureRequiresPrefix(feature: string): boolean {
		return this.getDataConsumer().get("data").get(feature).get("s").get(this.getId()).get(this.getVersion()).asBoolean(false);
	}

	/**
	 * Get the region usage of the browser
	 *
	 * @param region check internal/browsers-db/README.md for more info
	 */
	public getRegionUsage(region: string): number|undefined {
		return consumeUnknown(regions, "parse").get(region).get("data").get(this.getId()).get(this.getVersion()).asNumberOrVoid()
	}

	// https://github.com/microsoft/TypeScript/issues/34516
	public /*abstract*/ static getVersionImplementation(version: string): Browser {
		throw new Error("Unimplemented");
	}
}
