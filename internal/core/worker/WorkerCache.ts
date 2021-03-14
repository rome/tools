import {VERSION} from "@internal/core";
import {AbsoluteFilePath, AbsoluteFilePathMap, UIDPath} from "@internal/path";
import {FSStats} from "@internal/fs";
import Cache from "../common/Cache";
import Worker from "./Worker";
import {
	RSERValue,
	decodeSingleMessageRSERStream,
	hashRSERValue,
} from "@internal/binary-transport";
import {FileReference} from "../common/types/files";
import {Consumer, consumeUnknown} from "@internal/consume";
import {sha256} from "@internal/string-utils";
import {
	DIAGNOSTIC_CATEGORIES,
	DiagnosticIntegrity,
} from "@internal/diagnostics";
import {markup} from "@internal/markup";
import {isPlainObject} from "@internal/typescript-helpers";

// This can be shared across multiple machines safely
export type PortableCacheMetadata = {
	version: string;
	cacheKey: string;
	size: bigint;
	hash: string;
};

export type PortableCacheMetadataHashless = Omit<PortableCacheMetadata, "hash">;

// Contains file-system specific information specific to the system it's on
export type LocalCacheMetadata = {
	mtime: bigint;
	ctime: bigint;
	mode: bigint;
};

export type CacheKey = {
	ref: FileReference;
	name: string;
};

type CacheEntryLoader<Value extends RSERValue> = {
	key: string;
	validate: (consumer: Consumer) => Value;
};

export function createCacheEntryLoader<Value extends RSERValue>(
	key: string,
	validate: CacheEntryLoader<Value>["validate"],
): CacheEntryLoader<Value> {
	return {
		key,
		validate,
	};
}

const portableMetaLoader = createCacheEntryLoader<PortableCacheMetadata>(
	"meta-portable",
	(c) => {
		return {
			version: c.get("version").asString(),
			cacheKey: c.get("cacheKey").asString(),
			hash: c.get("hash").asString(),
			size: c.get("size").asBigInt(),
		};
	},
);

const localMetaLoader = createCacheEntryLoader<LocalCacheMetadata>(
	"meta-local",
	(c) => {
		return {
			ctime: c.get("ctime").asBigInt(),
			mtime: c.get("mtime").asBigInt(),
			mode: c.get("mode").asBigInt(),
		};
	},
);

type CacheKeyParts = RSERValue[];

function serializeCacheKey(rawParts: CacheKeyParts): string {
	const parts: string[] = [];
	for (const part of rawParts) {
		if (typeof part === "string") {
			parts.push(part);
		} else if (!isPlainObject(part) || Object.keys(part).length > 0) {
			parts.push(hashRSERValue(part));
		}
	}
	return parts.join("-");
}

export class CacheEntry<Value extends RSERValue = RSERValue> {
	constructor(
		file: CacheFile,
		cache: Cache,
		name: string,
		loader: CacheEntryLoader<Value>,
	) {
		this.value = undefined;
		this.cache = cache;
		this.file = file;
		this.path = this.cache.getCacheFilename(file.ref.uid, name);
		this.loader = loader;
	}

	private value: undefined | Value;
	private loader: CacheEntryLoader<Value>;
	private file: CacheFile;
	private path: AbsoluteFilePath;
	private cache: Cache;

	public async load(): Promise<undefined | Value> {
		if (this.value !== undefined) {
			return this.value;
		}

		if (this.file.ignoreDisk) {
			return undefined;
		}

		if (this.cache.readDisabled) {
			return undefined;
		}

		const {path} = this;
		if (await path.notExists()) {
			return undefined;
		}

		const stream = path.createReadStream();
		const decoded = await decodeSingleMessageRSERStream(stream);

		if (decoded.type === "INCOMPATIBLE") {
			this.cache.logger.warn(markup`Incompatible cache file ${path} ignored`);
			return undefined;
		}

		const consumer = consumeUnknown(
			decoded.value,
			DIAGNOSTIC_CATEGORIES.parse,
			"rser",
		);
		const value = this.loader.validate(consumer);
		this.value = value;
		return value;
	}

