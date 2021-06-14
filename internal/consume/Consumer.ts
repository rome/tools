/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticDescriptionOptional,
	DiagnosticLocation,
	DiagnosticsError,
	catchDiagnosticsSync,
	createSingleDiagnosticsError,
	descriptions,
} from "@internal/diagnostics";
import {
	Dict,
	UnknownFunction,
	UnknownObject,
	VoidCallback,
	isPlainObject,
} from "@internal/typescript-helpers";
import {
	JSONArray,
	JSONObject,
	JSONPropertyValue,
	JSONValue,
} from "@internal/codec-config";
import {
	ConsumeContext,
	ConsumeKey,
	ConsumePath,
	ConsumePropertyDefinition,
	ConsumePropertyMetadata,
	ConsumePropertyNumberDefinition,
	ConsumePropertyPrimitiveDefinition,
	ConsumePropertyStringDefinition,
	ConsumeProtectedFunction,
	ConsumeSourceLocationRequestTarget,
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
import {StaticMarkup, markup, readMarkup} from "@internal/markup";
import {consumeUnknown} from ".";
import {prettyFormatToString} from "@internal/pretty-format";
import {enhanceNodeInspectClass} from "@internal/node";

type UnexpectedConsumerOptions = {
	loc?: SourceLocation;
	target?: ConsumeSourceLocationRequestTarget;
	at?: "suffix" | "prefix" | "none";
	atParent?: boolean;
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
		this.propertyMetadata = opts.propertyMetadata;
		this.usedNames = new Set(opts.usedNames);
		this.forkCache = new Map();
		this.forceDiagnosticTarget = opts.forceDiagnosticTarget;
		this.declared = opts.declared === true;

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

				definitions.push(def);
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

	public declareDefinition(
		partialDef:
			| Omit<ConsumePropertyStringDefinition, "objectPath" | "metadata">
			| Omit<ConsumePropertyPrimitiveDefinition, "objectPath" | "metadata">
			| Omit<ConsumePropertyNumberDefinition, "objectPath" | "metadata">,
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

		const def: ConsumePropertyDefinition = {
			...partialDef,
			objectPath: this.keyPath,
			metadata,
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

	private generateUnexpectedMessage(
		msg: StaticMarkup,
		opts: UnexpectedConsumerOptions,
	): StaticMarkup {
		const {at = "suffix", atParent = false} = opts;
		const {parent} = this;

		let target: Consumer = this;

		if (atParent) {
			if (parent === undefined) {
				// Cannot target the parent if it does not exist
				return msg;
			} else {
				target = parent;
			}
		}

		if (at === "suffix") {
			msg = markup`${msg} at <emphasis>${target.getKeyPathString()}</emphasis>`;
		} else if (at === "prefix") {
			msg = markup`<emphasis>${target.getKeyPathString()}</emphasis> ${msg}`;
		}

		return msg;
	}

	public unexpected(
		description: DiagnosticDescriptionOptional = descriptions.CONSUME.INVALID,
		opts: UnexpectedConsumerOptions = {},
	): DiagnosticsError {
		const {target = "value"} = opts;

		const {path} = this;
		let location = this.getDiagnosticLocation(target);
		const fromSource = this.wasInSource();

		const message = this.generateUnexpectedMessage(description.message, opts);
		description = {
			...description,
			message,
		};

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
				throw new Error(readMarkup(message));
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
			throw new Error(readMarkup(message));
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

		if (isPlainObject(value)) {
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

	private _normalizeValueForSet(value: unknown): unknown {
		if (value instanceof Set) {
			return Array.from(value);
		}

		if (value instanceof Map) {
			const obj: UnknownObject = {};
			for (const [key, val] of value) {
				obj[key] = val;
			}
			return obj;
		}

		return value;
	}

	public getParentValue(def?: unknown): unknown {
		if (this.parent === undefined) {
			return def;
		} else {
			return this.parent.getValue();
		}
	}

	public getValue(def?: unknown): unknown {
		if (this.exists()) {
			return this.value;
		} else {
			return def;
		}
	}

	public setValue(rawValue: unknown): this {
		const value = this._normalizeValueForSet(rawValue);
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

		for (const [key, value] of this.asMap(false, false)) {
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
						at: "suffix",
						atParent: true,
					},
				);
			}

			if (recursive) {
				value.enforceUsedProperties(type, true);
			}
		}
	}

	public asNumberString(def?: string): number {
		this.declareDefinition({
			type: "number",
			default: def,
			required: def === undefined,
		});

		const str = this.asString(def);
		const num = parseFloat(str);
		if (isNaN(num)) {
			this.unexpected(descriptions.CONSUME.EXPECTED_VALID_NUMBER);
			return 0;
		} else {
			return num;
		}
	}

	public asNumberStringOrVoid(): undefined | number {
		this.declareDefinition({
			type: "number",
			default: undefined,
			required: false,
		});

		if (this.exists()) {
			return this.asNumberString();
		} else {
			return undefined;
		}
	}

	// JSON
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

	public exists() {
		return this.value != null;
	}

	public isObject(): boolean {
		const {value} = this;
		return (
			typeof value === "object" &&
			value !== null &&
			value.constructor === Object
		);
	}

	public asUnknownObject(optional: boolean = false): UnknownObject {
		this.declareDefinition({
			type: "object",
			default: undefined,
			required: !optional,
		});

		return {
			...this.asOriginalUnknownObject(optional),
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

		if (isPlainObject(value)) {
			return Object.keys(value).length === 0;
		}

		return false;
	}

	public asOriginalUnknownObject(optional: boolean = false): UnknownObject {
		if (optional && !this.exists()) {
			return {};
		}

		const {value} = this;
		if (!isPlainObject(value)) {
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

	public asMap(optional?: boolean, markUsed = true): Map<string, Consumer> {
		this.declareDefinition({
			type: "object",
			default: undefined,
			required: !optional,
		});

		const value = this.asOriginalUnknownObject(optional);
		const map = new Map();
		for (const key in value) {
			if (markUsed) {
				this.markUsedProperty(key);
			}
			map.set(key, this.fork(key, value[key]));
		}
		return map;
	}

	public asPlainArray(optional: boolean = false): unknown[] {
		this.declareDefinition({
			type: "array",
			default: undefined,
			required: !optional,
		});

		if (optional && !this.exists()) {
			return [];
		}

		const {value} = this;

		if (!Array.isArray(value)) {
			this.unexpected(descriptions.CONSUME.EXPECTED_ARRAY);
			return [];
		}

		return [...value];
	}

	public asIterable(optional?: boolean): Iterable<Consumer> {
		const arr = this.asPlainArray(optional);

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
			default: undefined,
			required: false,
		});
		if (this.exists()) {
			return this.asDate();
		} else {
			return undefined;
		}
	}

	public asDate(def?: Date): Date {
		this.declareDefinition({
			type: "date",
			default: def,
			required: def === undefined,
		});
		const value = this.getValue(def);
		if (!(value instanceof Date)) {
			this.unexpected(descriptions.CONSUME.EXPECTED_DATE);
			return new Date();
		}
		return value;
	}

	public asBooleanOrVoid(): undefined | boolean {
		this.declareDefinition({
			type: "boolean",
			default: undefined,
			required: false,
		});
		if (this.exists()) {
			return this.asBoolean();
		} else {
			return undefined;
		}
	}

	public asBoolean(def?: boolean): boolean {
		this.declareDefinition({
			type: "boolean",
			default: def,
			required: def === undefined,
		});
		const value = this.getValue(def);
		if (typeof value !== "boolean") {
			this.unexpected(descriptions.CONSUME.EXPECTED_BOOLEAN);
			return false;
		}
		return value;
	}

	public asStringOrVoid(): undefined | string {
		this.declareDefinition({
			type: "string",
			default: undefined,
			required: false,
		});

		if (this.exists()) {
			return this.asString();
		} else {
			return undefined;
		}
	}

	public asFunction(def?: UnknownFunction): UnknownFunction {
		this.declareDefinition({
			type: "function",
			default: def,
			required: def === undefined,
		});

		const fn = this.getValue(def);

		if (typeof fn === "function") {
			return fn as UnknownFunction;
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_FUNCTION);

			return () => {
				return undefined;
			};
		}
	}

	public async asPromise(def?: PromiseLike<unknown>): Promise<Consumer> {
		let value: unknown;

		if (this.isObject()) {
			const obj = this.asOriginalUnknownObject();
			if (typeof obj.then === "function") {
				value = await obj;
			} else {
				value = obj;
			}
		} else {
			value = this.getValue(def);
		}

		return consumeUnknown(
			value,
			this.context.category,
			this.context.categoryValue,
		);
	}

	public asWrappedFunction(def?: UnknownFunction): ConsumeProtectedFunction {
		const fn = this.asFunction(def);
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

	public asString(def?: string): string {
		this.declareDefinition({
			type: "string",
			default: def,
			required: def === undefined,
		});

		const value = this.getValue(def);
		if (typeof value !== "string") {
			this.unexpected(descriptions.CONSUME.EXPECTED_STRING);
			return "";
		}
		return value;
	}

	public asStringSet<ValidValue extends string>(
		validValues: ValidValue[],
		def?: ValidValue,
	): ValidValue {
		this.declareDefinition({
			type: "string",
			default: def,
			required: def === undefined,
			allowedValues: validValues,
		});

		const value = this.asString(def);

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
			default: undefined,
			required: false,
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
			default: undefined,
			required: false,
		});
		if (this.exists()) {
			return this.asBigInt();
		} else {
			return undefined;
		}
	}

	public asBigInt(def?: number | bigint): bigint {
		this.declareDefinition({
			type: "bigint",
			default: def,
			required: def === undefined,
		});

		const value = this.getValue(def);

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
				default: undefined,
				required: false,
			},
			"path",
		);
	}

	public asURLPath(def?: string): URLPath {
		const path = this.asAnyPath(def);
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

	public asAnyPath(def?: string): Path {
		this.declareDefinition(
			{
				type: "string",
				default: def,
				required: def === undefined,
			},
			"path",
		);

		// Allow path instances
		const value = this.getValue(def);
		if (isPath(value)) {
			return value;
		}

		// Otherwise expect a string
		return createPath(this.asString(def));
	}

	public asAnyPathOrVoid(): undefined | Path {
		if (this.exists()) {
			return this.asAnyPath();
		} else {
			this._declareOptionalPath();
			return undefined;
		}
	}

	public asFilePath(def?: string): FilePath {
		const path = this.asAnyPath(def);
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
		const path = this.asAnyPath(def);
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

	public asRelativePath(def?: string): RelativePath {
		const path = this.asAnyPath(def);
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

	public asExplicitRelativePath(def?: string): RelativePath {
		const path = this.asRelativePath(def);

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
			default: undefined,
			required: false,
		});

		if (this.exists()) {
			return this.asNumber();
		} else {
			return undefined;
		}
	}

	public asZeroIndexedNumber(def?: number): ZeroIndexed {
		return new ZeroIndexed(this.asNumber(def));
	}

	public asOneIndexedNumber(def?: number): OneIndexed {
		return new OneIndexed(this.asNumber(def));
	}

	public asZeroIndexedNumberOrVoid(): undefined | ZeroIndexed {
		const num = this.asNumberOrVoid();
		return num === undefined ? undefined : new ZeroIndexed(num);
	}

	public asOneIndexedNumberOrVoid(): undefined | OneIndexed {
		const num = this.asNumberOrVoid();
		return num === undefined ? undefined : new OneIndexed(num);
	}

	public asNumber(def?: number): number {
		this.declareDefinition({
			type: "number",
			default: def,
			required: def === undefined,
		});

		let value = this.getValue(def);
		if (isIndexedNumberish(value)) {
			value = value.valueOf();
		}
		if (typeof value !== "number") {
			this.unexpected(descriptions.CONSUME.EXPECTED_NUMBER);
			return 0;
		}
		return value;
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
			default?: number | IndexedNumber;
		},
	): number | IndexedNumber {
		const min = opts.min === undefined ? undefined : opts.min.valueOf();
		const max = opts.max === undefined ? undefined : opts.max.valueOf();
		const def = opts.default === undefined ? undefined : opts.default.valueOf();

		this.declareDefinition({
			type: "number",
			default: def,
			required: def !== undefined,
			min,
			max,
		});

		const num = this.asNumber(def);

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
		def?: ValidValue,
	): ValidValue {
		this.declareDefinition({
			type: "number",
			default: def,
			required: def === undefined,
			allowedValues: validValues,
		});

		const value = this.asNumber(def);

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
			default: undefined,
			required: false,
			allowedValues: validValues,
		});

		if (this.exists()) {
			return this.asNumberSet(validValues);
		} else {
			return undefined;
		}
	}

	public asUnknown(): unknown {
		return this.value;
	}

	// rome-ignore lint/ts/noExplicitAny: future cleanup
	public asAny(): any {
		return this.value;
	}
}

enhanceNodeInspectClass(
	Consumer,
	(consumer) => {
		return `Consumer<${prettyFormatToString(consumer.getValue())}>`;
	},
);
