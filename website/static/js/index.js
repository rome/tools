const mobileHandle = document.getElementsByClassName('mobile-handle')[0];
const toc = document.getElementsByClassName('toc-container')[0];
const sidebar = document.getElementsByClassName('sidebar')[0];
const overlay = document.getElementsByClassName('overlay')[0];
const tocLinks = document.querySelectorAll('.toc-container a');
const headings = [...document.querySelectorAll('.content h1, .content h2, .content h3')];

function isMobile(){
  return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
}

// hold the reference for the last clicked toc link,
// so .active can be removed on next click
let lastClickElement = null;

function handleTocClick(event){
  const target = event.target;
  event.preventDefault();

  if(lastClickElement === target){
    if(isMobile()){
      mobileToggleEvent(event);
    }
    return;
  }

  if(target.hasAttribute("href")){

    const heading = document.querySelector(target.getAttribute('href'));
    const marginTop = parseFloat(window.getComputedStyle(heading).marginTop, 10);
    const height = parseFloat(window.getComputedStyle(heading).height, 10);
    let mobileScrollOffset = marginTop;

    //only call if on mobile
    if(isMobile()){
      mobileToggleEvent(event);
      mobileScrollOffset = height + marginTop;
      removeActive(currentActiveLink);

      headingIndex = clamp(headings.indexOf(heading) - 1, 0, tocLinks.length - 1);

    }

    lastClickElement = target;

    console.log((heading.offsetTop - window.scrollY) - mobileScrollOffset);

    window.scrollBy({
      top: (heading.offsetTop - window.scrollY) - mobileScrollOffset,
      behavior: 'smooth'
    });

    updateTocLinkHighlight();
  }
}

function mobileToggleEvent(event){
  const bodyClassList = document.body.classList;
  event.preventDefault();
  sidebar.classList.toggle('visible');
  overlay.classList.toggle('visible');
  bodyClassList.toggle('no-scroll');
}

toc.addEventListener('click', handleTocClick, false);
mobileHandle.addEventListener('click', mobileToggleEvent, false);
overlay.addEventListener('click', mobileToggleEvent, false);
overlay.addEventListener("touchstart", mobileToggleEvent, false);

/*** Code to handle toc link highlight on scroll ****/

let headingIndex = foundClosestHeading();
let lastHeadingIndex = null;
let currentActiveLink = 0;
let lastScrollPosition = window.scrollY;

function foundClosestHeading(){
  let index = 0;

  headings.forEach(function(element){

    if( element.offsetTop - window.scrollY < 0){
      index = headings.indexOf(element) + 1;
    }
  });

  return index;
}

function clamp(number, min, max){
  return Math.min(Math.max(number, min), max);
}

function removeActive(index){
  index = clamp(index, 0, tocLinks.length - 1);
  if(tocLinks[index].classList.contains("active")){
    tocLinks[index].classList.remove("active");
  }
}

function addActive(index){
  index = clamp(index, 0, tocLinks.length - 1);
  if(!tocLinks[index].classList.contains("active")){
    tocLinks[index].classList.add("active");
  }
}

function updateTocLinkHighlight(){

  const id = tocLinks[headingIndex].getAttribute('href');
  const heading = document.querySelector(id);
  const marginTop = parseFloat(window.getComputedStyle(heading).marginTop, 10) + 3;
  const height = parseFloat(window.getComputedStyle(heading).height, 10);

  let mobileNavbarHeight = 0;
  let mobileScrollOffset = marginTop - 2;

  if(isMobile()){
    mobileNavbarHeight = 64;
    mobileScrollOffset = height + marginTop;
  }

  let diff = window.scrollY - lastScrollPosition;
  let length = Math.sqrt(diff * diff);
  //-1 == up || 1 == down
  let normal = diff === 0 ? 0 : (diff / length);

  console.log(normal);

  switch (normal) {
    case 1:
      if( window.scrollY + mobileNavbarHeight > heading.offsetTop - mobileScrollOffset){
        headingIndex += 1;
      }

      break;
    case -1:
      if( window.scrollY - mobileNavbarHeight < heading.offsetTop + mobileScrollOffset && headingIndex >= 0){
        headingIndex -= 1;
      }
      break;

    default:
      break;
  }

  if(lastHeadingIndex !== headingIndex){

    removeActive(lastHeadingIndex + normal);
    removeActive(lastHeadingIndex - normal);
    currentActiveLink = headingIndex - normal;
    addActive(currentActiveLink);

  }

  headingIndex = clamp(headingIndex, 0, tocLinks.length - 1);
  lastHeadingIndex = headingIndex;
  lastScrollPosition = window.scrollY;

}

updateTocLinkHighlight();

window.addEventListener('scroll', function(){

  updateTocLinkHighlight();

});
