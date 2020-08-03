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
	concatMarkup,
	convertToMarkupFromRandomString,
	isEmptyMarkup,
	markup,
	readMarkup,
} from "./escape";
import {sliceEscaped} from "@internal/string-utils";
import {buildFileLink, formatGrammarNumber} from "./util";

function buildTag(
	tag: MarkupParsedTag,
	inner: AnyMarkup,
	opts: MarkupFormatNormalizeOptions,
): StaticMarkup {
	let {attributes} = tag;

	switch (tag.name) {
		// Normalize filename of <filelink target>
		case "filelink": {
			// Clone
			attributes = attributes.copy();

			const {filename, line, column, text} = buildFileLink(attributes, opts);

			attributes.get("target").setValue(filename);

			if (isEmptyMarkup(inner) || opts.stripFilelinkText) {
				inner = markup`${text}`;
			}

			if (opts.stripPositions) {
				attributes.get("line").setValue(undefined);
				attributes.get("column").setValue(undefined);
			} else {
				attributes.get("column").setValue(column);
				attributes.get("line").setValue(line);
			}
			break;
		}

		// We don't technically need to normalize this but it's one less tag to have to support
		// if other tools need to consume it
		case "grammarNumber":
			return markup`${formatGrammarNumber(attributes, readMarkup(inner))}`;
	}

	let open = `<${tag.name}`;

	// Print attributes
	for (const [key, value] of attributes.asMap()) {
		if (!value.exists()) {
			continue;
		}

		const raw = value.asUnknown();
		if (raw === true) {
			open += ` ${key}`;
		} else {
			open += readMarkup(markup` ${key}="${String(raw)}"`);
		}
	}

	if (isEmptyMarkup(inner)) {
		return convertToMarkupFromRandomString(`${open} />`);
	} else {
		return convertToMarkupFromRandomString(
			`${open}>${readMarkup(inner)}</${tag.name}>`,
		);
	}
}

function normalizeMarkupChildren(
	children: MarkupParsedChildren,
	opts: MarkupFormatNormalizeOptions,
	remainingChars: number,
): {
	textLength: number;
	text: StaticMarkup;
} {
	// Sometimes we'll populate the inner text of a tag with no children
	if (children.length === 0) {
		return {text: markup``, textLength: 0};
	}

	let textLength = 0;

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

			if (remainingChars > 0) {
				parts.push(buildTag(child, inner.text, opts));
			}
			textLength += inner.textLength;
			remainingChars -= inner.textLength;
		} else {
			throw new Error("Unknown child node type");
		}
	}

	return {
		text: concatMarkup(parts),
		textLength,
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
	const {textLength, text} = normalizeMarkupChildren(
		parseMarkup(input),
		opts,
		maxLength,
	);

	const isTruncated = textLength > maxLength;

	return {
		textLength,
		text,
		truncated: isTruncated,
		visibleTextLength: isTruncated ? maxLength : textLength,
		truncatedLength: isTruncated ? textLength - maxLength : 0,
	};
}
