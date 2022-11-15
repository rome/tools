import type Astro from "astro";

type GlobInstance = Astro.MarkdownInstance<any> | Astro.MDXInstance<any>;

function getTitle(page: GlobInstance): string {
	return page.frontmatter.title ?? "";
}

export function buildGetPages(pages: GlobInstance[]) {
	return (category?: string): GlobInstance[] => {
		return pages
			.filter(
				(page) =>
					category === undefined || page.frontmatter.category === category,
			)
			.sort((a, b) => {
				return getTitle(a).localeCompare(getTitle(b));
			});
	};
}

export function buildTOC(page: Astro.MDXInstance<any>): string {
	const headings = page
		.getHeadings()
		.filter((heading) => heading.depth <= 4 && heading.depth > 1);
	if (headings.length === 0) {
		return "";
	}

	function buildDepth(startIndex: number, depth: number): string {
		let buf = "";

		for (let i = startIndex; i < headings.length; i++) {
			const heading = headings[i]!;
			if (heading.depth === depth - 1) {
				break;
			}
			if (heading.depth !== depth) {
				continue;
			}

			buf += "<li>";
			buf += `<a href="#${encodeURI(heading.slug)}">${heading.text}</a>`;

			const subTOC = buildDepth(i + 1, depth + 1);
			if (subTOC !== "") {
				buf += `<ol>${subTOC}</ol>`;
			}

			buf += "</li>";
		}

		return buf;
	}

	const toc = buildDepth(0, 2);
	if (toc === "") {
		return "";
	} else {
		return `<div class="toc"><ol>${toc}</ol?></div>`;
	}
}
