tocbot.init({
  tocSelector: '.table-of-contents',
  ignoreSelector: '[toc-exclude]',
  contentSelector: '.content',
  headingSelector: 'h1, h2, h3',
  hasInnerContainers: true,
  collapseDepth: 3,
  headingObjectCallback: function (def, domElement) {
    def.textContent = domElement.innerText;
    return def;
  },
});