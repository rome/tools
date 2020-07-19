const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");
const eleventyNavigationPlugin = require("@11ty/eleventy-navigation");
const markdownIt = require("markdown-it");
const markdownItAnchor = require("markdown-it-anchor");
const pluginSass = require("eleventy-plugin-sass");
const fs = require("fs");
const pluginTOC = require("eleventy-plugin-nesting-toc");
const path = require("path");

module.exports = function(eleventyConfig) {
	eleventyConfig.addPassthroughCopy({"static": "."});

	eleventyConfig.addPlugin(pluginSass, {
		sourcemaps: true,
		watch: ["src/**/*.{scss,sass}"],
	});

	eleventyConfig.addPlugin(syntaxHighlight);
	
	eleventyConfig.addPlugin(pluginTOC, {
		tags: ['h2', 'h3'],
		wrapper: 'div  ',
		wrapperClass: 'fixed toc',
	});

	eleventyConfig.addPlugin(eleventyNavigationPlugin);

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

	// Taken from https://github.com/11ty/eleventy-base-blog/blob/master/_11ty/getTagList.js
	eleventyConfig.addCollection("tagList", function(collection) {
		let tagSet = new Set();
		collection.getAll().forEach(function(item) {
			if( "tags" in item.data ) {
				let tags = item.data.tags;

				tags = tags.filter(function(item) {
					switch(item) {
						// this list should match the `filter` list in tags.njk
						case "all":
						case "nav":
						case "post":
						case "posts":
							return false;
					}

					return true;
				});

				for (const tag of tags) {
					tagSet.add(tag);
				}
			}
		});

		// Returning an array in addCollection works in Eleventy 0.5.3
		return [...tagSet];
	});

	eleventyConfig.addFilter("dateFormat", function(value) {
		return new Date(value).toLocaleDateString(undefined, { year: 'numeric', month: 'long', day: 'numeric' });
	});

	eleventyConfig.addFilter("kebabCase", function(string) {
		return string.toLowerCase().replace(/\s/g, '-');
	});

	eleventyConfig.addFilter("shouldIncludeTOC", function(content) {
		const li = content.match(/<li/g);
		if (li == null || li.length <= 2) {
			return "";
		} else {
			return content;
		}
	});

	return {
		dir: {
			input: "src",
			output: "build",
		},
		passthroughFileCopy: true,
		templateFormats: ["njk", "md", "css", "html", "yml"],
		htmlTemplateEngine: "njk",
	};
};
