export function buildGetPages(pages) {
	return (category: string) => {
		return pages
			.filter((page) => page.frontmatter.category === category)
			.sort((a, b) => {
				return a.frontmatter.title.localeCompare(b.frontmatter.title);
			});
	};
}
