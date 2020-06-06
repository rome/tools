const elements = {
  mobileHandle: document.getElementsByClassName('mobile-handle')[0],
  toc: document.getElementsByClassName('toc-container')[0],
  tocLinks: [...document.querySelectorAll('.toc-container a')],
  sidebar: document.getElementsByClassName('sidebar')[0],
  overlay: document.getElementsByClassName('overlay')[0],
  headings: [...document.querySelectorAll('.content h1, .content h2, .content h3')],
  headerMobile: document.getElementsByClassName('header-mobile')[0],
  modeSwitch: document.getElementById('mode-switch')
};

function isMobile(){
  return  elements.sidebar.classList.contains('visible') || /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
}

const toc = {
  currentH1Link: null,
  getMobileNavbarHeight(){

    if(isMobile()){
      return parseFloat(window.getComputedStyle(elements.headerMobile).height, 10);
    }

    return 0;
  },
  highlight(){

    var scrollY = window.scrollY;

    for (let i = 0; i < elements.headings.length; i++) {
      const element = elements.headings[i];

      const id = `#${element.getAttribute('id')}`;
      const y = element.offsetTop;
      const marginTop = parseFloat(window.getComputedStyle(element).marginTop, 10);
      const height = parseFloat(window.getComputedStyle(element).marginTop, 10);
      const link = document.querySelectorAll(`.toc-container a[href='${id}']`)[0];

      const nextElement = elements.headings[i + 1];

      let start = y - marginTop;
      let end = y + height + marginTop;

      if(nextElement){
        const nextMarginTop = parseFloat(window.getComputedStyle(nextElement).marginTop, 10);
        end = (nextElement.offsetTop - nextMarginTop);
      }

      start -= toc.getMobileNavbarHeight();
      end -= toc.getMobileNavbarHeight();

      if (scrollY > start && scrollY < end) {

        link.classList.add('active');

        if(toc.currentH1Link){
          if(toc.currentH1Link.parentElement.lastElementChild.tagName == 'OL'){
            toc.currentH1Link.parentElement.lastElementChild.classList.remove('not-sr-only');
          }
        }

        if(element.tagName === 'H2'){

          toc.currentH1Link = link // link inside a level 2 OL
                              .parentElement // li
                              .parentElement // ol
                              .parentElement // Level one ol
                              .firstElementChild; // h1 link

          link.parentElement.parentElement.classList.add('not-sr-only');

        }

      } else {

        link.classList.remove('active');

      }

    }

  },
  handleClick(event){
    const target = event.target;
    event.preventDefault();

    if(target.hasAttribute("href")){

      scrollToHeading(target.getAttribute('href'));

      if(isMobile()){
        mobileToggleEvent(event);
      }

    }
  },
}

function scrollToHeading(hash){
  const heading = document.querySelector(hash);

  if(!heading) return null;

  const marginTop = parseFloat(window.getComputedStyle(heading).marginTop, 10);
  window.scrollTo(0, (heading.offsetTop) - toc.getMobileNavbarHeight() - (marginTop - 2));
}

function handleScroll(){

  if(isMobile()){
    return false;
  }

  toc.highlight();

  if(window.scrollY > 6){
    elements.sidebar.style['border-top-width'] = '5.3333333333px';
  } else {
    elements.sidebar.style['border-top-width'] = '0px';
  }

}

function mobileToggleEvent(event){
  const bodyClassList = document.body.classList;
  event.preventDefault();
  elements.sidebar.classList.toggle('visible');
  elements.overlay.classList.toggle('visible');
  bodyClassList.toggle('no-scroll');
  toc.highlight();
}

function modeSwitch(event){

  event.preventDefault();

  const $doc = document.documentElement;
  const atribute = $doc.getAttribute('data-theme');

  if(atribute === 'light'){
    $doc.setAttribute('data-theme', 'dark');
    elements.modeSwitch.innerText = "Light Mode";
  }

  if(atribute === 'dark'){
    $doc.setAttribute('data-theme', 'light');
    elements.modeSwitch.innerText = "Dark Mode";
  }

}

//remove permalinkSymbol "#" from table of contents
elements.tocLinks.forEach(function(link){
  link.innerText = link.innerText.replace(/(\s#)$/,'');
});

window.onload = function(){
  scrollToHeading(window.location.hash);
  toc.highlight();
}

document.addEventListener('click', function (event) {

	if (!event.target.matches('.header-anchor')) return;

	event.preventDefault();

  scrollToHeading(event.target.getAttribute('href'));

}, false);

elements.toc.addEventListener('click', toc.handleClick, false);
elements.mobileHandle.addEventListener('click', mobileToggleEvent, false);
elements.overlay.addEventListener('click', mobileToggleEvent, false);
elements.overlay.addEventListener("touchstart", mobileToggleEvent, false);
window.addEventListener('scroll', handleScroll, false);

elements.modeSwitch.addEventListener('click', modeSwitch, false);