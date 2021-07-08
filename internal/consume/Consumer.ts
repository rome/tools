/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticLocation,
	DiagnosticsError,
	catchDiagnostics,
	catchDiagnosticsSync,
	createSingleDiagnosticsError,
	descriptions,
} from "@internal/diagnostics";
import {
	AsyncVoidCallback,
	Dict,
	UnknownFunction,
	UnknownObject,
	VoidCallback,
	isObject,
} from "@internal/typescript-helpers";
import {
	ConsumeContext,
	ConsumeKey,
	ConsumePath,
	ConsumePropertyDefinition,
	ConsumePropertyDefinitionBase,
	ConsumePropertyMetadata,
	ConsumePropertyNumberDefinition,
	ConsumePropertyPrimitiveDefinition,
	ConsumePropertyStringDefinition,
	ConsumeProtectedFunction,
	ConsumeSourceLocationRequestTarget,
	ConsumeUnexpectedDescription,
	ConsumerHandleUnexpected,
	ConsumerMapCallback,
	ConsumerOnDefinition,
	ConsumerOptions,
} from "./types";
import {SourceLocation, UNKNOWN_POSITION} from "@internal/parser-core";
import {
	IndexedNumber,
	OneIndexed,
	ZeroIndexed,
	isIndexedNumberish,
} from "@internal/numbers";
import {isValidIdentifierName} from "@internal/js-ast-utils";
import {escapeJSString} from "@internal/string-escape";
import {
	AbsoluteFilePath,
	FilePath,
	Path,
	RelativePath,
	URLPath,
	createAbsoluteFilePath,
	createPath,
	createRelativePath,
	createURLPath,
	isPath,
} from "@internal/path";
import {markup, readMarkup} from "@internal/markup";
import {consumeUnknown} from ".";
import {prettyFormatToString} from "@internal/pretty-format";
import {enhanceNodeInspectClass} from "@internal/node";
import {
	JSONArray,
	JSONObject,
	JSONPropertyValue,
	JSONValue,
} from "@internal/codec-config";

type UnexpectedConsumerOptions = {
	loc?: SourceLocation;
	target?: ConsumeSourceLocationRequestTarget;
	suffix?: boolean;
};

function isComputedPart(part: ConsumeKey): boolean {
	return typeof part === "number" || !isValidIdentifierName(part);
}

export default class Consumer {
	constructor(opts: ConsumerOptions) {
		this.path = opts.path;
		this.value = opts.value;
		this.parent = opts.parent;
		this.keyPath = opts.objectPath;
		this.context = opts.context;
		this.onDefinition = opts.onDefinition;
		this.usedNames = new Set(opts.usedNames);
		this.forkCache = new Map();
		this.forceDiagnosticTarget = opts.forceDiagnosticTarget;

		this.propertyMetadata = opts.propertyMetadata;
		this.declared = opts.declared === true;
		this.isRequired = false;
		this.defaultValue = undefined;

		// See shouldDispatchUnexpected for explanation
		this.hasHandledUnexpected = false;
		this.handleUnexpected = opts.handleUnexpectedDiagnostic;
	}

	public path: Path;

	private declared: boolean;
	private handleUnexpected: undefined | ConsumerHandleUnexpected;
	private onDefinition: undefined | ConsumerOnDefinition;
	private propertyMetadata: undefined | ConsumePropertyMetadata;
	private parent: undefined | Consumer;
	private value: unknown;
	private defaultValue: unknown;
	private isRequired: boolean;
	private context: ConsumeContext;
	public keyPath: ConsumePath;
	private usedNames: Set<string>;
	private forkCache: Map<string, Consumer>;
	private hasHandledUnexpected: boolean;
	private forceDiagnosticTarget: undefined | ConsumeSourceLocationRequestTarget;

	public capture(): {
		consumer: Consumer;
		definitions: ConsumePropertyDefinition[];
		diagnostics: Diagnostic[];
	} {
		let diagnostics: Diagnostic[] = [];
		const definitions: ConsumePropertyDefinition[] = [];

		const consumer = this.cloneConsumer({
			onDefinition: (def, consumer) => {
				if (this.onDefinition !== undefined) {
					this.onDefinition(def, consumer);
				}

				definitions.push();
			},
			handleUnexpectedDiagnostic(diag) {
				diagnostics.push(diag);
			},
		});
		return {consumer, definitions, diagnostics};
	}

