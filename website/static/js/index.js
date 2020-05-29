const elements = {
  mobileHandle: document.getElementsByClassName('mobile-handle')[0],
  toc: document.getElementsByClassName('toc-container')[0],
  sidebar: document.getElementsByClassName('sidebar')[0],
  overlay: document.getElementsByClassName('overlay')[0],
  headings: [...document.querySelectorAll('.content h1, .content h2, .content h3')],
  headerMobile: document.getElementsByClassName('header-mobile')[0],
};

function isMobile(){
  return  elements.sidebar.classList.contains('visible') || /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
}

const toc = {
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
      const link = document.querySelectorAll(`.toc-container a[href='${id}']`)[0];

      const nextElement = elements.headings[i + 1];
      let offsetTop = (y - (marginTop));
      if(nextElement){
        offsetTop = (nextElement.offsetTop);
      }

      let start = (y - marginTop) - toc.getMobileNavbarHeight() - 2;
      let end = (offsetTop - toc.getMobileNavbarHeight()) - (marginTop) - 5;

      if (scrollY > start && scrollY < end) {

        link.classList.add('active');

      } else {
        link.classList.remove('active');
      }

    }

  },
  handleClick(event){
    const target = event.target;
    event.preventDefault();

    if(target.hasAttribute("href")){

      const heading = document.querySelector(target.getAttribute('href'));
      const marginTop = parseFloat(window.getComputedStyle(heading).marginTop, 10);

      window.scrollTo(0, (heading.offsetTop) - toc.getMobileNavbarHeight() - marginTop);

      if(isMobile()){
        mobileToggleEvent(event);
      }

    }
  },
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

toc.highlight();

elements.toc.addEventListener('click', toc.handleClick, false);
elements.mobileHandle.addEventListener('click', mobileToggleEvent, false);
elements.overlay.addEventListener('click', mobileToggleEvent, false);
elements.overlay.addEventListener("touchstart", mobileToggleEvent, false);
window.addEventListener('scroll', handleScroll, false);