import {ParsedPath, ParsedPathUID} from "../types";
import {BasePath, FilePathMemo} from "../bases";

export default class UIDPath extends BasePath<ParsedPathUID, UIDPath> {
	public [Symbol.toStringTag] = "UIDPath";

	protected _assert(): UIDPath {
		return this;
	}

	protected _equalAbsolute(parsed: ParsedPath): boolean {
		return parsed.type === "uid";
	}

	protected _join(): string {
		return `uid://${this.getDisplaySegments().join("/")}`;
	}

	protected _getUnique() {
		return this;
	}

	protected _fork(parsed: ParsedPathUID, opts: FilePathMemo<UIDPath>): UIDPath {
		return new UIDPath(parsed, opts);
	}

	protected _format(): string {
		return this.relativeSegments.join("/");
	}

	public isUID(): this is UIDPath {
		return true;
	}

	public assertUID(): UIDPath {
		return this;
	}
}

UIDPath.prototype[Symbol.toStringTag] = "UIDPath";