	public async bufferDiagnostics<T>(
		callback: (consumer: Consumer) => Promise<T> | T,
	): Promise<T> {
		const {diagnostics, consumer} = await this.capture();
		const result = await callback(consumer);
		if (result === undefined || diagnostics.length > 0) {
			throw new DiagnosticsError("Captured diagnostics", diagnostics);
		}
		return result;
	}

	// Just for JSON.stringify debugging of Consumer instances
	public toJSON() {
		return this.value;
	}

	public handleThrownDiagnostics(callback: VoidCallback) {
		if (this.handleUnexpected === undefined) {
			callback();
		} else {
			const {diagnostics} = catchDiagnosticsSync(callback);

			if (diagnostics !== undefined) {
				for (const diag of diagnostics) {
					this.handleUnexpected(diag);
				}
			}
		}
	}

	public async handleAsyncThrownDiagnostics(callback: AsyncVoidCallback) {
		if (this.handleUnexpected === undefined) {
			await callback();
		} else {
			const {diagnostics} = await catchDiagnostics(async () => {
				await callback();
			});
			if (diagnostics !== undefined) {
				for (const diag of diagnostics) {
					this.handleUnexpected(diag);
				}
			}
		}
	}

	public declareDefinition(
		partialDef:
			| Omit<
					ConsumePropertyStringDefinition,
					keyof ConsumePropertyDefinitionBase
				>
			| Omit<
					ConsumePropertyPrimitiveDefinition,
					keyof ConsumePropertyDefinitionBase
				>
			| Omit<
					ConsumePropertyNumberDefinition,
					keyof ConsumePropertyDefinitionBase
				>,
		inputName?: string,
	) {
		if (this.declared) {
			return;
		}

		if (this.onDefinition === undefined) {
			return;
		}

		const metadata: ConsumePropertyMetadata = {
			inputName,
			...this.propertyMetadata,
		};

		const base: ConsumePropertyDefinitionBase = {
			default: this.defaultValue,
			required: this.isRequired,
			objectPath: this.keyPath,
			metadata,
		};

		const def: ConsumePropertyDefinition = {
			...partialDef,
			...base,
		};

		this.declared = true;

		this.onDefinition(def, this);
	}

	public getDiagnosticLocation(
		target: ConsumeSourceLocationRequestTarget = "all",
	): DiagnosticLocation {
		const {forceDiagnosticTarget} = this;
		if (forceDiagnosticTarget !== undefined) {
			target = forceDiagnosticTarget;
		}

		let getPropertyDiagnosticLocation = this.propertyMetadata?.getDiagnosticLocation;
		if (getPropertyDiagnosticLocation !== undefined) {
			const loc = getPropertyDiagnosticLocation(target);
			if (loc !== undefined) {
				return loc;
			}
		}

		const getDiagnosticLocation = this.context.getDiagnosticLocation;
		if (getDiagnosticLocation !== undefined) {
			const loc = getDiagnosticLocation(this.keyPath, target);
			if (loc !== undefined) {
				return loc;
			}
		}

		return {
			path: this.path,
		};
	}

	public getLocation(
		target?: ConsumeSourceLocationRequestTarget,
	): SourceLocation {
		const location = this.getDiagnosticLocation(target);
		if (
			location === undefined ||
			location.start === undefined ||
			location.end === undefined
		) {
			return {
				path: this.path,
				start: UNKNOWN_POSITION,
				end: UNKNOWN_POSITION,
			};
		} else {
			return {
				path: location.path,
				start: location.start,
				end: location.end,
			};
		}
	}

	public getLocationRange(
		startIndex: ZeroIndexed,
		endIndex: ZeroIndexed = startIndex,
		target?: ConsumeSourceLocationRequestTarget,
	): SourceLocation {
		const loc = this.getLocation(target);
		if (loc.start === UNKNOWN_POSITION) {
			return loc;
		}

		const {start, end} = loc;

		// We don't support handling line differences here... yet?
		if (start.line !== end.line) {
			return loc;
		}

		return {
			...loc,
			start: {
				...start,
				column: start.column.add(startIndex),
			},
			end: {
				...start,
				column: start.column.add(endIndex),
			},
		};
	}

