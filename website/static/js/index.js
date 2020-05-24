const mobileHandle = document.getElementsByClassName('mobile-handle')[0];
const toc = document.getElementsByClassName('sidebar')[0];
const overlay = document.getElementsByClassName('overlay')[0];

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

function mobileToggleEvent(event){
  event.preventDefault();
  toc.classList.toggle('visible');
  overlay.classList.toggle('visible');
}

mobileHandle.addEventListener('click', mobileToggleEvent);
overlay.addEventListener('click', mobileToggleEvent);