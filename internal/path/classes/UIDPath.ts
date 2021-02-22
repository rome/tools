import {AnyParsedPath, ParsedPathUID} from "../parse";
import {BasePath, FilePathMemo} from "./BasePath";

export default class UIDPath extends BasePath<ParsedPathUID, UIDPath> {
	public [Symbol.toStringTag] = "UIDPath";

	protected _assert(): UIDPath {
		return this;
	}

	protected _equalAbsolute(parsed: AnyParsedPath): boolean {
		return parsed.type === "uid";
	}

	protected _join(relative: Array<string>): string {
		return `uid://${relative.join("/")}`;
	}

	protected _getUnique() {
		return this;
	}

	protected _fork(parsed: ParsedPathUID, opts: FilePathMemo<UIDPath>): UIDPath {
		return new UIDPath(parsed, opts);
	}

	public isUID(): this is UIDPath {
		return true;
	}

	public assertUID(): UIDPath {
		return this;
	}

	public format(): string {
		return this.relativeSegments.slice(2).join("/");
	}
}