	public getKey(): Consumer {
		return this.cloneConsumer({
			forceDiagnosticTarget: "key",
			value: this.getParentKey(),
		});
	}

	public getParentKey(): ConsumeKey {
		return this.keyPath[this.keyPath.length - 1];
	}

	public hasChangedFromSource(): boolean {
		const {getOriginalValue} = this.context;
		if (getOriginalValue === undefined) {
			return false;
		}

		const originalValue = getOriginalValue(this.keyPath);
		return !this.wasInSource() || this.value !== originalValue;
	}

	public wasInSource(): boolean {
		const loc = this.getDiagnosticLocation();
		return loc.start !== undefined || loc.end !== undefined;
	}

	public getKeyPathString(path: ConsumePath = this.keyPath): string {
		const {normalizeKey} = this.context;
		let str = "";

		for (let i = 0; i < path.length; i++) {
			const prevPart = path[i - 1];
			let part = path[i];
			const nextPart = path[i + 1];

			if (typeof part === "string" && normalizeKey !== undefined) {
				part = normalizeKey(part);
			}

			if (prevPart !== undefined && isComputedPart(prevPart)) {
				str += ".";
			}

			// If we are a computed property then wrap in brackets, the previous part would not have inserted a dot
			// We allow a computed part at the beginning of a path
			if (isComputedPart(part)) {
				const inner =
					typeof part === "number"
						? String(part)
						: escapeJSString(
								part,
								{
									quote: '"',
								},
							);

				str += `[${inner}]`;
			} else {
				if (nextPart === undefined || isComputedPart(nextPart)) {
					// Don't append a dot if there are no parts or the next is computed
					str += part;
				} else {
					str += `${part}.`;
				}
			}
		}

		return str;
	}

	public unexpected(
		description: ConsumeUnexpectedDescription = descriptions.CONSUME.INVALID,
		opts: UnexpectedConsumerOptions = {},
	): DiagnosticsError {
		const {suffix = true, target = "value"} = opts;

		const {path} = this;
		let location = this.getDiagnosticLocation(target);
		const fromSource = this.wasInSource();

		let keyPath: undefined | string = this.getKeyPathString();
		if (this.keyPath.length === 0) {
			keyPath = undefined;
		}

		if (typeof description === "function") {
			description = description(keyPath);
		} else if (suffix && keyPath !== undefined) {
			description = {
				...description,
				message: markup`${description.message} at <emphasis>${keyPath}</emphasis>`,
			};
		}

		const advice: DiagnosticAdvice[] = [...(description.advice || [])];

		// Make the errors more descriptive
		if (fromSource) {
			if (this.hasChangedFromSource()) {
				advice.push({
					type: "log",
					category: "warn",
					text: markup`Our internal value has been modified since we read the original source`,
				});
			}
		} else {
			// Go up the consumer tree and take the position from the first consumer found in the source
			let consumer: undefined | Consumer = this;
			do {
				if (consumer.wasInSource()) {
					location = consumer.getDiagnosticLocation(target);
					break;
				}
				consumer = consumer.parent;
			} while (consumer !== undefined);

			// If consumer is undefined and we have no filename then we were not able to find a location,
			// in this case, just throw a normal error
			if (consumer === undefined && path === undefined) {
				throw new Error(readMarkup(description.message));
			}

			// Warn that we didn't find this value in the source if it's parent wasn't either
			if (this.parent === undefined || !this.parent.wasInSource()) {
				advice.push({
					type: "log",
					category: "warn",
					text: markup`This value was expected to be found at <emphasis>${this.getKeyPathString()}</emphasis> but was not in the original source`,
				});
			}
		}

		if (opts.loc !== undefined) {
			location = opts.loc;
		}

		if (location === undefined) {
			throw new Error(readMarkup(description.message));
		}

		const diagnostic: Diagnostic = {
			description: {
				category: this.context.category,
				categoryValue: this.context.categoryValue,
				...description,
				advice,
			},
			location: {
				...location,
				path: this.path,
			},
		};

		const err = createSingleDiagnosticsError(diagnostic);

		if (this.handleUnexpected === undefined) {
			throw err;
		} else {
			if (this.shouldDispatchUnexpected()) {
				this.handleUnexpected(diagnostic);
				this.hasHandledUnexpected = true;
			}

			// Still allow throwing the diagnostic
			return err;
		}
	}

