/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AbsoluteVersionNode,
	ComparatorNode,
	ComparatorOperator,
	LogicalAndNode,
	LogicalOrNode,
	RangeNode,
	Tokens,
	VersionNode,
	VersionPrereleaseParts,
	VersionRangeNode,
	WildcardNode,
} from './types';
import {
	ParserOptions,
	TokenValues,
	createParser,
	isAlpha,
	isDigit,
} from '@romejs/parser-core';

import {Number0, ob1Add, ob1Get0} from '@romejs/ob1';
import {descriptions} from '@romejs/diagnostics';

type ParseMode = 'version' | 'range';

export type SemverParserOptions = ParserOptions & {
	loose?: boolean;
};

const createSemverParser = createParser((ParserCore) =>
	class SemverParser extends ParserCore<Tokens, void> {
		constructor({loose, ...opts}: SemverParserOptions, mode: ParseMode) {
			super(opts, 'parse/semver');
			this.input = this.input.trimRight();
			this.mode = mode;
			this.loose = loose === undefined ? false : loose;
		}

		loose: boolean;
		mode: ParseMode;

		// For some reason Flow will throw an error without the type casts...
		tokenize(index: Number0, input: string): undefined | TokenValues<Tokens> {
			const char = input[ob1Get0(index)];
			const nextChar = input[ob1Get0(index) + 1];

			if (
				(char === '<' && nextChar === '=') ||
				(char === '>' && nextChar === '=') ||
				(char === '~' && nextChar === '>')
			) {
				// @ts-ignore: TS doesn't infer the possible combinations
				const value: ComparatorOperator = char + nextChar;
				return this.finishValueToken('Operator', value, ob1Add(index, 2));
			}

			if (
				char === '^' ||
				char === '<' ||
				char === '>' ||
				char === '~' ||
				char === '='
			) {
				const op: ComparatorOperator = char;
				return this.finishValueToken('Operator', op);
			}

			if (char === '|' && nextChar === '|') {
				return this.finishToken('Pipe', ob1Add(index, 2));
			}

			if (char === '*') {
				return this.finishToken('Star');
			}

			if (input[ob1Get0(index) - 1] === ' ' && char === '-' && nextChar === ' ') {
				return this.finishToken('RangeDash');
			}

			if (char === '-') {
				return this.finishToken('Dash');
			}

			if (char === '+') {
				return this.finishToken('Plus');
			}

			if (char === '.') {
				return this.finishToken('Dot');
			}

			if (isDigit(char)) {
				const [value] = this.readInputFrom(index, isDigit);
				return this.finishValueToken(
					'Number',
					Number(value),
					ob1Add(index, value.length),
				);
			}

			if (isAlpha(char)) {
				const [value] = this.readInputFrom(index, isAlpha);
				return this.finishValueToken('Word', value, ob1Add(index, value.length));
			}

			if (char === ' ' || char === '\t') {
				return this.finishToken('Space');
			}

			// Unknown character
			return undefined;
		}

		// Remove all subsequent space tokens
		eatSpaceToken() {
			while (this.eatToken('Space') !== undefined) {
				// empty
			}
		}

		parseVersionOrWildcard(): WildcardNode | VersionNode {
			const startPos = this.getPosition();
			const startToken = this.getToken();
			const version = this.parseVersion();

			// We should return a bare wildcard when parsed in a version position if there was nothing else attached
			if (
				this.isWildcardToken(startToken) &&
				version.minor === undefined &&
				version.patch === undefined &&
				version.prerelease.length === 0 &&
				version.build.length === 0
			) {
				return {
					type: 'Wildcard',
					loc: this.finishLoc(startPos),
				};
			}

			return version;
		}

		parseVersion(): VersionNode {
			const startPos = this.getPosition();
			const startToken = this.getToken();

			if (this.isVersionCharacter(startToken)) {
				this.nextToken();
			}

			const major = this.parseVersionNumber();
			let minor = undefined;
			let patch = undefined;

			if (this.eatToken('Dot')) {
				minor = this.parseVersionNumber();
			} else if (this.mode === 'version') {
				throw this.unexpected({
					description: descriptions.SEMVER.MISSING_MINOR_VERSION,
				});
			}

			if (this.eatToken('Dot')) {
				patch = this.parseVersionNumber();
			} else if (this.mode === 'version') {
				throw this.unexpected({
					description: descriptions.SEMVER.MISSING_PATCH_VERSION,
				});
			}

			if (this.matchToken('Dot')) {
				throw this.unexpected({
					description: descriptions.SEMVER.EXCESSIVE_VERSION_PARTS,
				});
			}

			// The dash is optional in loose mode. eg. 1.2.3pre
			let prerelease: VersionPrereleaseParts = [];
			if (this.eatToken('Dash') || (this.loose && this.matchToken('Word'))) {
				prerelease = this.parseVersionQualifierParts();
			}

			let build: VersionPrereleaseParts = [];
			if (this.eatToken('Plus')) {
				build = this.parseVersionQualifierParts();
			}

			if (major !== undefined && minor !== undefined && patch !== undefined) {
				return {
					type: 'AbsoluteVersion',
					loc: this.finishLoc(startPos),
					major,
					minor,
					patch,
					prerelease,
					build,
				};
			} else {
				return {
					type: 'WildcardVersion',
					loc: this.finishLoc(startPos),
					major,
					minor,
					patch,
					prerelease,
					build,
				};
			}
		}

		parseVersionQualifierParts(): VersionPrereleaseParts {
			const parts: VersionPrereleaseParts = [];
			do {
				parts.push(this.parseVersionQualifierPart());
			} while (this.eatToken('Dot') !== undefined);
			return parts;
		}

		parseVersionQualifierPart(): string | number {
			const parts: Array<string | number> = [];

			do {
				const token = this.getToken();

				if (token.type === 'Number' || token.type === 'Word') {
					this.nextToken();
					parts.push(token.value);
				} else if (token.type === 'Dash') {
					this.nextToken();
					parts.push('-');
				} else {
					throw this.unexpected({
						description: descriptions.SEMVER.INVALID_QUANTIFIER_PART,
					});
				}
			} while (
				this.matchToken('Number') ||
				this.matchToken('Word') ||
				this.matchToken('Dash')
			);

			if (parts.length === 1 && typeof parts[0] === 'number') {
				return parts[0];
			} else {
				return parts.join('');
			}
		}

		isWildcardToken(token: TokenValues<Tokens>): boolean {
			if (token.type === 'Star') {
				return true;
			}

			if (token.type === 'Word') {
				return token.value === 'x' || token.value === 'X';
			}

			return false;
		}

		parseVersionNumber(): undefined | number {
			const token = this.getToken();

			if (token.type === 'Number') {
				this.nextToken();
				return token.value;
			}

			if (this.isWildcardToken(token)) {
				if (this.mode === 'version') {
					throw this.unexpected({
						description: descriptions.SEMVER.WILDCARD_IN_VERSION,
					});
				}

				this.nextToken();
			} else {
				throw this.unexpected({
					description: descriptions.SEMVER.INVALID_VERSION_NUMBER,
				});
			}

			return undefined;
		}

		parseLogicalOr(left: RangeNode): LogicalOrNode {
			this.nextToken();
			this.eatSpaceToken();

			const right = this.parseExpression();
			return {
				loc: this.finishLoc(this.getLoc(left).start),
				type: 'LogicalOr',
				left,
				right,
			};
		}

		validateRangeSide(node: RangeNode): VersionNode | WildcardNode {
			// In loose mode, we allow ranges to be a bare wildcard instead of a version
			// eg. * - 1.2.3
			if (node.type === 'WildcardVersion' || node.type === 'AbsoluteVersion') {
				return node;
			}

			if (node.type === 'Wildcard' && this.loose) {
				return node;
			}

			throw this.unexpected({
				...descriptions.SEMVER.INVALID_RANGE,
				start: this.getLoc(node).start,
			});
		}

		parseVersionRange(left: RangeNode): VersionRangeNode {
			this.nextToken();
			this.eatSpaceToken();

			const right = this.parseVersionOrWildcard();

			return {
				type: 'VersionRange',
				loc: this.finishLoc(this.getLoc(left).start),
				left: this.validateRangeSide(left),
				right: this.validateRangeSide(right),
			};
		}

		parseWildcard(): WildcardNode {
			const startPos = this.getPosition();
			this.nextToken();
			return {type: 'Wildcard', loc: this.finishLoc(startPos)};
		}

		parseAtomOperator(token: Tokens['Operator']): ComparatorNode {
			const startPos = this.getPosition();
			this.nextToken();
			this.eatSpaceToken();

			const version = this.parseVersionOrWildcard();

			return {
				type: 'Comparator',
				loc: this.finishLoc(startPos),
				operator: token.value,
				version,
			};
		}

		isVersionCharacter(token: TokenValues<Tokens>): boolean {
			if (this.loose && token.type === 'Word') {
				return token.value === 'v';
			}

			return false;
		}

		parseAtomStartPipe() {
			if (this.loose) {
				// A bare pipe in an atom start position is treated the same as a wildcard...
				// Why...? Because node-semver allows it lol
				// > satisfies('1.2.3', '||') === true
				return this.parseWildcard();
			} else {
				throw this.unexpected({
					description: descriptions.SEMVER.BARE_PIPE_WITHOUT_LOOSE,
				});
			}
		}

		parseAtomStartWord(token: Tokens['Word']) {
			if (this.isWildcardToken(token)) {
				return this.parseWildcard();
			} else if (this.isVersionCharacter(token)) {
				return this.parseVersion();
			} else {
				throw this.unexpected({
					description: descriptions.SEMVER.UNEXPECTED_WORD(token.value),
				});
			}
		}

		parseAtom() {
			const token = this.getToken();

			switch (token.type) {
				case 'Number':
					return this.parseVersion();

				case 'Operator':
					return this.parseAtomOperator(token);

				case 'Star':
					return this.parseWildcard();

				case 'Pipe':
					return this.parseAtomStartPipe();

				case 'Word':
					return this.parseAtomStartWord(token);

				default:
					throw this.unexpected({
						description: descriptions.SEMVER.UNKNOWN_START,
					});
			}
		}

		parseLogicalAnd(left: RangeNode): LogicalAndNode {
			const right = this.parseExpression();

			return {
				type: 'LogicalAnd',
				left,
				right,
				loc: {
					filename: this.filename,
					start: this.getLoc(left).start,
					end: this.getLoc(right).end,
				},
			};
		}

		parseExpression(): RangeNode {
			const left = this.parseAtom();
			this.eatSpaceToken();

			if (this.matchToken('RangeDash')) {
				return this.parseVersionRange(left);
			}

			if (this.matchToken('Pipe')) {
				return this.parseLogicalOr(left);
			}

			if (!this.matchToken('EOF')) {
				return this.parseLogicalAnd(left);
			}

			return left;
		}

		parseInitialRange(): RangeNode {
			// Allow spaces at the beginning, spaces at the end have been removed by the trimRight in the constructor
			this.eatSpaceToken();

			// Empty string is an implicit wildcard in loose mode
			if (this.matchToken('EOF') && this.loose) {
				return this.parseWildcard();
			}

			const expr = this.parseExpression();
			this.finalize();

			return expr;
		}

		parseInitialVersion(): AbsoluteVersionNode {
			const node = this.parseInitialRange();

			// Verify the return value in version mode
			if (node.type !== 'AbsoluteVersion') {
				throw this.unexpected({
					...descriptions.SEMVER.EXPECTED_VERSION,
					start: this.getLoc(node).start,
				});
			}

			return node;
		}
	}
);

export function parseSemverRange(opts: SemverParserOptions): RangeNode {
	return createSemverParser(opts, 'range').parseInitialRange();
}

export function parseSemverVersion(
	opts: SemverParserOptions,
): AbsoluteVersionNode {
	return createSemverParser(opts, 'version').parseInitialVersion();
}