	public update(value: Value) {
		this.value = value;

		if (this.file.shouldWrite) {
			this.cache.addPendingWrite(this.path, {type: "update", value});
		}
	}
}

class CacheFile {
	constructor(cache: WorkerCache, worker: Worker, ref: FileReference) {
		this.cache = cache;
		this.worker = worker;
		this.ref = ref;
		this.stats = undefined;
		this.readFile = undefined;
		this.entries = new Map();

		this.ignoreDisk = false;
		this.shouldWrite = !worker.hasBuffer(ref.real);
	}

	public ref: FileReference;
	public ignoreDisk: boolean;
	public shouldWrite: boolean;

	private readFile: undefined | string;
	private stats: undefined | FSStats;
	private worker: Worker;
	private cache: WorkerCache;
	private entries: Map<string, CacheEntry<RSERValue>>;

	public async getStats(): Promise<FSStats> {
		if (this.stats !== undefined) {
			return this.stats;
		}

		const {worker} = this;
		const path = this.ref.real;

		let stats: FSStats;
		if (worker.hasBuffer(path)) {
			stats = worker.getBufferFakeStats(path);
		} else if (worker.virtualModules.isVirtualPath(path)) {
			stats = worker.virtualModules.getFakeStats(path);
		} else {
			stats = await path.lstat();
		}

		this.stats = stats;
		return stats;
	}

	public takePossibleReadFile(): undefined | string {
		const {readFile} = this;
		if (readFile !== undefined) {
			// Clear it so we don't hold onto the string reference
			this.readFile = undefined;
		}
		return readFile;
	}

	private async getHash(): Promise<string> {
		const content = await this.worker.readFile(this.ref);

		if (typeof content === "string") {
			this.readFile = content;
			return sha256.sync(content);
		} else {
			let buff = "";
			content.on(
				"data",
				(chunk) => {
					buff += chunk.toString();
				},
			);

			const hash = await sha256.async(content);
			this.readFile = buff;
			return hash;
		}
	}

	public async isOutdated(
		{
			cachedPortable,
			cachedLocal,
			expectedLocal,
			expectedPortable,
		}: {
			cachedPortable: undefined | PortableCacheMetadata;
			cachedLocal: undefined | LocalCacheMetadata;
			expectedLocal: LocalCacheMetadata;
			expectedPortable: PortableCacheMetadataHashless;
		},
	): Promise<{
		outdated: boolean;
		hash?: string;
	}> {
		// No existing portable means the cache file does not exist
		if (cachedPortable === undefined) {
			return {outdated: true};
		}

		// Different filesize or cache key clearly indicates a mismatch
		if (
			cachedPortable.size !== expectedPortable.size ||
			cachedPortable.cacheKey !== expectedPortable.cacheKey
		) {
			return {outdated: true};
		}

		// If there is no local entry then we only have a portable. We just need to fetch and verify a matching hash.
		if (cachedLocal === undefined) {
			const hash = await this.getHash();
			return {hash, outdated: hash === cachedPortable.hash};
		} else {
			// Ignore the hash and just use our local meta
			if (
				cachedLocal.mtime !== expectedLocal.mtime ||
				cachedLocal.ctime !== expectedLocal.ctime ||
				cachedLocal.mode !== expectedLocal.mode
			) {
				return {outdated: true};
			}
		}

		return {outdated: false};
	}