	// Only dispatch a single error for the current consumer, and suppress any if we have a parent consumer with errors
	// We do this since we could be producing redundant stale errors based on
	// results we've normalized to allow us to continue
	private shouldDispatchUnexpected(): boolean {
		if (this.hasHandledUnexpected) {
			return false;
		}

		const {parent} = this;
		if (parent !== undefined) {
			return parent.shouldDispatchUnexpected();
		}

		return true;
	}

	private cloneConsumer(opts: Partial<ConsumerOptions>): Consumer {
		return new Consumer({
			declared: this.declared,
			usedNames: this.usedNames,
			onDefinition: this.onDefinition,
			handleUnexpectedDiagnostic: this.handleUnexpected,
			path: this.path,
			context: this.context,
			value: this.value,
			parent: this.parent,
			objectPath: this.keyPath,
			propertyMetadata: this.propertyMetadata,
			...opts,
		});
	}

	public copy(mix?: object) {
		let value = this.value;

		if (isObject(value)) {
			value = {...this.asOriginalUnknownObject()};
		}

		if (Array.isArray(value)) {
			value = [...value];
		}

		if (mix !== undefined) {
			Object.assign(value, mix);
		}

		const consumer = this.cloneConsumer({
			value,
		});

		// Add on cached property metadata if necessary
		for (const [key, value] of this.forkCache) {
			if (value.propertyMetadata !== undefined) {
				consumer.get(key, value.propertyMetadata);
			}
		}

		return consumer;
	}

	private fork(
		key: ConsumeKey,
		value: unknown,
		propertyMetadata?: ConsumePropertyMetadata,
	) {
		// We require this cache as we sometimes want to store state about a forked property such as used items
		const cached = this.forkCache.get(String(key));
		if (
			!!cached &&
			cached.value === value &&
			(cached.propertyMetadata === undefined ||
			cached.propertyMetadata === propertyMetadata)
		) {
			return cached;
		}

		const forked = this.cloneConsumer({
			propertyMetadata,
			value,
			parent: this,
			objectPath: [...this.keyPath, key],
		});
		this.forkCache.set(String(key), forked);
		return forked;
	}

	public getParentValue(): unknown {
		if (this.parent === undefined) {
			return undefined;
		} else {
			return this.parent.asUnknown();
		}
	}

	public required(def?: unknown): this {
		this.defaultValue = def;

		if (!this.exists() && def === undefined) {
			this.unexpected(descriptions.CONSUME.REQUIRED);
		}

		return this;
	}

	public setValue(value: unknown): this {
		this.value = value;

		// If we're at the root (as indicated by the lack of these properties) then go no where else
		const {parent, keyPath} = this;
		if (parent === undefined || keyPath.length === 0) {
			return this;
		}

		// Validate the parent is an object
		const parentValue = parent.asUnknown();
		if (
			parentValue === undefined ||
			parentValue === null ||
			typeof parentValue !== "object"
		) {
			throw parent.unexpected(descriptions.CONSUME.SET_PROPERTY_NON_OBJECT);
		}

		// Mutate the parent
		const parentObj = parent.asOriginalUnknownObject();
		const key = this.getParentKey();
		parentObj[String(key)] = value;
		parent.setValue(parentObj);

		return this;
	}

	public has(key: string): boolean {
		const value = this.asOriginalUnknownObject();
		return value[key] != null;
	}

	public delete(key: string) {
		this.get(key).setValue(undefined);
	}

	public set(key: string, value: unknown) {
		this.get(key).setValue(value);
	}

	public get(key: string, metadata?: ConsumePropertyMetadata): Consumer {
		const value = this.asOriginalUnknownObject();
		const valueKey =
			metadata?.alternateName && this.has(metadata.alternateName)
				? metadata.alternateName
				: key;
		this.markUsedProperty(valueKey);
		return this.fork(key, value[valueKey], metadata);
	}

