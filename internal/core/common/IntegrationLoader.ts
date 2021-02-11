import {normalizeManifest} from "@internal/codec-js-manifest";
import {
	SemverRangeNode,
	SemverVersionNode,
	parseSemverRange,
	satisfiesSemver,
	stringifySemver,
} from "@internal/codec-semver";
import {Consumer, consumeUnknown} from "@internal/consume";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	createAbsoluteFilePath,
} from "@internal/path";
import {json} from "@internal/codec-config";
import {readFileTextMeta} from "@internal/fs";
import {
	createSingleDiagnosticError,
	descriptions,
	provideDiagnosticAdviceForError,
} from "@internal/diagnostics";
import internalModule = require("module");

const requires: AbsoluteFilePathMap<NodeRequire> = new AbsoluteFilePathMap();

export function getRequire(path: AbsoluteFilePath): NodeRequire {
	const existing = requires.get(path);
	if (existing !== undefined) {
		return existing;
	}

	const require: NodeRequire = internalModule.createRequire
		? internalModule.createRequire(path.join())
		: internalModule.createRequireFromPath(path.join());
	requires.set(path, require);
	return require;
}

type IntegrationLoaderNormalize<Value> = (
	consumer: Consumer,
	path: AbsoluteFilePath,
	version: undefined | SemverVersionNode,
) => Value;

type IntegrationLoaderEntry<Value> = {
	version: undefined | SemverVersionNode;
	module: Value;
};

export default class IntegrationLoader<Value> {
	constructor(
		{name, range, normalize}: {
			name: string;
			range?: string;
			normalize: IntegrationLoaderNormalize<Value>;
		},
	) {
		this.loaded = new AbsoluteFilePathMap();
		this.name = name;
		this.normalize = normalize;
		this.range =
			range === undefined ? undefined : parseSemverRange({input: range});
	}

	private loaded: AbsoluteFilePathMap<IntegrationLoaderEntry<Value>>;
	private normalize: IntegrationLoaderNormalize<Value>;
	private name: string;
	private range: undefined | SemverRangeNode;

	private resolve(
		id: string,
		require: NodeRequire,
		path: AbsoluteFilePath,
	): string {
		try {
			return require.resolve(id);
		} catch (err) {
			if (err.code === "MODULE_NOT_FOUND") {
				throw createSingleDiagnosticError({
					description: descriptions.INTEGRATIONS.NOT_FOUND(this.name),
					location: {
						path,
					},
				});
			} else {
				throw err;
			}
		}
	}

	public async wrap<T>(callback: () => Promise<T>): Promise<T> {
		const beginError = new Error();
		try {
			return await callback();
		} catch (err) {
			throw provideDiagnosticAdviceForError(
				err,
				{
					description: descriptions.INTEGRATIONS.LOAD(this.name),
					cleanRelativeError: beginError,
				},
			);
		}
	}

	public async load(
		path: AbsoluteFilePath,
		cwd: AbsoluteFilePath,
	): Promise<IntegrationLoaderEntry<Value>> {
		const existing = this.loaded.get(path);
		if (existing !== undefined) {
			return existing;
		}

		// Try to resolve
		const require = getRequire(path);

		let version: undefined | SemverVersionNode = undefined;

		// Validate range against the package version field
		const expectedRange = this.range;
		if (expectedRange !== undefined) {
			const manifestPath = createAbsoluteFilePath(
				this.resolve(`${this.name}/package.json`, require, path),
			);

			const jsonConsumer = json.consumeValue(
				await readFileTextMeta(manifestPath),
			);
			const versionProp = jsonConsumer.get("version");

			const manifest = await normalizeManifest(manifestPath, jsonConsumer, []);

			if (manifest.version === undefined) {
				throw versionProp.unexpected(
					descriptions.INTEGRATIONS.MISSING_VERSION(this.name),
					{at: "none"},
				);
			}

			if (!satisfiesSemver(manifest.version, expectedRange)) {
				throw versionProp.unexpected(
					descriptions.INTEGRATIONS.UNSUPPORTED_VERSION(
						this.name,
						stringifySemver(expectedRange),
					),
					{at: "none"},
				);
			}

			version = manifest.version;
		}

		const filename = this.resolve(this.name, require, path);

		const value: unknown = await this.wrap(() => {
			return require(filename);
		});

		const consumer = consumeUnknown(value, "integration/load", this.name);
		const module = this.normalize(consumer, cwd, version);

		const entry: IntegrationLoaderEntry<Value> = {version, module};
		this.loaded.set(path, entry);
		return entry;
	}
}
