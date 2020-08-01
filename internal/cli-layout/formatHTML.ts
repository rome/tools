import {escapeXHTMLEntities} from "@internal/html-parser";
import {MarkupParsedTag, validateColor} from "@internal/markup";

export function htmlFormatText(
	{name: tagName, attributes}: MarkupParsedTag,
	value: string,
): string {
	switch (tagName) {
		case "hyperlink": {
			return `<a href="${escapeXHTMLEntities(
				attributes.get("target").asString(value),
			)}">${value}</a>`;
		}

		case "filelink": {
			// We probably don't need filelinks if it's just for presentation in the browser?
			//const filename = getFileLinkFilename(attributes, opts);
			//return `<a href="file://${escapeXHTMLEntities(filename)}">${value}</a>`;
			return `<span style="text-decoration-style: dashed; text-decoration-line: underline;">${value}</span>`;
		}

		case "inverse":
			return `<span style="color: #000; background-color: #ddd;">${value}</span>`;

		case "emphasis":
			return `<strong>${value}</strong>`;

		case "dim":
			return `<span style="opacity: 0.8;">${value}</span>`;

		case "italic":
			return `<i>${value}</i>`;

		case "underline":
			return `<u>${value}</u>`;

		case "strike":
			return `<strike>${value}</strike>`;

		case "error":
			return `<span style="color: Tomato;">${value}</span>`;

		case "success":
			return `<span style="color: MediumSeaGreen;">${value}</span>`;

		case "warn":
			return `<span style="color: Orange;">${value}</span>`;

		case "info":
			return `<span style="color: DodgerBlue;">${value}</span>`;

		case "code":
			return `<i>${value}</i>`;

		case "highlight": {
			const index = Math.min(0, attributes.get("i").asNumber(0));
			const color = highlightColors[index % highlightColors.length];
			return `<span style="color: ${color};">${value}</span>`;
		}

		case "color": {
			const styles = [];

			const fg = validateColor(attributes.get("fg").asStringOrVoid());
			if (fg !== undefined) {
				styles.push(`color: ${fg}`);
			}

			const bg = validateColor(attributes.get("bg").asStringOrVoid());
			if (bg !== undefined) {
				styles.push(`background-color: ${bg}`);
			}

			return `<span style="${styles.join("; ")}">${value}</span>`;
		}

		case "token":
			return `<span class="token ${attributes.get("type").asString("")}">${value}</span>`;

		default:
			return value;
	}
}

// TODO fill this with more
const highlightColors = ["magenta", "cyan"];
