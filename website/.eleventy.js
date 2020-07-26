// @ts-check
const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");
const eleventyNavigationPlugin = require("@11ty/eleventy-navigation");
const markdownIt = require("markdown-it");
const markdownItAnchor = require("markdown-it-anchor");
const pluginSass = require("eleventy-plugin-sass");
const fs = require("fs");
const pluginTOC = require("eleventy-plugin-nesting-toc");
const path = require("path");
const terser = require("terser");

/**
 * @type {any}
 */
const grayMatter = require("gray-matter");

module.exports = function(eleventyConfig) {
	eleventyConfig.addPassthroughCopy({"static": "."});

	eleventyConfig.setLiquidOptions({
		cache: true,
	});

	eleventyConfig.addPlugin(
		pluginSass,
		{
			sourcemaps: true,
			watch: ["src/**/*.{scss,sass}"],
		},
	);

	eleventyConfig.addPlugin(syntaxHighlight);

	eleventyConfig.addPlugin(
		pluginTOC,
		{
			tags: ["h2", "h3", "h4"],
			wrapper: "div  ",
			wrapperClass: "toc",
		},
	);

	eleventyConfig.addPlugin(eleventyNavigationPlugin);

	const md = markdownIt({
		html: true,
		linkify: true,
		typographer: true,
	}).use(
		markdownItAnchor,
		{
			permalink: true,
			permalinkSymbol: "",
		},
	);

	eleventyConfig.setLibrary("md", md);

	eleventyConfig.addShortcode(
		"rootmd",
		function(file) {
			const data = fs.readFileSync(path.join(__dirname, "..", file));
			return md.render(data.toString());
		},
	);

	// Taken from https://github.com/11ty/eleventy-base-blog/blob/master/_11ty/getTagList.js
	eleventyConfig.addCollection(
		"tagList",
		function(collection) {
			let tagSet = new Set();
			collection.getAll().forEach(function(item) {
				if ("tags" in item.data) {
					let tags = item.data.tags;

					tags = tags.filter(function(item) {
						switch (item) {
							// this list should match the `filter` list in tags.liquid
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
		},
	);

	const jsminCache = new Map();

	eleventyConfig.addFilter(
		"jsmin",
		function(code) {
			const cached = jsminCache.get(code);
			if (cached !== undefined) {
				return cached;
			}

			const minified = terser.minify(code);
			if (minified.error) {
				console.log("Terser error: ", minified.error);
				return code;
			}

			jsminCache.set(code, minified.code);
			return minified.code;
		},
	);

	eleventyConfig.addFilter(
		"dateFormat",
		function(value) {
			return new Date(value).toLocaleDateString(
				undefined,
				{year: "numeric", month: "long", day: "numeric"},
			);
		},
	);

	eleventyConfig.addFilter(
		"kebabCase",
		function(string) {
			return string.toLowerCase().replace(/\s/g, "-");
		},
	);

	// Customize YAML engine so we can parse hard tabs lol...
	eleventyConfig.setFrontMatterParsingOptions({
		engines: {
			yaml: {
				...grayMatter.engines.yaml,
				parse(content) {
					content = content.replace(/\t/g, "  ");
					return grayMatter.engines.yaml.parse(content);
				},
			},
		},
	});

	return {
		dir: {
			input: "src",
			output: "build",
		},
		templateFormats: ["liquid", "md", "css", "html", "yml"],
		htmlTemplateEngine: "liquid",
	};
};
