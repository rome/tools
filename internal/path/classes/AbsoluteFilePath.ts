import {ParsedPath, parsePathSegments} from "../parse";
import {BasePath, FilePathMemo} from "./BasePath";
import {AnyFilePath, AnyPath, PathSegments} from "../types";
import {HOME_PATH, createRelativePath} from "..";
import RelativePath from "./RelativePath";
import {createFilePath} from "../factories";

export default class AbsoluteFilePath extends BasePath<AbsoluteFilePath> {
	public [Symbol.toStringTag] = "AbsoluteFilePath";

	private chain: undefined | (AbsoluteFilePath[]);

	protected _assert(): AbsoluteFilePath {
		return this;
	}

	protected _fork(
		parsed: ParsedPath,
		opts?: FilePathMemo<AbsoluteFilePath>,
	): AbsoluteFilePath {
		return new AbsoluteFilePath(parsed, opts);
	}

	public isFilePath(): this is AnyFilePath {
		return true;
	}

	public assertFilePath(): AnyFilePath {
		return this;
	}

	public isAbsolute(): this is AbsoluteFilePath {
		return true;
	}

	public assertAbsolute(): AbsoluteFilePath {
		return this;
	}

	public getChain(): AbsoluteFilePath[] {
		if (this.chain !== undefined) {
			return this.chain;
		}

		const paths: AbsoluteFilePath[] = [];
		this.chain = paths;

		// We use getParent here so we can reuse as much memoized information as possible
		let target: AbsoluteFilePath = this;
		while (true) {
			paths.push(target);
			if (target.isRoot()) {
				break;
			} else {
				target = target.getParent();
			}
		}

		return paths;
	}

	public resolve(other: string | AnyFilePath): AbsoluteFilePath;
	public resolve(other: AnyPath): Exclude<AnyPath, RelativePath>;
	public resolve(
		other: string | AnyPath,
	): AbsoluteFilePath | Exclude<AnyPath, RelativePath> {
		if (typeof other === "string") {
			other = createFilePath(other);
		}
		if (!other.isRelative()) {
			return other;
		}

		return new AbsoluteFilePath(
			parsePathSegments(
				[...this.getSegments(), ...other.getSegments()],
				"absolute",
				{
					explicitDirectory: other.isExplicitDirectory(),
				},
			),
		);
	}

	public relative(
		otherRaw: AbsoluteFilePath | RelativePath,
	): AbsoluteFilePath | RelativePath {
		const other = this.resolve(otherRaw);

		if (other.equal(this)) {
			return createRelativePath(".");
		}

		const absolute = this.getSegments().slice();
		const relative = other.getSegments().slice();

		// Impossible to relativize two absolute paths with different roots
		if (absolute[0] !== relative[0]) {
			return other;
		}

		// Remove common starting segments
		while (absolute[0] === relative[0]) {
			absolute.shift();
			relative.shift();
		}

		let finalSegments: PathSegments = [];
		for (let i = 0; i < absolute.length; i++) {
			finalSegments.push("..");
		}
		finalSegments = finalSegments.concat(relative);

		return new RelativePath(parsePathSegments(finalSegments, "relative"));
	}

	public format(cwd?: AbsoluteFilePath): string {
		const filename = this.join();
		const names: string[] = [];
		names.push(filename);

		// Get a path relative to HOME
		if (this.isFilePath() && this.isRelativeTo(HOME_PATH)) {
			// Path starts with the home directory, so let's trim it off
			const relativeToHome = HOME_PATH.relative(this._assert());

			// Add tilde and push it as a possible name
			// We construct this manually to get around the segment normalization which would explode ~
			names.push(
				new RelativePath({
					hint: "relative",
					segments: ["~", ...relativeToHome.getSegments()],
					absoluteType: "posix",
					absoluteTarget: undefined,
					explicitDirectory: this.parsed.explicitDirectory,
					explicitRelative: false,
				}).join(),
			);
		}

		// Get a path relative to the cwd
		if (cwd !== undefined) {
			names.push(cwd.relative(this).join());
		}

		// Get the shortest name
		const human = names.sort((a, b) => a.length - b.length)[0];
		if (human === "") {
			return "./";
		} else {
			return human;
		}
	}
}
