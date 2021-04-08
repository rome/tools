import {ZeroIndexed} from "@internal/numbers";
import {ParserCore, PositionTracker} from ".";
import {ComplexToken, ParserCoreReadCallback, ParserCoreTypes, SimpleToken, ValueToken, Position} from "./types";

type StringTokenNames<Types extends ParserCoreTypes> = Extract<keyof Types["tokens"], string>;

export default class TokenizerCore<Types extends ParserCoreTypes> {
  constructor(input: string, indexTracker?: PositionTracker, parser?: ParserCore<Types>) {
    this.parser = parser;
		this.input = input;
		this.indexTracker = indexTracker ?? new PositionTracker({input});
    this.start = new ZeroIndexed(0);
		this.index = new ZeroIndexed(0);
		this.indexChar = input[0];
  }

  private parser: undefined | ParserCore<Types>;
  private input: string;
	private start: ZeroIndexed;
	private indexTracker: PositionTracker;

	public index: ZeroIndexed;
	private indexChar: string;

	public isEOF(): boolean {
		return this.index.valueOf() >= this.input.length;
	}

  public setTokenStart(index: ZeroIndexed) {
		this.start = index;
		this.setIndex(index);
	}

	public setIndex(index: ZeroIndexed) {
		this.index = index;
		this.indexChar = this.input[index.valueOf()];
	}

	public getPosition(): Position {
		return this.indexTracker.getPositionFromIndex(this.index);
	}

	public startsWith(str: string): boolean {
		if (str[0] !== this.indexChar) {
			return false;
		}

		if (str.length === 1) {
			// Otherwise the other match would have failed
			return true;
		}

		const i = this.index.valueOf();
		let chunk = this.input.slice(i, i + str.length);
		if (this.parser?.impl.caseInsensitiveTokenMatches) {
			chunk = chunk.toLowerCase();
		}
		return chunk === str;
	}

	public reverse(count: number): string {
    const i = this.index.valueOf();
    const str = this.input.slice(i - count, i);
    this.setIndex(this.index.subtract(str.length));
    return str;
  }

  public take(count: number): string {
    const i = this.index.valueOf();
    const str = this.input.slice(i, i + count);
    this.setIndex(this.index.add(str.length));
    return str;
  }

  public eat<T extends string>(str: T): undefined | T {
    if (this.startsWith(str)) {
      this.setIndex(this.index.add(str.length));
      return str;
		} else {
      return undefined;
		}
  }

  public consume(str: string): boolean {
    if (this.startsWith(str)) {
      this.setIndex(this.index.add(str.length));
      return true;
		} else {
      return false;
		}
  }

	public assert(str: string): void {
		if (!this.consume(str)) {
			// TODO message
			if (this.parser === undefined) {
				throw new Error();
			} else {
				this.parser.unexpectedDiagnostic({
					index: this.index,
				});
			}
		}
	}

	public get(offset?: number): string {
		if (offset === undefined || offset === 0) {
			return this.indexChar;
		}

		const {input, index} = this;
		const i = index.valueOf() + offset;

		// Allow an overflow since we call this method to check for trailing characters
		if (i >= input.length || i < 0) {
			return "";
		} else {
			return input[i];
		}
	}

	public getRange(count: number, offset: number = 0): string {
		const {input, index} = this;
		const start = index.valueOf() + offset;
		const end = start + count;
		return input.slice(start, end);
	}

	// Read from the input until the callback returns false
	public read(callback: ParserCoreReadCallback): string {
		// Perform a quick check first
		if (!callback(this.indexChar, this.index, this.input)) {
			return "";
		}

    const {input} = this;
		let value = "";

		// Skip running the callback for the first character as we already did it above
		let first = true;

		let {index} = this;
		let i = index.valueOf();

		while (true) {
			// Stop when we get to the end of the file
			if (i >= input.length) {
        this.setIndex(index);
				return value;
			}

			const char = input[i];
			if (first || callback(char, index, input)) {
				value += char;
				i++;
				index = index.increment();
				first = false;
			} else {
				break;
			}
		}

		this.setIndex(index);
		return value;
	}

	private getEndIndex(): ZeroIndexed {
		if (this.index.equal(this.start)) {
			return this.index.increment();
		} else {
			return this.index;
		}
	}

	public finishToken<Type extends StringTokenNames<Types>>(type: Type): SimpleToken<Type> {
		return {
			type,
			start: this.start,
			end: this.getEndIndex(),
		};
	}

	public finishValueToken<Type extends StringTokenNames<Types>, Value>(
		type: Type,
		value: Value,
	): ValueToken<Type, Value> {
		return {
			type,
			value,
			start: this.start,
			end: this.getEndIndex(),
		};
	}

	public finishComplexToken<Type extends StringTokenNames<Types>, Data extends Omit<Types["tokens"][Type], "type" | "start" | "end">>(
		type: Type,
		data: Data,
	): ComplexToken<Type, Data> {
		return {
			type,
			...data,
			start: this.start,
			end: this.getEndIndex(),
		};
	}
}
