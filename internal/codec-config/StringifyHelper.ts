import {Consumer} from "@internal/consume";
import {ConfigCommentMap, PathComments} from "./types";

type StringifyOptions = {
	comments: ConfigCommentMap;
	isTopLevel: boolean;
	level: number;
	stack: Set<unknown>;
	prevIndent: string;
	indent: string;
};

export function createStringifyHelper(
	comments: ConfigCommentMap,
): StringifyHelper {
	return new StringifyHelper({
		prevIndent: "",
		indent: "\t",
		comments,
		isTopLevel: true,
		level: 0,
		stack: new Set(),
	});
}

export default class StringifyHelper {
	constructor(opts: StringifyOptions) {
		this.options = opts;
	}

	public options: StringifyOptions;

	public getComments(consumer: Consumer): PathComments {
		const comments = this.options.comments.get(consumer.keyPath.join("."));
		if (comments === undefined) {
			return {
				inner: [],
				outer: [],
			};
		} else {
			return comments;
		}
	}

	public wrap(open: string, elems: string[], close: string): string {
		if (elems.length === 0) {
			return `${open}${close}`;
		} else {
			const inner = elems.map((str) => {
				return `${this.options.indent}${str}`;
			}).join("\n");
			return `${open}\n${inner}\n${this.options.prevIndent}${close}`;
		}
	}

	public fork(increment: boolean = true): StringifyHelper {
		const {comments, level, stack, indent, prevIndent} = this.options;

		return new StringifyHelper({
			comments,
			indent: increment ? `\t${indent}` : indent,
			prevIndent: increment ? indent : prevIndent,
			isTopLevel: false,
			level: increment ? level + 1 : level,
			stack,
		});
	}
}
