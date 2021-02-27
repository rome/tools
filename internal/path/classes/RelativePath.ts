import {AnyParsedPath, ParsedPathRelative} from "../types";
import {BasePath, FilePathMemo} from "./BasePath";
import {AnyFilePath} from "../types";

export default class RelativePath extends BasePath<ParsedPathRelative, RelativePath> {
	public [Symbol.toStringTag] = "RelativePath";

	// TypeScript is structurally typed whereas here we would prefer nominal typing
	// We use this as a hack.
	protected type: "relative" = "relative";

	protected _equalAbsolute(other: AnyParsedPath): boolean {
		return other.type === "relative";
	}
	
	protected _getUnique() {
		if (this.parsed.explicitRelative) {
			return this._fork({
				...this.parsed,
				explicitRelative: false,
			}, {
				parent: this.memo.parent,
			});
		} else {
			return this;
		}
	}

	protected _format(): string {
		return this.join();
	}

	protected _join(segments: Array<string>): string {
		if (this.parsed.explicitRelative && segments[0] !== "..") {
			segments.unshift(".");
		}

		if (segments.length === 0) {
			return ".";
		}

		return segments.join("/");
	}

	protected _assert(): RelativePath {
		return this;
	}

	protected _fork(
		parsed: ParsedPathRelative,
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

	public isExplicitRelative(): boolean {
		return this.parsed.explicitRelative;
	}

	public isImplicitRelative(): boolean {
		return !this.parsed.explicitRelative;
	}

	public toExplicitRelative(): RelativePath {
		if (this.isExplicitRelative()) {
			return this;
		} else {
			return new RelativePath({
				...this.parsed,
				explicitRelative: true,
			}, {
				ext: this.memo.ext,
				parent: this.memo.parent,
			});
		}
	}
}
