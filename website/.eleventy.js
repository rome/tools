const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");
const markdownIt = require("markdown-it");
const markdownItAnchor = require("markdown-it-anchor");
const fs = require("fs");

const Options = {
  dirInput: "src",
  staticPath: "static",
  docsPath: "_includes/docs",
  dirOutput: "build",
};

module.exports = function (eleventyConfig) {
  // Aything listed in .gitignore will be ignored by the watch process,
  // workaround to let eleventry rebuild when the css stylesheet gets rebuild.
  eleventyConfig.setUseGitIgnore(false);

  eleventyConfig.addPassthroughCopy(Options.staticPath);

  eleventyConfig.addPlugin(syntaxHighlight);

  const md = markdownIt({
    html: true,
    linkify: true,
    typographer: true,
  }).use(markdownItAnchor, {
    permalink: true,
    permalinkSymbol: '#',
  });

  eleventyConfig.setLibrary("md", md);
  eleventyConfig.addShortcode("doc", function (file) {
    const relativeFilePath = `./${Options.dirInput}/${Options.docsPath}/${file}`;
    const data = fs.readFileSync(relativeFilePath, function (err, contents) {
      if (err) {
        throw new Error(err);
      }
      return contents;
    });
    return md.render(data.toString());
  });

  return {
    dir: {
      input: Options.dirInput,
      output: Options.dirOutput,
    },
    passthroughFileCopy: true,
    templateFormats: ["njk", "md", "css", "html", "yml"],
    htmlTemplateEngine: "njk",
  };
};