	public getPath(keys: (string | number)[]): Consumer {
		let target: Consumer = this;
		for (const key of keys) {
			if (typeof key === "number") {
				target = target.getIndex(key);
			} else {
				target = target.get(key);
			}
		}
		return target;
	}

	public getIndex(index: number): Consumer {
		const arr = this.asPlainArray();
		return this.fork(index, arr[index]);
	}

	public markUsedProperty(name: string) {
		this.usedNames.add(name);
	}

	public enforceUsedProperties(
		type: string = "property",
		recursive: boolean = true,
	) {
		if (!this.isObject()) {
			return;
		}

		let knownProperties = Array.from(this.usedNames.keys());

		const {normalizeKey} = this.context;
		if (normalizeKey !== undefined) {
			knownProperties = knownProperties.map((key) => normalizeKey(key));
		}

		for (const [key, value] of this.asMap(false)) {
			if (!this.usedNames.has(key)) {
				const normalizedKey = normalizeKey ? normalizeKey(key) : key;
				value.unexpected(
					descriptions.CONSUME.UNUSED_PROPERTY(
						normalizedKey,
						type,
						knownProperties,
					),
					{
						target: "key",
						suffix: false,
					},
				);
			}

			if (recursive) {
				value.enforceUsedProperties(type, true);
			}
		}
	}

	public deriveBooleanString(): Consumer {
		this.declareDefinition({
			type: "boolean",
		});

		if (!this.exists()) {
			return this.cloneConsumer({
				value: undefined,
			});
		}

		if (typeof this.asUnknown() === "boolean") {
			return this;
		}

		const str = this.asString();

		if (str === "0" || str === "false" || str === "False") {
			return this.cloneConsumer({
				value: false,
			});
		}

		if (str === "1" || str === "true" || str === "True") {
			return this.cloneConsumer({
				value: true,
			});
		}

		return this.cloneConsumer({
			value: str,
		});
	}

	public deriveNumberString(): Consumer {
		this.declareDefinition({
			type: "number",
		});

		if (!this.exists()) {
			return this.cloneConsumer({
				value: undefined,
			});
		}

		const str = this.asString();
		const num = parseFloat(str);
		return this.cloneConsumer({
			value: num,
		});
	}

	public exists() {
		return this.value != null;
	}

	public isObject(): boolean {
		return isObject(this.asUnknown());
	}

	public asUnknownObject(): UnknownObject {
		this.declareDefinition({
			type: "object",
		});

		return {
			...this.asOriginalUnknownObject(),
		};
	}

	public isEmpty(): boolean {
		const value = this.asUnknown();

		if (value == null) {
			return true;
		}

		if (value === "") {
			return true;
		}

		if (isObject(value)) {
			return Object.keys(value).length === 0;
		}

		if (Array.isArray(value)) {
			return value.length === 0;
		}

		return false;
	}

	public asOriginalUnknownObject(): UnknownObject {
		const value = this.asUnknown();

		if (!isObject(value)) {
			this.unexpected(descriptions.CONSUME.EXPECTED_OBJECT);
			return {};
		}

		return value;
	}

	public asMappedObject<T>(callback: (c: Consumer, key: string) => T): Dict<T> {
		const obj: Dict<T> = {};
		for (const [key, value] of this.asMap()) {
			obj[key] = callback(value, key);
		}
		return obj;
	}

	public asMap(markUsed: boolean = true): Map<string, Consumer> {
		this.declareDefinition({
			type: "object",
		});

		const value = this.asOriginalUnknownObject();
		const map = new Map();
		for (const key in value) {
			if (markUsed) {
				this.markUsedProperty(key);
			}
			map.set(key, this.fork(key, value[key]));
		}
		return map;
	}

	public asPlainArray(): unknown[] {
		this.declareDefinition({
			type: "array",
		});

		const value = this.asUnknown();

		if (!Array.isArray(value)) {
			this.unexpected(descriptions.CONSUME.EXPECTED_ARRAY);
			return [];
		}

		return [...value];
	}

	public asIterable(): Iterable<Consumer> {
		const arr = this.asPlainArray();

		return arr.map((val, index) => {
			return this.fork(index, val);
		});
	}

