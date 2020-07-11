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
	blogPath: "src/posts",
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
  eleventyConfig.addPassthroughCopy("_redirects");

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

	eleventyConfig.addShortcode(
		"postslist",
		function() {

			let files = fs.readdirSync(opts.blogPath);

			//remove index.md from the list and only accept .md files
			files = files.filter(function(file){
				return file !== 'index.md' && path.extname(file) === '.md';
			});

			//sort by date
			files.sort(function(fileA,fileB){
				const contentA = fs.readFileSync(`${opts.blogPath}/${fileA}`, "utf8").toString();
				const dateA = contentA.match(/date:(.*)/)[1];

				const contentB = fs.readFileSync(`${opts.blogPath}/${fileB}`, "utf8").toString();
				const dateB = contentB.match(/date:(.*)/)[1];
				return new Date(dateB) - new Date(dateA);
			});

			let list = ``;
			files.forEach(file => {
				list += '<article>';
				const content = fs.readFileSync(`${opts.blogPath}/${file}`, "utf8").toString();
				const title = content.match(/title:(.*)/)[1];
				const author = content.match(/author:(.*)/)[1];
				const date = content.match(/date:(.*)/)[1];
				const description = content.match(/description:(.*)/)[1];

				list += `<h1><a href="${file.replace(path.extname(file), "")}">${title}</a></h1>`;
				list += `<div class="${author}">by ${author}</div>`;
				list += `<time datetime="${date}">${date}</time>`;
				list += `<p>${description}</p>`;

				list += '</article>';
			});

			return list;
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
