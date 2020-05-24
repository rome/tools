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
  onClick: mobileToggleEvent,
  headingObjectCallback: function (def, domElement) {
    def.textContent = domElement.innerText;
    return def;
  },
});

function mobileToggleEvent(event){
  event.preventDefault();
  toc.classList.toggle('visible');
  overlay.classList.toggle('visible');
  document.body.classList.toggle('no-scroll');
}

mobileHandle.addEventListener('click', mobileToggleEvent, false);
overlay.addEventListener('click', mobileToggleEvent, false);
overlay.addEventListener("touchstart", mobileToggleEvent, false);