	public asMappedArray<T>(callback: ConsumerMapCallback<T>): T[] {
		return Array.from(this.asIterable(), callback);
	}

	public asImplicitArray(): Consumer[] {
		return this.asImplicitMappedArray((c) => c);
	}

	public asImplicitMappedArray<T>(callback: ConsumerMapCallback<T>): T[] {
		if (Array.isArray(this.asUnknown())) {
			return this.asMappedArray(callback);
		} else if (this.exists()) {
			return [callback(this, 0)];
		} else {
			return [];
		}
	}

	public asDateOrVoid(): undefined | Date {
		this.declareDefinition({
			type: "date",
		});

		if (this.exists()) {
			return this.asDate();
		} else {
			return undefined;
		}
	}

	public asDate(): Date {
		this.declareDefinition({
			type: "date",
		});

		const value = this.asUnknown();

		if (value instanceof Date) {
			return value;
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_DATE);
			return new Date();
		}
	}

	public asBooleanOrVoid(): undefined | boolean {
		this.declareDefinition({
			type: "boolean",
		});

		if (this.exists()) {
			return this.asBoolean();
		} else {
			return undefined;
		}
	}

	public asBoolean(): boolean {
		this.declareDefinition({
			type: "boolean",
		});

		const value = this.asUnknown();

		if (typeof value === "boolean") {
			return value;
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_BOOLEAN);
			return false;
		}
	}

	public asStringOrVoid(): undefined | string {
		this.declareDefinition({
			type: "string",
		});

		if (this.exists()) {
			return this.asString();
		} else {
			return undefined;
		}
	}

	public asFunction(): UnknownFunction {
		this.declareDefinition({
			type: "function",
		});

		const fn = this.asUnknown();

		if (typeof fn === "function") {
			return fn as UnknownFunction;
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_FUNCTION);

			return () => {
				return undefined;
			};
		}
	}

	public async asPromise(): Promise<Consumer> {
		let value: unknown;

		if (isObject(this.value)) {
			const obj = this.asOriginalUnknownObject();
			if (typeof obj.then === "function") {
				value = await obj;
			} else {
				value = obj;
			}
		} else {
			value = this.asUnknown();
		}

		return consumeUnknown(
			value,
			this.context.category,
			this.context.categoryValue,
		);
	}

	public asWrappedFunction(): ConsumeProtectedFunction {
		const fn = this.asFunction();
		const context = this.getParentValue();

		return (...args) => {
			const ret = fn.apply(context, args);
			return consumeUnknown(
				ret,
				this.context.category,
				this.context.categoryValue,
			);
		};
	}

	public asString(): string {
		this.declareDefinition({
			type: "string",
		});

		const value = this.asUnknown();

		if (typeof value === "string") {
			return value;
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_STRING);
			return "";
		}
	}

	public asStringSet<ValidValue extends string>(
		validValues: ValidValue[],
	): ValidValue {
		this.declareDefinition({
			type: "string",
			allowedValues: validValues,
		});

		const value = this.asString();

		// @ts-expect-error
		if (validValues.includes(value)) {
			// @ts-expect-error
			return value;
		} else {
			this.unexpected(
				descriptions.CONSUME.INVALID_STRING_SET_VALUE(
					value,
					// rome-ignore lint/ts/noExplicitAny: future cleanup
					(validValues as any) as string[],
				),
				{
					target: "value",
				},
			);
			return validValues[0];
		}
	}

	public asStringSetOrVoid<ValidValue extends string>(
		validValues: ValidValue[],
	): undefined | ValidValue {
		this.declareDefinition({
			type: "string",
			allowedValues: validValues,
		});

		if (this.exists()) {
			return this.asStringSet(validValues);
		} else {
			return undefined;
		}
	}

	public asBigIntOrVoid(): undefined | bigint {
		this.declareDefinition({
			type: "bigint",
		});

		if (this.exists()) {
			return this.asBigInt();
		} else {
			return undefined;
		}
	}

	public asBigInt(): bigint {
		this.declareDefinition({
			type: "bigint",
		});

		const value = this.asUnknown();

		if (typeof value === "number") {
			return BigInt(value);
		}

		if (typeof value === "bigint") {
			return value;
		}

		this.unexpected(descriptions.CONSUME.EXPECTED_BIGINT);
		return BigInt("0");
	}

	private _declareOptionalPath() {
		this.declareDefinition(
			{
				type: "string",
			},
			"path",
		);
	}

	public asURLPath(): URLPath {
		const path = this.asAnyPath();
		if (path.isURL()) {
			return path.assertURL();
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_URL);
			return createURLPath("unknown://").append(...path.getSegments());
		}
	}

	public asURLPathOrVoid(): undefined | URLPath {
		if (this.exists()) {
			return this.asURLPath();
		} else {
			this._declareOptionalPath();
			return undefined;
		}
	}

	public asAnyPath(): Path {
		this.declareDefinition(
			{
				type: "string",
			},
			"path",
		);

		const value = this.asUnknown();

		// Allow path instances
		if (isPath(value)) {
			return value;
		}

		// Otherwise expect a string
		return createPath(this.asString());
	}

	public asAnyPathOrVoid(): undefined | Path {
		if (this.exists()) {
			return this.asAnyPath();
		} else {
			this._declareOptionalPath();
			return undefined;
		}
	}

	public asFilePath(): FilePath {
		const path = this.asAnyPath();
		if (path.isFilePath()) {
			return path.assertFilePath();
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_FILE_PATH);
			return createRelativePath("unknown");
		}
	}

	public asFilePathOrVoid(): undefined | FilePath {
		const path = this.asAnyPath();
		if (path.isFilePath()) {
			return path.assertFilePath();
		} else {
			this._declareOptionalPath();
			return undefined;
		}
	}

	public asAbsoluteFilePath(
		def?: string,
		cwd?: AbsoluteFilePath,
	): AbsoluteFilePath {
		const path = this.asAnyPath();
		if (path.isAbsolute()) {
			return path.assertAbsolute();
		} else if (cwd !== undefined && path.isRelative()) {
			return cwd.resolve(path);
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_ABSOLUTE_PATH);
			return createAbsoluteFilePath("/").append(...path.getSegments());
		}
	}

	public asAbsoluteFilePathOrVoid(
		cwd?: AbsoluteFilePath,
	): undefined | AbsoluteFilePath {
		if (this.exists()) {
			return this.asAbsoluteFilePath(undefined, cwd);
		} else {
			this._declareOptionalPath();
			return undefined;
		}
	}

	public asRelativePath(): RelativePath {
		const path = this.asAnyPath();
		if (path.isRelative()) {
			return path.assertRelative();
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_RELATIVE_PATH);
			return createRelativePath("unknown");
		}
	}

	public asRelativePathOrVoid(): undefined | RelativePath {
		if (this.exists()) {
			return this.asRelativePath();
		} else {
			this._declareOptionalPath();
			return undefined;
		}
	}

	public asExplicitRelativePath(): RelativePath {
		const path = this.asRelativePath();

		if (path.isExplicitRelative()) {
			return path;
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_EXPLICIT_RELATIVE_PATH);
			return path.toExplicitRelative();
		}
	}

	public asExplicitRelativePathOrVoid(): undefined | RelativePath {
		if (this.exists()) {
			return this.asExplicitRelativePath();
		} else {
			this._declareOptionalPath();
			return undefined;
		}
	}

	public asNumberOrVoid(): undefined | number {
		this.declareDefinition({
			type: "number",
		});

		if (this.exists()) {
			return this.asNumber();
		} else {
			return undefined;
		}
	}

	public asZeroIndexedNumber(): ZeroIndexed {
		return new ZeroIndexed(this.asNumber());
	}

	public asOneIndexedNumber(): OneIndexed {
		return new OneIndexed(this.asNumber());
	}

	public asZeroIndexedNumberOrVoid(): undefined | ZeroIndexed {
		const num = this.asNumberOrVoid();
		return num === undefined ? undefined : new ZeroIndexed(num);
	}

	public asOneIndexedNumberOrVoid(): undefined | OneIndexed {
		const num = this.asNumberOrVoid();
		return num === undefined ? undefined : new OneIndexed(num);
	}

	public asNumber(): number {
		this.declareDefinition({
			type: "number",
		});

		let value = this.asUnknown();

		if (isIndexedNumberish(value)) {
			value = value.valueOf();
		}

		if (typeof value === "number") {
			return value;
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_NUMBER);
			return 0;
		}
	}

	public asNumberInRange(
		opts: {
			min?: number;
			max?: number;
			default?: number;
		},
	): number;

	public asNumberInRange(
		opts: {
			min: OneIndexed;
			max?: OneIndexed;
			default?: OneIndexed;
		},
	): OneIndexed;

	public asNumberInRange(
		opts: {
			min: OneIndexed;
			max?: OneIndexed;
			default?: OneIndexed;
		},
	): OneIndexed;

	public asNumberInRange(
		opts: {
			min?: number | IndexedNumber;
			max?: number | IndexedNumber;
		},
	): number | IndexedNumber {
		const min = opts.min === undefined ? undefined : opts.min.valueOf();
		const max = opts.max === undefined ? undefined : opts.max.valueOf();

		this.declareDefinition({
			type: "number",
			min,
			max,
		});

		const num = this.asNumber();

		// Nice error message when both min and max are specified
		if (min !== undefined && max !== undefined && (num < min || num > max)) {
			this.unexpected(descriptions.CONSUME.EXPECTED_NUMBER_BETWEEN(min, max));
		} else {
			if (min !== undefined && num < min) {
				this.unexpected(descriptions.CONSUME.EXPECTED_NUMBER_HIGHER(min));
			}

			if (max !== undefined && num > max) {
				this.unexpected(descriptions.CONSUME.EXPECTED_NUMBER_LOWER(max));
			}
		}

		if (opts.min instanceof OneIndexed) {
			return new OneIndexed(num);
		}

		if (opts.min instanceof ZeroIndexed) {
			return new ZeroIndexed(num);
		}

		return num;
	}

	public asNumberSet<ValidValue extends number>(
		validValues: ValidValue[],
	): ValidValue {
		this.declareDefinition({
			type: "number",
			allowedValues: validValues,
		});

		const value = this.asNumber();

		// @ts-expect-error
		if (validValues.includes(value)) {
			// @ts-expect-error
			return value;
		} else {
			this.unexpected(
				descriptions.CONSUME.INVALID_NUMBER_SET_VALUE(
					value,
					// rome-ignore lint/ts/noExplicitAny: future cleanup
					(validValues as any) as number[],
				),
				{
					target: "value",
				},
			);
			return validValues[0];
		}
	}

	public asNumberSetOrVoid<ValidValue extends number>(
		validValues: ValidValue[],
	): undefined | ValidValue {
		this.declareDefinition({
			type: "number",
			allowedValues: validValues,
		});

		if (this.exists()) {
			return this.asNumberSet(validValues);
		} else {
			return undefined;
		}
	}

	public asJSONValue(): JSONValue {
		const {value} = this;

		switch (typeof value) {
			case "number":
			case "string":
			case "boolean":
				return value;
		}

		if (value === null) {
			return null;
		}

		if (Array.isArray(value)) {
			return this.asJSONArray();
		}

		if (this.isObject()) {
			return this.asJSONObject();
		}

		this.unexpected(descriptions.CONSUME.EXPECTED_JSON_VALUE);
		return "";
	}

	public asJSONArray(): JSONArray {
		const arr: JSONArray = [];
		for (const value of this.asIterable()) {
			arr.push(value.asJSONValue());
		}
		return arr;
	}

	public asJSONObject(): JSONObject {
		const obj: JSONObject = {};
		for (const [key, value] of this.asMap()) {
			obj[key] = value.asJSONPropertyValue();
		}
		return obj;
	}

	public asJSONPropertyValue(): JSONPropertyValue {
		if (this.exists()) {
			return this.asJSONValue();
		} else {
			return undefined;
		}
	}

	public asUnknown(): unknown {
		if (this.exists()) {
			return this.value;
		} else {
			return this.defaultValue;
		}
	}

	// rome-ignore lint/ts/noExplicitAny: future cleanup
	public asAny(): any {
		return this.asUnknown();
	}
}

enhanceNodeInspectClass(
	Consumer,
	(consumer) => {
		return `Consumer<${prettyFormatToString(consumer.asUnknown())}>`;
	},
);
