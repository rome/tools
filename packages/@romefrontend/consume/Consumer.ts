/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticDescriptionOptionalCategory,
	DiagnosticLocation,
	Diagnostics,
	DiagnosticsError,
	catchDiagnosticsSync,
	createBlessedDiagnosticMessage,
	createSingleDiagnosticError,
	descriptions,
} from "@romefrontend/diagnostics";
import {UnknownObject, isPlainObject} from "@romefrontend/typescript-helpers";
import {
	JSONArray,
	JSONObject,
	JSONPropertyValue,
	JSONValue,
} from "@romefrontend/codec-json";
import {
	ConsumeContext,
	ConsumeKey,
	ConsumePath,
	ConsumePropertyDefinition,
	ConsumePropertyMetadata,
	ConsumePropertyNumberDefinition,
	ConsumePropertyPrimitiveDefinition,
	ConsumePropertyStringDefinition,
	ConsumeSourceLocationRequestTarget,
	ConsumerHandleUnexpected,
	ConsumerOnDefinition,
	ConsumerOptions,
} from "./types";
import {SourceLocation, UNKNOWN_POSITION} from "@romefrontend/parser-core";
import {
	Number0,
	Number1,
	UnknownNumber,
	ob1Add,
	ob1Coerce0,
	ob1Coerce1,
	ob1Get,
} from "@romefrontend/ob1";
import {isValidIdentifierName} from "@romefrontend/js-ast-utils";
import {escapeJSString} from "@romefrontend/string-escape";
import {
	AbsoluteFilePath,
	RelativeFilePath,
	URLFilePath,
	UnknownFilePath,
	createAbsoluteFilePath,
	createURLFilePath,
	createUnknownFilePath,
} from "@romefrontend/path";

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
		this.path = opts.filePath;
		this.filename = this.path === undefined ? undefined : this.path.join();

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

	path: undefined | UnknownFilePath;
	filename: undefined | string;
	declared: boolean;
	handleUnexpected: undefined | ConsumerHandleUnexpected;
	onDefinition: undefined | ConsumerOnDefinition;
	propertyMetadata: undefined | ConsumePropertyMetadata;
	parent: undefined | Consumer;
	value: unknown;
	context: ConsumeContext;
	keyPath: ConsumePath;
	usedNames: Set<string>;
	forkCache: Map<string, Consumer>;
	hasHandledUnexpected: boolean;
	forceDiagnosticTarget: undefined | ConsumeSourceLocationRequestTarget;

	capture(): {
		consumer: Consumer;
		definitions: Array<ConsumePropertyDefinition>;
		diagnostics: Diagnostics;
	} {
		let diagnostics: Diagnostics = [];
		const definitions: Array<ConsumePropertyDefinition> = [];

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

	async bufferDiagnostics<T>(
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
	toJSON() {
		return this.value;
	}

	handleThrownDiagnostics(callback: () => void) {
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

	declareDefinition(
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

	getDiagnosticLocation(
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
		if (getDiagnosticLocation === undefined) {
			return {};
		} else {
			return getDiagnosticLocation(this.keyPath, target);
		}
	}

	getLocation(target?: ConsumeSourceLocationRequestTarget): SourceLocation {
		const location = this.getDiagnosticLocation(target);
		if (
			location === undefined ||
			location.start === undefined ||
			location.end === undefined
		) {
			return {
				filename: this.filename,
				start: UNKNOWN_POSITION,
				end: UNKNOWN_POSITION,
			};
		} else {
			return {
				filename: location.filename,
				start: location.start,
				end: location.end,
			};
		}
	}

	getLocationRange(
		startIndex: Number0,
		endIndex: Number0 = startIndex,
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
				column: ob1Add(start.column, startIndex),
				index: ob1Add(start.index, startIndex),
			},
			end: {
				...start,
				column: ob1Add(start.column, endIndex),
				index: ob1Add(start.index, endIndex),
			},
		};
	}

	getKey(): Consumer {
		return this.cloneConsumer({
			forceDiagnosticTarget: "key",
			value: this.getParentKey(),
		});
	}

	getParentKey(): ConsumeKey {
		return this.keyPath[this.keyPath.length - 1];
	}

	hasChangedFromSource(): boolean {
		const {getOriginalValue} = this.context;
		if (getOriginalValue === undefined) {
			return false;
		}

		const originalValue = getOriginalValue(this.keyPath);
		return !this.wasInSource() || this.value !== originalValue;
	}

	wasInSource() {
		return this.getDiagnosticLocation() !== undefined;
	}

	getKeyPathString(path: ConsumePath = this.keyPath): string {
		const {normalizeKey} = this.context;
		let str = "";

		for (let i = 0; i < path.length; i++) {
			let part = path[i];
			const nextPart = path[i + 1];

			if (typeof part === "string" && normalizeKey !== undefined) {
				part = normalizeKey(part);
			}

			// If we are a computed property then wrap in brackets, the previous part would not have inserted a dot
			// We allow a computed part at the beginning of a path
			if (isComputedPart(part) && i > 0) {
				const inner =
					typeof part === "number"
						? String(part)
						: escapeJSString(
								part,
								{
									quote: "'",
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

	generateUnexpectedMessage(
		msg: string,
		opts: UnexpectedConsumerOptions,
	): string {
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
			msg += ` at <emphasis>${target.getKeyPathString()}</emphasis>`;
		} else {
			msg = `<emphasis>${target.getKeyPathString()}</emphasis> ${msg}`;
		}

		return msg;
	}

	unexpected(
		description: DiagnosticDescriptionOptionalCategory = descriptions.CONSUME.INVALID,
		opts: UnexpectedConsumerOptions = {},
	): DiagnosticsError {
		const {target = "value"} = opts;

		const {filename} = this;
		let location = this.getDiagnosticLocation(target);
		const fromSource = location !== undefined;

		const message = this.generateUnexpectedMessage(
			description.message.value,
			opts,
		);
		description = {
			...description,
			message: createBlessedDiagnosticMessage(message),
		};

		const advice: DiagnosticAdvice = [...(description.advice || [])];

		// Make the errors more descriptive
		if (fromSource) {
			if (this.hasChangedFromSource()) {
				advice.push({
					type: "log",
					category: "warn",
					text: "Our internal value has been modified since we read the original source",
				});
			}
		} else {
			// Go up the consumer tree and take the position from the first consumer found in the source
			let consumer: undefined | Consumer = this;
			do {
				const possibleLocation = consumer.getDiagnosticLocation(target);
				if (possibleLocation !== undefined) {
					location = possibleLocation;
					break;
				}
				consumer = consumer.parent;
			} while (consumer !== undefined);

			// If consumer is undefined and we have no filename then we were not able to find a location,
			// in this case, just throw a normal error
			if (consumer === undefined && filename === undefined) {
				throw new Error(message);
			}

			// Warn that we didn't find this value in the source if it's parent wasn't either
			if (this.parent === undefined || !this.parent.wasInSource()) {
				advice.push({
					type: "log",
					category: "warn",
					text: `This value was expected to be found at <emphasis>${this.getKeyPathString()}</emphasis> but was not in the original source`,
				});
			}
		}

		if (opts.loc !== undefined) {
			location = opts.loc;
		}

		if (location === undefined) {
			throw new Error(message);
		}

		const diagnostic: Diagnostic = {
			description: {
				category: this.context.category,
				...description,
				advice,
			},
			location: {
				...location,
				filename: this.filename,
			},
		};

		const err = createSingleDiagnosticError(diagnostic);

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
	shouldDispatchUnexpected(): boolean {
		if (this.hasHandledUnexpected) {
			return false;
		}

		const {parent} = this;
		if (parent !== undefined) {
			return parent.shouldDispatchUnexpected();
		}

		return true;
	}

	cloneConsumer(opts: Partial<ConsumerOptions>): Consumer {
		return new Consumer({
			declared: this.declared,
			usedNames: this.usedNames,
			onDefinition: this.onDefinition,
			handleUnexpectedDiagnostic: this.handleUnexpected,
			filePath: this.path,
			context: this.context,
			value: this.value,
			parent: this.parent,
			objectPath: this.keyPath,
			propertyMetadata: this.propertyMetadata,
			...opts,
		});
	}

	copy(mix?: object) {
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

	fork(
		key: ConsumeKey,
		value: unknown,
		propertyMetadata?: ConsumePropertyMetadata,
	) {
		// We require this cache as we sometimes want to store state about a forked property such as used items
		const cached = this.forkCache.get(String(key));
		if (
			cached !== undefined &&
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

	_normalizeValueForSet(value: unknown): unknown {
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

	getValue(def?: unknown): unknown {
		if (this.exists()) {
			return this.value;
		} else {
			return def;
		}
	}

	setValue(rawValue: unknown): this {
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

	has(key: string): boolean {
		const value = this.asOriginalUnknownObject();
		return value[key] != null;
	}

	delete(key: string) {
		this.get(key).setValue(undefined);
	}

	set(key: string, value: unknown) {
		this.get(key).setValue(value);
	}

	get(key: string, metadata?: ConsumePropertyMetadata): Consumer {
		const value = this.asOriginalUnknownObject();
		this.markUsedProperty(key);
		return this.fork(key, value[key], metadata);
	}

	markUsedProperty(name: string) {
		this.usedNames.add(name);
	}

	enforceUsedProperties(type: string = "property", recursive: boolean = true) {
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
				value.unexpected(
					descriptions.CONSUME.UNUSED_PROPERTY(
						this.getKeyPathString([key]),
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

	asPossibleNumberString(def?: number): Consumer {
		this.declareDefinition({
			type: "number",
			default: def,
			required: def === undefined,
		});

		if (this.exists()) {
			const str = this.asUndeclaredString();
			const num = parseFloat(str);
			if (isNaN(num)) {
				this.unexpected(descriptions.CONSUME.EXPECTED_VALID_NUMBER);
			} else {
				return this.cloneConsumer({
					value: num,
				});
			}
		}

		return this.cloneConsumer({
			value: def,
		});
	}

	asPossibleParsedJSON(): Consumer {
		if (typeof this.asUnknown() === "string") {
			return this.cloneConsumer({
				value: JSON.parse(this.asString()),
			});
		} else {
			return this;
		}
	}

	// JSON
	asJSONValue(): JSONValue {
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

	asJSONArray(): JSONArray {
		const arr: JSONArray = [];
		for (const value of this.asArray()) {
			arr.push(value.asJSONValue());
		}
		return arr;
	}

	asJSONObject(): JSONObject {
		const obj: JSONObject = {};
		for (const [key, value] of this.asMap()) {
			obj[key] = value.asJSONPropertyValue();
		}
		return obj;
	}

	asJSONPropertyValue(): JSONPropertyValue {
		if (this.exists()) {
			return this.asJSONValue();
		} else {
			return undefined;
		}
	}

	exists() {
		return this.value != null;
	}

	isObject(): boolean {
		const {value} = this;
		return (
			typeof value === "object" &&
			value !== null &&
			value.constructor === Object
		);
	}

	asUnknownObject(optional: boolean = false): UnknownObject {
		this.declareDefinition({
			type: "object",
			default: undefined,
			required: !optional,
		});

		return {
			...this.asOriginalUnknownObject(optional),
		};
	}

	asOriginalUnknownObject(optional: boolean = false): UnknownObject {
		if (optional === true && !this.exists()) {
			return {};
		}

		const {value} = this;
		if (!isPlainObject(value)) {
			this.unexpected(descriptions.CONSUME.EXPECTED_OBJECT);
			return {};
		}

		return value;
	}

	asMap(optional?: boolean, markUsed = true): Map<string, Consumer> {
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

	asPlainArray(optional: boolean = false): Array<unknown> {
		this.declareDefinition({
			type: "array",
			default: undefined,
			required: !optional,
		});

		if (optional === true && !this.exists()) {
			return [];
		}

		const {value} = this;

		if (!Array.isArray(value)) {
			this.unexpected(descriptions.CONSUME.EXPECTED_ARRAY);
			return [];
		}

		return [...value];
	}

	asArray(optional?: boolean): Array<Consumer> {
		const arr = this.asPlainArray(optional);

		return arr.map((val, index) => {
			return this.fork(index, val);
		});
	}

	asImplicitArray(): Array<Consumer> {
		if (Array.isArray(this.asUnknown())) {
			return this.asArray();
		} else if (this.exists()) {
			return [this];
		} else {
			return [];
		}
	}

	asDateOrVoid(): undefined | Date {
		this.declareDefinition({
			type: "date",
			default: undefined,
			required: false,
		});
		if (this.exists()) {
			return this.asUndeclaredDate();
		} else {
			return undefined;
		}
	}

	asDate(def?: Date): Date {
		this.declareDefinition({
			type: "date",
			default: def,
			required: def === undefined,
		});
		return this.asUndeclaredDate(def);
	}

	asUndeclaredDate(def?: Date): Date {
		const value = this.getValue(def);
		if (!(value instanceof Date)) {
			this.unexpected(descriptions.CONSUME.EXPECTED_DATE);
			return new Date();
		}
		return value;
	}

	asBooleanOrVoid(): undefined | boolean {
		this.declareDefinition({
			type: "boolean",
			default: undefined,
			required: false,
		});
		if (this.exists()) {
			return this.asUndeclaredBoolean();
		} else {
			return undefined;
		}
	}

	asBoolean(def?: boolean): boolean {
		this.declareDefinition({
			type: "boolean",
			default: def,
			required: def === undefined,
		});
		return this.asUndeclaredBoolean(def);
	}

	asUndeclaredBoolean(def?: boolean): boolean {
		const value = this.getValue(def);
		if (typeof value !== "boolean") {
			this.unexpected(descriptions.CONSUME.EXPECTED_BOOLEAN);
			return false;
		}
		return value;
	}

	asStringOrVoid(): undefined | string {
		this.declareDefinition({
			type: "string",
			default: undefined,
			required: false,
		});

		if (this.exists()) {
			return this.asUndeclaredString();
		} else {
			return undefined;
		}
	}

	asString(def?: string): string {
		this.declareDefinition({
			type: "string",
			default: def,
			required: def === undefined,
		});
		return this.asUndeclaredString(def);
	}

	asUndeclaredString(def?: string): string {
		const value = this.getValue(def);
		if (typeof value !== "string") {
			this.unexpected(descriptions.CONSUME.EXPECTED_STRING);
			return "";
		}
		return value;
	}

	asStringSet<ValidValue extends string>(
		validValues: Array<ValidValue>,
		def?: ValidValue,
	): ValidValue {
		this.declareDefinition({
			type: "string",
			default: def,
			required: def === undefined,
			allowedValues: validValues,
		});
		return this.asUndeclaredStringSet(validValues, def);
	}

	asUndeclaredStringSet<ValidValue extends string>(
		validValues: Array<ValidValue>,
		def?: ValidValue,
	): ValidValue {
		const value = this.asUndeclaredString(def);

		// @ts-ignore
		if (validValues.includes(value)) {
			// @ts-ignore
			return value;
		} else {
			this.unexpected(
				descriptions.CONSUME.INVALID_STRING_SET_VALUE(
					value,
					// rome-ignore lint/js/noExplicitAny
					((validValues as any) as Array<string>),
				),
				{
					target: "value",
				},
			);
			return validValues[0];
		}
	}

	asStringSetOrVoid<ValidValue extends string>(
		validValues: Array<ValidValue>,
	): undefined | ValidValue {
		this.declareDefinition({
			type: "string",
			default: undefined,
			required: false,
			allowedValues: validValues,
		});

		if (this.exists()) {
			return this.asUndeclaredStringSet(validValues);
		} else {
			return undefined;
		}
	}

	asBigIntOrVoid(): undefined | bigint {
		this.declareDefinition({
			type: "bigint",
			default: undefined,
			required: false,
		});
		if (this.exists()) {
			return this.asUndeclaredBigInt();
		} else {
			return undefined;
		}
	}

	asBigInt(def?: number | bigint): bigint {
		this.declareDefinition({
			type: "bigint",
			default: def,
			required: def === undefined,
		});
		return this.asUndeclaredBigInt(def);
	}

	asUndeclaredBigInt(def?: number | bigint): bigint {
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

	_declareOptionalFilePath() {
		this.declareDefinition(
			{
				type: "string",
				default: undefined,
				required: false,
			},
			"path",
		);
	}

	asURLFilePath(def?: string): URLFilePath {
		const path = this.asUnknownFilePath(def);
		if (path.isURL()) {
			return path.assertURL();
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_URL);
			return createURLFilePath("unknown://").append(path);
		}
	}

	asURLFilePathOrVoid(): undefined | URLFilePath {
		if (this.exists()) {
			return this.asURLFilePath();
		} else {
			this._declareOptionalFilePath();
			return undefined;
		}
	}

	asUnknownFilePath(def?: string): UnknownFilePath {
		this.declareDefinition(
			{
				type: "string",
				default: def,
				required: def === undefined,
			},
			"path",
		);

		return createUnknownFilePath(this.asUndeclaredString(def));
	}

	asUnknownFilePathOrVoid(): undefined | UnknownFilePath {
		if (this.exists()) {
			return this.asUnknownFilePath();
		} else {
			this._declareOptionalFilePath();
			return undefined;
		}
	}

	asAbsoluteFilePath(def?: string, cwd?: AbsoluteFilePath): AbsoluteFilePath {
		const path = this.asUnknownFilePath(def);
		if (path.isAbsolute()) {
			return path.assertAbsolute();
		} else if (cwd !== undefined && path.isRelative()) {
			return cwd.resolve(path);
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_ABSOLUTE_PATH);
			return createAbsoluteFilePath("/").append(path);
		}
	}

	asAbsoluteFilePathOrVoid(cwd?: AbsoluteFilePath): undefined | AbsoluteFilePath {
		if (this.exists()) {
			return this.asAbsoluteFilePath(undefined, cwd);
		} else {
			this._declareOptionalFilePath();
			return undefined;
		}
	}

	asRelativeFilePath(def?: string): RelativeFilePath {
		const path = this.asUnknownFilePath(def);
		if (path.isRelative()) {
			return path.assertRelative();
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_RELATIVE_PATH);
			return path.toExplicitRelative();
		}
	}

	asRelativeFilePathOrVoid(): undefined | RelativeFilePath {
		if (this.exists()) {
			return this.asRelativeFilePath();
		} else {
			this._declareOptionalFilePath();
			return undefined;
		}
	}

	asExplicitRelativeFilePath(def?: string): RelativeFilePath {
		const path = this.asRelativeFilePath(def);

		if (path.isExplicitRelative()) {
			return path;
		} else {
			this.unexpected(descriptions.CONSUME.EXPECTED_EXPLICIT_RELATIVE_PATH);
			return path.toExplicitRelative();
		}
	}

	asExplicitRelativeFilePathOrVoid(): undefined | RelativeFilePath {
		if (this.exists()) {
			return this.asExplicitRelativeFilePath();
		} else {
			this._declareOptionalFilePath();
			return undefined;
		}
	}

	asNumberOrVoid(): undefined | number {
		this.declareDefinition({
			type: "number",
			default: undefined,
			required: false,
		});

		if (this.exists()) {
			return this.asUndeclaredNumber();
		} else {
			return undefined;
		}
	}

	asZeroIndexedNumber(def?: number): Number0 {
		return ob1Coerce0(this.asNumber(def));
	}

	asOneIndexedNumber(def?: number): Number1 {
		return ob1Coerce1(this.asNumber(def));
	}

	asZeroIndexedNumberOrVoid(): undefined | Number0 {
		const num = this.asNumberOrVoid();
		return num === undefined ? undefined : ob1Coerce0(num);
	}

	asOneIndexedNumberOrVoid(): undefined | Number1 {
		const num = this.asNumberOrVoid();
		return num === undefined ? undefined : ob1Coerce1(num);
	}

	asNumber(def?: number): number {
		this.declareDefinition({
			type: "number",
			default: def,
			required: def === undefined,
		});
		return this.asUndeclaredNumber(def);
	}

	asNumberInRange(
		opts: {
			min?: number;
			max?: number;
			default?: number;
		},
	): number

	asNumberInRange(
		opts: {
			min: Number0;
			max?: Number0;
			default?: Number0;
		},
	): Number0

	asNumberInRange(
		opts: {
			min: Number1;
			max?: Number1;
			default?: Number1;
		},
	): Number1

	asNumberInRange(
		opts: {
			min?: UnknownNumber;
			max?: UnknownNumber;
			default?: UnknownNumber;
		},
	): UnknownNumber {
		const num = this.asUndeclaredNumber(opts.default);
		const min = ob1Get(opts.min);
		const max = ob1Get(opts.max);

		this.declareDefinition({
			type: "number",
			default: opts.default,
			required: opts.default !== undefined,
			min,
			max,
		});

		// Nice error message when both min and max are specified
		if (min !== undefined && max !== undefined && (num < min || num > max)) {
			this.unexpected(descriptions.CONSUME.EXPECTED_NUMBER_BETWEEN(min, max));
			return num;
		}

		if (min !== undefined && num < min) {
			this.unexpected(descriptions.CONSUME.EXPECTED_NUMBER_HIGHER(min));
		}

		if (max !== undefined && num > max) {
			this.unexpected(descriptions.CONSUME.EXPECTED_NUMBER_LOWER(max));
		}

		return num;
	}

	asUndeclaredNumber(def?: UnknownNumber): number {
		const value = this.getValue(def);
		if (typeof value !== "number") {
			this.unexpected(descriptions.CONSUME.EXPECTED_NUMBER);
			return 0;
		}
		return value;
	}

	asNumberSet<ValidValue extends number>(
		validValues: Array<ValidValue>,
		def?: ValidValue,
	): ValidValue {
		this.declareDefinition({
			type: "number",
			default: def,
			required: def === undefined,
			allowedValues: validValues,
		});
		return this.asUndeclaredNumberSet(validValues, def);
	}

	asUndeclaredNumberSet<ValidValue extends number>(
		validValues: Array<ValidValue>,
		def?: ValidValue,
	): ValidValue {
		const value = this.asUndeclaredNumber(def);

		// @ts-ignore
		if (validValues.includes(value)) {
			// @ts-ignore
			return value;
		} else {
			this.unexpected(
				descriptions.CONSUME.INVALID_NUMBER_SET_VALUE(
					value,
					// rome-ignore lint/js/noExplicitAny
					((validValues as any) as Array<number>),
				),
				{
					target: "value",
				},
			);
			return validValues[0];
		}
	}

	asNumberSetOrVoid<ValidValue extends number>(
		validValues: Array<ValidValue>,
	): undefined | ValidValue {
		this.declareDefinition({
			type: "number",
			default: undefined,
			required: false,
			allowedValues: validValues,
		});

		if (this.exists()) {
			return this.asUndeclaredNumberSet(validValues);
		} else {
			return undefined;
		}
	}

	asUnknown(): unknown {
		return this.value;
	}

	// rome-ignore lint/js/noExplicitAny
	asAny(): any {
		return this.value;
	}
}