	public async init() {
		const portableEntry = await this.getEntry(portableMetaLoader);
		const cachedPortable = await portableEntry.load();

		const localEntry = await this.getEntry(localMetaLoader);
		const cachedLocal = await localEntry.load();

		const expectedLocal = await this.createFreshLocalMetadata();
		const expectedPortable = await this.createFreshPortableMetadata();

		let {hash, outdated} = await this.isOutdated({
			cachedLocal,
			cachedPortable,
			expectedLocal,
			expectedPortable,
		});

		// Write the local meta if we are outdated or lacked it and validated the hash
		if (outdated || cachedLocal === undefined) {
			localEntry.update(expectedLocal);
		}

		if (outdated) {
			// This tells all derived entries to completely ignore the disk for initial load
			this.ignoreDisk = true;

			// Load hash so we can save it
			if (hash === undefined) {
				hash = await this.getHash();
			}

			// Update with proper metadata
			portableEntry.update({
				...expectedPortable,
				hash,
			});
		}
	}

	private async createFreshPortableMetadata(): Promise<PortableCacheMetadataHashless> {
		const {ref} = this;
		const project = this.worker.getProject(ref);
		const configCacheKeys: string[] = [];

		for (const key of Object.keys(project.configCacheKeys).sort()) {
			configCacheKeys.push(`${key}:${project.configCacheKeys[key]}`);
		}

		const manifest = this.worker.getPartialManifest(ref);
		if (manifest !== undefined) {
			configCacheKeys.push(`manifest:${manifest.hash}`);
		}

		const stats = await this.getStats();

		return {
			version: VERSION,
			cacheKey: configCacheKeys.join(";"),
			size: stats.size,
		};
	}

	private async createFreshLocalMetadata(): Promise<LocalCacheMetadata> {
		const stats = await this.getStats();

		return {
			mtime: stats.mtimeNs,
			ctime: stats.ctimeNs,
			mode: stats.mode,
		};
	}

	public async getEntry<Value extends RSERValue>(
		loader: CacheEntryLoader<Value>,
		parts?: CacheKeyParts,
	): Promise<CacheEntry<Value>> {
		const key =
			parts === undefined
				? loader.key
				: serializeCacheKey([loader.key, ...parts]);

		const loaded = this.entries.get(key);
		if (loaded !== undefined) {
			// @ts-ignore: Trust
			return loaded;
		}

		const entry: CacheEntry<Value> = new CacheEntry(
			this,
			this.cache,
			key,
			loader,
		);
		this.entries.set(key, entry);
		return entry;
	}
}

export default class WorkerCache extends Cache {
	constructor(worker: Worker) {
		super(
			"worker",
			{
				userConfig: worker.userConfig,
				parentLogger: worker.logger,
				fatalErrorHandler: worker.fatalErrorHandler,
				writeDisabled: worker.options.cacheWriteDisabled,
				readDisabled: worker.options.cacheReadDisabled,
			},
			worker.resources,
		);
		this.worker = worker;
		this.loadedFiles = new AbsoluteFilePathMap();
	}

	private worker: Worker;
	private loadedFiles: AbsoluteFilePathMap<CacheFile>;

	public async remove(uid: UIDPath, path: AbsoluteFilePath) {
		this.loadedFiles.delete(path);
		await super.remove(uid, path);
	}

	public async getIntegrity(
		ref: FileReference,
	): Promise<undefined | DiagnosticIntegrity> {
		if (!this.worker.isDiskSynced(ref.real)) {
			return undefined;
		}

		const entry = await this.getEntry(ref, portableMetaLoader);
		const meta = await entry.load();
		if (meta === undefined) {
			throw new Error("Portable meta should always be present");
		}

		return {
			hash: meta.hash,
		};
	}

	public async getFile(ref: FileReference): Promise<CacheFile> {
		let entry = this.loadedFiles.get(ref.real);
		if (entry === undefined) {
			entry = new CacheFile(this, this.worker, ref);
			this.loadedFiles.set(ref.real, entry);
			await entry.init();
		}
		return entry;
	}

	public async getEntry<Value extends RSERValue>(
		ref: FileReference,
		loader: CacheEntryLoader<Value>,
		parts?: CacheKeyParts,
	): Promise<CacheEntry<Value>> {
		const file = await this.getFile(ref);
		return await file.getEntry(loader, parts);
	}
}
