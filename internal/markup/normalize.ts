import {
	MarkupFormatNormalizeOptions,
	MarkupLinesAndWidth,
	MarkupParsedChildren,
	MarkupParsedTag,
} from "./types";
import {parseMarkup} from "./parse";
import {
	AnyMarkup,
	AnyMarkups,
	StaticMarkup,
	convertToMarkupFromRandomString,
	isEmptyMarkup,
	joinMarkup,
	markup,
	readMarkup,
} from "./escape";
import {sliceEscaped} from "@internal/string-utils";
import {buildFileLink, formatGrammarNumber} from "./util";
import {Dict} from "@internal/typescript-helpers";

function buildTag(
	tag: MarkupParsedTag,
	inner: AnyMarkup,
	opts: MarkupFormatNormalizeOptions,
): {
	modified: boolean;
	value: StaticMarkup;
} {
	let {attributes} = tag;

	let originalAttributes = attributes.asMappedObject((value) => {
		if (!value.exists()) {
			return undefined;
		}

		let raw = value.asUnknown();
		if (raw === true) {
			return true;
		} else {
			return String(raw);
		}
	});

	let modified = false;
	let setAttributes: Dict<undefined | string> = {};

	switch (tag.name) {
		case "filelink": {
			const {path, line, column, text} = buildFileLink(attributes, opts);
			setAttributes.target = path.join();

			if (isEmptyMarkup(inner) || opts.stripFilelinkText) {
				inner = markup`${text}`;
				modified = true;
			}

			if (opts.stripPositions) {
				setAttributes.line = undefined;
				setAttributes.column = undefined;
			} else {
				setAttributes.column = column;
				setAttributes.line = line;
			}

			break;
		}

		// We don't technically need to normalize this but it's one less tag to have to support
		// if other tools need to consume it
		case "grammarNumber":
			return {
				value: markup`${formatGrammarNumber(attributes, readMarkup(inner))}`,
				modified: true,
			};
	}

	let open = `<${tag.name}`;

	let finalAttributes = {
		...originalAttributes,
		...setAttributes,
	};
	for (const key in finalAttributes) {
		const val = finalAttributes[key];
		if (val === undefined) {
			continue;
		}

		if (val === true) {
			open += ` ${key}`;
		} else {
			open += readMarkup(markup` ${key}="${val}"`);
		}
	}

	for (let key in setAttributes) {
		if (originalAttributes[key] !== setAttributes[key]) {
			modified = true;
		}
	}

	if (isEmptyMarkup(inner)) {
		return {
			value: convertToMarkupFromRandomString(`${open} />`),
			modified,
		};
	} else {
		return {
			value: convertToMarkupFromRandomString(
				`${open}>${readMarkup(inner)}</${tag.name}>`,
			),
			modified,
		};
	}
}

function normalizeMarkupChildren(
	children: MarkupParsedChildren,
	opts: MarkupFormatNormalizeOptions,
	remainingChars: number,
): {
	textLength: number;
	text: StaticMarkup;
	modified: boolean;
} {
	// Sometimes we'll populate the inner text of a tag with no children
	if (children.length === 0) {
		return {text: markup``, textLength: 0, modified: false};
	}

	let textLength = 0;
	let modified = false;

	let parts: AnyMarkups = [];

	for (const child of children) {
		if (child.type === "Text") {
			let text = readMarkup(markup`${child.value}`);
			textLength += text.length;

			const isVisible = remainingChars > 0;
			if (text.length > remainingChars) {
				text = sliceEscaped(text, remainingChars);
			}
			remainingChars -= text.length;
			if (isVisible) {
				// We already escaped it
				parts.push(convertToMarkupFromRandomString(text));
			}
		} else if (child.type === "Tag") {
			const inner = normalizeMarkupChildren(
				child.children,
				opts,
				remainingChars,
			);

			if (inner.modified) {
				modified = true;
			}

			if (remainingChars > 0) {
				const tag = buildTag(child, inner.text, opts);
				parts.push(tag.value);

				if (tag.modified) {
					modified = true;
				}
			}

			textLength += inner.textLength;
			remainingChars -= inner.textLength;
		} else {
			throw new Error("Unknown child node type");
		}
	}

	return {
		text: joinMarkup(parts),
		textLength,
		modified,
	};
}

export function joinMarkupLines({lines}: MarkupLinesAndWidth): string {
	return lines.join("\n");
}

export function normalizeMarkup(
	input: AnyMarkup,
	opts: MarkupFormatNormalizeOptions = {},
	maxLength: number = Infinity,
): {
	visibleTextLength: number;
	truncatedLength: number;
	textLength: number;
	text: StaticMarkup;
	truncated: boolean;
} {
	const inputRead = readMarkup(input);

	const {textLength, text, modified} = normalizeMarkupChildren(
		parseMarkup(inputRead),
		opts,
		maxLength,
	);

	const isTruncated = textLength > maxLength;

	return {
		textLength,
		text: modified ? text : convertToMarkupFromRandomString(inputRead),
		truncated: isTruncated,
		visibleTextLength: isTruncated ? maxLength : textLength,
		truncatedLength: isTruncated ? textLength - maxLength : 0,
	};
}
