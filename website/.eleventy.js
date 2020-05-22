const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");
const CleanCSS = require("clean-css");
const markdownIt = require("markdown-it");
const markdownItAnchor = require('markdown-it-anchor');

module.exports = function(eleventyConfig) {

  eleventyConfig.addFilter("cssmin", function(code) {
    return new CleanCSS({}).minify(code).styles;
  });

  eleventyConfig.addPlugin(syntaxHighlight);

  let options = {
    html: true,
    linkify: true,
    typographer: true,
  };

  eleventyConfig.setLibrary("md", markdownIt(options).use(markdownItAnchor, {}));

  return {
    dir: {
      input: "src",
      output: "build",
      includes: "includes"
    }

  };

};