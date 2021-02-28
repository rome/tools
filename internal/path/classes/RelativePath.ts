import {ParsedPath} from "../parse";
import {BasePath, FilePathMemo} from "./BasePath";
import {AnyFilePath} from "../types";

export default class RelativePath extends BasePath<RelativePath> {
	public [Symbol.toStringTag] = "RelativePath";

	// TypeScript is structurally typed whereas here we would prefer nominal typing
	// We use this as a hack.
	protected type: "relative" = "relative";

	protected _assert(): RelativePath {
		return this;
	}

	protected _fork(
		parsed: ParsedPath,
		opts: FilePathMemo<RelativePath>,
	): RelativePath {
		return new RelativePath(parsed, opts);
	}

	public isFilePath(): this is AnyFilePath {
		return true;
	}

	public assertFilePath(): AnyFilePath {
		return this;
	}

	public isRelative(): this is RelativePath {
		return true;
	}

	public assertRelative(): RelativePath {
		return this;
	}
}
