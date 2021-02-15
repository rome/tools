import { ParsedPath, parsePathSegments } from "../parse";
import { BasePath, FilePathMemo } from "../BasePath";
import { AnyPath } from "../types";

export default class URLPath extends BasePath<URLPath> {
	public [Symbol.toStringTag] = "URLPath";

	protected _assert(): URLPath {
		return this;
	}

	protected _fork(parsed: ParsedPath, opts: FilePathMemo<URLPath>): URLPath {
		return new URLPath(parsed, opts);
	}

	public assertURL(): URLPath {
		return this;
	}

	public isURL(): this is URLPath {
		return true;
	}

	public getDomain(): string {
		return this.segments[2];
	}

	public getProtocol(): string {
		const {absoluteTarget} = this.parsed;
		if (absoluteTarget === undefined) {
			throw new Error("Expected a URLPath to always have an absoluteTarget");
		}
		return absoluteTarget;
	}

	public resolve(path: AnyPath): URLPath {
		if (path.isURL()) {
			return path.assertURL();
		} else if (path.isAbsolute()) {
			// Get the segments that include the protocol and domain
			const domainSegments = this.getSegments().slice(0, 3);
			const finalSegments = [...domainSegments, ...path.getSegments()];
			return new URLPath(
				parsePathSegments(finalSegments, "url"),
			);
		} else {
			return this.append(path.assertRelative());
		}
	}
}