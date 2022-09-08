// @ts-check
const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");
const eleventyNavigationPlugin = require("@11ty/eleventy-navigation");
const markdownIt = require("markdown-it");
const markdownItHeaderSections = require("markdown-it-header-sections");
const markdownItAnchor = require("markdown-it-anchor");
const markdownItImageSize = require("markdown-it-imsize");
const markdownItFootnote = require("markdown-it-footnote");
const fs = require("fs");
const pluginTOC = require("eleventy-plugin-nesting-toc");
const path = require("path");
const terser = require("terser");
const CleanCSS = require("clean-css");
const htmlmin = require("html-minifier");
const { base64Encode } = require("./utils");
const pluginRss = require("@11ty/eleventy-plugin-rss");

require("dotenv").config();

/**
 * @type {any}
 */
const grayMatter = require("gray-matter");

const isProduction = process.env.ELEVENTY_ENV === "production";

module.exports = function (eleventyConfig) {
	eleventyConfig.addPassthroughCopy({ static: "." });
	eleventyConfig.setUseGitIgnore(false);

	eleventyConfig.setLiquidOptions({
		cache: true,
		root: ["_includes", "./src/_includes", "./src/_includes/layouts"],
		dynamicPartials: false,
		strictFilters: false,
	});

	eleventyConfig.addPlugin(syntaxHighlight);
	eleventyConfig.addPlugin(pluginRss);

	eleventyConfig.addPlugin(pluginTOC, {
		tags: ["h2", "h3", "h4"],
		wrapper: "div  ",
		wrapperClass: "toc",
	});

	eleventyConfig.addPlugin(eleventyNavigationPlugin);

	const md = markdownIt({ html: true, linkify: true, typographer: true });

	md.use(markdownItHeaderSections);

	md.use(markdownItImageSize);

	md.use(markdownItFootnote);

	md.use(markdownItAnchor, {
		permalink: true,
		permalinkSymbol: "",
		permalinkAttrs: (slug) => ({ "aria-label": slug }),
		slugify: (title) => {
			return encodeURIComponent(
				String(title).trim().toLowerCase().replace(
					/[^a-zA-Z\s0-9]/g,
					"",
				).replace(/\s+/g, "-"),
			);
		},
	});

	eleventyConfig.setLibrary("md", md);

	// Taken from https://github.com/11ty/eleventy-base-blog/blob/master/_11ty/getTagList.js
	eleventyConfig.addCollection("tagList", function (collection) {
		let tagSet = new Set();
		collection.getAll().forEach(function (item) {
			if ("tags" in item.data) {
				let tags = item.data.tags;

				tags = tags.filter(function (item) {
					switch (item) {
						// This list should match the `filter` list in tags.liquid
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

	// Used for including raw files without having them processed by liquid
	const includerawCache = new Map();
	eleventyConfig.addFilter("includeraw", function (loc) {
		const cached = includerawCache.get(loc);
		if (cached !== undefined) {
			return cached;
		}

		const file = fs.readFileSync(path.resolve(__dirname, loc), "utf8");
		includerawCache.set(loc, file);
		return file;
	});

	eleventyConfig.addFilter("toBase64", (content, ext) => {
		return base64Encode(new Buffer(content), ext);
	});

	const minCache = new Map();

	// Minify JS in production
	eleventyConfig.addFilter("jsmin", function (code) {
		if (!isProduction) {
			return code;
		}

		const cached = minCache.get(code);
		if (cached !== undefined) {
			return cached;
		}

		const minified = terser.minify(code);
		if (minified.error) {
			throw minified.error;
		}

		minCache.set(code, minified.code);
		return minified.code;
	});

	// Minify CSS in production
	eleventyConfig.addFilter("mincss", function (code) {
		if (!isProduction) {
			return code;
		}

		const cached = minCache.get(code);
		if (cached !== undefined) {
			return cached;
		}

		const minified = new CleanCSS({}).minify(code).styles;
		minCache.set(code, minified);
		return minified;
	});

	// Minify HTML in production
	eleventyConfig.addTransform("htmlmin", function (content, outputPath) {
		if (isProduction && outputPath.endsWith(".html")) {
			return htmlmin.minify(content, {
				useShortDoctype: true,
				removeComments: true,
				conservativeCollapse: true,
				collapseWhitespace: true,
			});
		}

		return content;
	});

	eleventyConfig.addFilter("blogSummary", (val) => {
		const lines = val.split("<!-- DESCRIPTION_END -->")[0].split("\n");
		return lines.filter((line) => {
			return line.startsWith("<p>");
		}).join("\n");
	});

	eleventyConfig.addFilter("dateFormat", function (value) {
		return new Date(value).toLocaleDateString(undefined, {
			year: "numeric",
			month: "long",
			day: "numeric",
			timeZone: "UTC",
		});
	});

	eleventyConfig.addFilter("titlify", function (title) {
		title = (title || "").trim();
		if (!title.includes("Rome")) {
			if (title !== "") {
				title += " \u2014 ";
			}
			title += "Rome Toolchain";
		}
		return title;
	});

	eleventyConfig.addFilter("kebabCase", function (string) {
		return string.toLowerCase().replace(/\s/g, "-");
	});

	eleventyConfig.addFilter("withAbsoluteUrl", function (string) {
		return `https://rome.tools${string}`;
	});

	eleventyConfig.addShortcode("romeVersion", function () {
		return "?.?.?";
	});

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
		dir: { input: "src", output: "build" },
		templateFormats: ["liquid", "md", "css", "html", "yml"],
		htmlTemplateEngine: "liquid",
	};
};
