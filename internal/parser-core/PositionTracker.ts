import {OneIndexed, ZeroIndexed} from "@internal/math";
import {Position} from "./types";
import {pretty} from "@internal/pretty-format";
import {derivePositionKey} from "./utils";
import {ExtendedMap} from "@internal/collections";
import {AnyPath, equalPaths} from "@internal/path";

type GetPosition = () => Position;

export default class PositionTracker {
	constructor(
		{
			path,
			input,
			offsetPosition = {
				line: new OneIndexed(),
				column: new ZeroIndexed(),
			},
			getPosition,
		}: {
			path: undefined | AnyPath;
			input: string;
			offsetPosition?: Position;
			getPosition?: GetPosition;
		},
	) {
		this.getPosition = getPosition;
		this.path = path;
		this.input = input;

		this.latestPosition = offsetPosition;

		this.positionsToIndex = new ExtendedMap("positionsToIndex");
		this.positionsToIndex.set(derivePositionKey(offsetPosition), 0);

		this.cachedPositions = new Map();
	}

	private path: undefined | AnyPath;
	private input: string;
	private latestPosition: Position;
	public cachedPositions: Map<number, Position>;
	private positionsToIndex: ExtendedMap<string, number>;
	private getPosition: undefined | GetPosition;

	public getIndexFromPosition(
		pos: Position,
		path: undefined | AnyPath,
	): ZeroIndexed {
		if (!equalPaths(path, this.path)) {
			throw new Error(
				pretty`PositionTracker filename mismatch. Path ${path} is different than the filename we're tracking of ${this.path}. Position: ${pos}`,
			);
		}

		return new ZeroIndexed(this.positionsToIndex.assert(derivePositionKey(pos)));
	}

	public getPositionFromIndex(index: number | ZeroIndexed): Position {
		const cached = this.cachedPositions.get(index.valueOf());
		if (cached !== undefined) {
			return cached;
		}

		let line: OneIndexed = new OneIndexed();
		let column: ZeroIndexed = new ZeroIndexed();
		let indexSearchStart: number = 0;

		// Reuse existing line information if possible
		const {latestPosition} = this;
		const latestPositionIndex = this.getIndexFromPosition(
			latestPosition,
			this.path,
		);

		const currPosition =
			this.getPosition === undefined ? undefined : this.getPosition();
		const currPositionIndex =
			currPosition === undefined
				? undefined
				: this.getIndexFromPosition(latestPosition, this.path);

		if (
			currPosition !== undefined &&
			currPositionIndex !== undefined &&
			currPositionIndex > latestPositionIndex &&
			currPositionIndex < index
		) {
			line = currPosition.line;
			column = currPosition.column;
			indexSearchStart = currPositionIndex.valueOf();
		} else if (latestPositionIndex < index) {
			line = latestPosition.line;
			column = latestPosition.column;
			indexSearchStart = latestPositionIndex.valueOf();
		}

		// Read the rest of the input until we hit the index
		for (let i = indexSearchStart; i < index.valueOf(); i++) {
			const char = this.input[i];

			if (char === "\n") {
				line = line.increment();
				column = new ZeroIndexed();
			} else {
				column = column.increment();
			}
		}

		const pos: Position = {
			line,
			column,
		};

		if (latestPosition === undefined || index > latestPositionIndex) {
			this.latestPosition = pos;
		}

		this.setPositionIndex(pos, index);
		return pos;
	}

	public setPositionIndex(pos: Position, index: number | ZeroIndexed) {
		this.positionsToIndex.set(derivePositionKey(pos), index.valueOf());
		this.cachedPositions.set(index.valueOf(), pos);
	}
}
