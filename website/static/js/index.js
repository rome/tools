tocbot.init({
  tocSelector: '.toc-container',
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

document
  .getElementsByClassName('mobile-handle')[0]
  .addEventListener('click', function (event) {
    event.preventDefault();
    const toc = document.getElementsByClassName('table-of-contents')[0];
    toc.classList.toggle('mobile-visible');
  });