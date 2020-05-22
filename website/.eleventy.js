const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");
const CleanCSS = require("clean-css");
const markdownIt = require("markdown-it");
const markdownItAnchor = require('markdown-it-anchor');
const Terser = require("terser");

module.exports = function(eleventyConfig) {

  eleventyConfig.addFilter("cssmin", function(code) {
    return new CleanCSS({}).minify(code).styles;
  });


  eleventyConfig.addFilter("jsmin", function(code) {
    let minified = Terser.minify(code);
    if( minified.error ) {
        console.log("Terser error: ", minified.error);
        return code;
    }

    return minified.code;
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