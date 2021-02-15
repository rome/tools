import { ParsedPath } from "../parse";
import { BasePath, FilePathMemo } from "../BasePath";

export default class UIDPath extends BasePath<UIDPath> {
	public [Symbol.toStringTag] = "UIDPath";

	protected _assert(): UIDPath {
		return this;
	}

	protected _fork(parsed: ParsedPath, opts: FilePathMemo<UIDPath>): UIDPath {
		return new UIDPath(parsed, opts);
	}

	public isUID(): this is UIDPath {
		return true;
	}

	public assertUID(): UIDPath {
		return this;
	}

	public format(): string {
		return this.segments.slice(2).join("/");
	}
}