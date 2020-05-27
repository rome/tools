const mobileHandle = document.getElementsByClassName('mobile-handle')[0];
const toc = document.getElementsByClassName('toc-container')[0];
const sidebar = document.getElementsByClassName('sidebar')[0];
const overlay = document.getElementsByClassName('overlay')[0];

// hold the reference for the last clicked toc link,
// so .active can be removed on next click
let lastClickElement = null;

function handleTocClick(event){
  const target = event.target;
  event.preventDefault();

  if(target.hasAttribute("href")){

    const heading = document.querySelector(target.getAttribute('href'));
    const rect = heading.getBoundingClientRect();
    let mobileScrollOffset = 20;
    if(lastClickElement) lastClickElement.classList.remove('active');

    target.classList.add('active');
    lastClickElement = target;
    //only call if on mobile
    if(sidebar.classList.contains('visible')){
      mobileToggleEvent(event);
      mobileScrollOffset = 84;
    }

    window.scrollBy({
      top: (heading.offsetTop - window.scrollY) - mobileScrollOffset,
      behavior: 'smooth'
    });

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