import {ParsedPath, ParsedPathURL, Path} from "../types";
import {parseURLPathRelativeSegments} from "../parse";
import {BasePath, FilePathMemo} from "../bases";

import RelativePath from "./RelativePath";
import {equalArray} from "@internal/typescript-helpers";

export default class URLPath extends BasePath<ParsedPathURL, URLPath> {
	private memoizedJoinedPathname: undefined | string;
	public [Symbol.toStringTag] = "URLPath";

	protected _getUnique() {
		return this;
	}

	protected _forkAppend(segments: string[]): URLPath {
		const newParsed = parseURLPathRelativeSegments([
			...this.getSegments(),
			...segments,
		]);
		return new URLPath({
			...this.parsed,
			...newParsed,
		});
	}

	protected _format(): string {
		return this.join();
	}

	public joinPathname(): string {
		if (this.memoizedJoinedPathname !== undefined) {
			return this.memoizedJoinedPathname;
		}

		const {hash, search} = this.parsed;

		const segments: string[] = this.getDisplaySegments().map((seg) =>
			encodeURIComponent(seg)
		);

		if (hash !== undefined || search.size > 0) {
			let lastSegment = segments.pop() ?? "";

			// Add search
			if (search.size > 0) {
				const searchPairs: string[] = [];
				for (const [key, values] of search) {
					let encodedKey = encodeURIComponent(key);
					for (const value of values) {
						if (value === "") {
							searchPairs.push(encodedKey);
						} else {
							searchPairs.push(`${encodedKey}=${encodeURIComponent(value)}`);
						}
					}
				}
				lastSegment += `?${searchPairs.join("&")}`;
			}

			// Add hash
			if (hash !== undefined) {
				lastSegment += `#${encodeURIComponent(hash)}`;
			}

			segments.push(lastSegment);
		}

		const joined = `/${segments.join("/")}`;
		this.memoizedJoinedPathname = joined;
		return joined;
	}

	protected _join(): string {
		const {protocol, port, hostname, username, password} = this.parsed;

		// Build prefix
		let prefix = `${protocol}//`;
		if (username !== undefined || password !== undefined) {
			prefix += `${encodeURIComponent(username ?? "")}:${encodeURIComponent(
				password ?? "",
			)}@`;
		}
		prefix += encodeURIComponent(hostname);
		if (port !== undefined) {
			prefix += `:${String(port)}`;
		}

		// Join it all together!
		return prefix + this.joinPathname();
	}

	protected _assert(): URLPath {
		return this;
	}

	protected _equalAbsolute(other: ParsedPath): boolean {
		if (other.type !== "url") {
			return false;
		}

		const {parsed} = this;

		// Check for primitive equivalency
		if (
			other.protocol !== parsed.protocol ||
			other.username !== parsed.username ||
			other.password !== parsed.password ||
			other.hostname !== parsed.hostname ||
			other.port !== parsed.port ||
			other.hash !== parsed.hash ||
			other.search.size !== parsed.search.size
		) {
			return false;
		}

		// Check for search parameter equivalency
		for (const [key, value] of parsed.search) {
			const otherValue = other.search.get(key);
			if (otherValue === undefined || !equalArray(otherValue, value)) {
				return false;
			}
		}

		return true;
	}

	protected _fork(parsed: ParsedPathURL, opts: FilePathMemo<URLPath>): URLPath {
		return new URLPath(parsed, opts);
	}

	public assertURL(): URLPath {
		return this;
	}

	public isURL(): this is URLPath {
		return true;
	}

	public getPathname(): RelativePath {
		return new RelativePath({
			type: "relative",
			relativeSegments: this.relativeSegments,
			explicitRelative: true,
			explicitDirectory: this.parsed.explicitDirectory,
		}).assertRelative();
	}

	public getHostname(): string {
		return this.parsed.hostname;
	}

	public getPort(): undefined | number {
		return this.parsed.port;
	}

	public getProtocol(): string {
		return this.parsed.protocol;
	}

	public getParams(): ParsedPathURL["search"] {
		return this.parsed.search;
	}

	public getParam(key: string): undefined | string {
		const values = this.getParamAll(key);
		if (values === undefined) {
			return undefined;
		} else {
			return values[0];
		}
	}

	public getParamAll(key: string): undefined | string[] {
		return this.parsed.search.get(key);
	}

	public clearParams(): URLPath {
		return new URLPath({
			...this.parsed,
			search: new Map(),
		});
	}

	public resolve(path: Path): URLPath {
		if (path.isURL()) {
			return path.assertURL();
		} else if (path.isAbsolute() && path.parsed.type === "absolute-unix") {
			return this.append(...path.getSegments());
		} else {
			return this.append(path.assertRelative());
		}
	}

	public fetch(init?: RequestInit): Promise<Response> {
		return fetch(this.join(), init);
	}
}

URLPath.prototype[Symbol.toStringTag] = "URLPath";
