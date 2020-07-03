const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");
const markdownIt = require("markdown-it");
const markdownItAnchor = require("markdown-it-anchor");
const fs = require("fs");
const pluginTOC = require("eleventy-plugin-nesting-toc");
const path = require("path");

const opts = {
	dirInput: "src",
	staticPath: "static",
	docsPath: "_includes/docs",
	dirOutput: "build",
	toc: {
		headingText: "test", // Optional text to show in heading above the wrapper element
	},
};

module.exports = function(eleventyConfig) {
	// Aything listed in .gitignore will be ignored by the watch process,
	// workaround to let eleventry rebuild when the css stylesheet gets rebuild.
	eleventyConfig.setUseGitIgnore(false);

	eleventyConfig.addPassthroughCopy(opts.staticPath);

	eleventyConfig.addPlugin(syntaxHighlight);
	eleventyConfig.addPlugin(pluginTOC);

	const md = markdownIt({
		html: true,
		linkify: true,
		typographer: true,
	}).use(
		markdownItAnchor,
		{
			permalink: true,
			permalinkSymbol: "#",
		},
	);

	eleventyConfig.setLibrary("md", md);

	eleventyConfig.addShortcode(
		"rootmd",
		function(file) {
			const relativeFilePath = path.join(__dirname, "..", file);
			const data = fs.readFileSync(
				relativeFilePath,
				function(err, contents) {
					if (err) {
						throw new Error(err);
					}
					return contents;
				},
			);
			return md.render(data.toString());
		},
	);

	return {
		dir: {
			input: opts.dirInput,
			output: opts.dirOutput,
		},
		passthroughFileCopy: true,
		templateFormats: ["njk", "md", "css", "html", "yml"],
		htmlTemplateEngine: "njk",
	};
};
