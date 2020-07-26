// @ts-check
const mobileHandleTOC = document.querySelector(".mobile-handle-toc");
const mobileHandleNav = document.querySelector(".mobile-handle-nav");
const toc = document.querySelector(".toc-container");
const sidebar = document.querySelector(".sidebar");

/** @type {NodeListOf<HTMLElement>}*/
const headings = document.querySelectorAll(
	".content h1, .content h2, .content h3",
);
const headerMobile = document.querySelector(".header-mobile");

/**
 * @returns {boolean}
 */
function isMobile() {
	return (
		sidebar.classList.contains("visible") ||
		/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
			navigator.userAgent,
		)
	);
}

const headingsSize = [];
for (const heading of headings) {
	const id = `#${heading.getAttribute("id")}`;
	const link = document.querySelector(`.toc a[href='${id}']`);
	if (link === undefined) {
		continue;
	}

	headingsSize.push({
		id,
		link,
		headingOffsetTop: heading.offsetTop,
		headingMarginTop: parseFloat(window.getComputedStyle(heading).marginTop),
	});
}

function isHeadingHighlighted(i) {
	const {headingMarginTop, headingOffsetTop} = headingsSize[i];

	let start = headingOffsetTop - headingMarginTop;
	let end = headingOffsetTop + headingMarginTop;

	const nextElement = headingsSize[i + 1];
	if (nextElement) {
		end = nextElement.headingOffsetTop - nextElement.headingMarginTop;
	}

	start -= tocControl.getMobileNavbarHeight();
	end -= tocControl.getMobileNavbarHeight();

	return scrollY > start && scrollY < end;
}

function toggleTOCActive(i) {
	const {link} = headingsSize[i];
	let target = link;
	while (target != null && target.tagName !== "DIV") {
		if (target.tagName === "LI") {
			target.classList.toggle("active");
		}
		target = target.parentElement;
	}
}

let lastActiveHeading;

const tocControl = {
	/**
	 * @returns {number}
	 */
	getMobileNavbarHeight() {
		if (isMobile()) {
			return parseFloat(window.getComputedStyle(headerMobile).height);
		}

		return 0;
	},

	highlight() {
		if (lastActiveHeading !== undefined) {
			if (isHeadingHighlighted(lastActiveHeading)) {
				return;
			} else {
				toggleTOCActive(lastActiveHeading);
				lastActiveHeading = undefined;
			}
		}

		for (let i = 0; i < headingsSize.length; i++) {
			if (isHeadingHighlighted(i)) {
				lastActiveHeading = i;
				toggleTOCActive(i);
				break;
			}
		}
	},

	/**
	 * @param {MouseEvent} event
	 */
	handleClick(event) {
		const target = event.target;
		event.preventDefault();
		if (!(target instanceof HTMLElement)) {
			return;
		}

		const hash = target.getAttribute("href");

		if (target.hasAttribute("href")) {
			window.location.hash = hash;

			scrollToHeading(hash);

			if (isMobile()) {
				toggleMobileTOC(event);
			}
		}
	},
};

/**
 * @param {string} hash
 */
function scrollToHeading(hash) {
	const heading = document.getElementById(hash.replace(/^(#)/, ""));
	if (!heading) {
		return;
	}

	heading.setAttribute("tabindex", "-1");
	heading.focus();

	const marginTop = parseFloat(window.getComputedStyle(heading).marginTop);
	window.scrollTo(
		0,
		heading.offsetTop - tocControl.getMobileNavbarHeight() - (marginTop - 2),
	);
}

const siteNavigationContainer = document.querySelector(".site-navigation");
const siteNavigationHeight = siteNavigationContainer.clientHeight;
function setSiteNavigationHeight() {
	console.log({siteNavigationHeight, scrollY});
	siteNavigationContainer.style.height = `${siteNavigationHeight - scrollY}px`;
}
setSiteNavigationHeight();

function handleScroll() {
	if (isMobile()) {
		return false;
	}

	setSiteNavigationHeight();

	tocControl.highlight();
}

/**
 * @type {undefined | "toc" | "nav"}
 */
let mobileSidebarActive;

function toggleMobileNav(event) {
	if (mobileSidebarActive === "nav") {
		mobileSidebarActive = undefined;
	} else {
		if (mobileSidebarActive === "toc") {
			toggleMobileTOC(event);
		}
		mobileSidebarActive = "nav";
	}

	event.preventDefault();
	mobileHandleNav.classList.toggle("active");
	sidebar.classList.toggle("visible");
	document.body.classList.toggle("no-scroll");
}

function toggleMobileTOC(event) {
	if (mobileSidebarActive === "toc") {
		mobileSidebarActive = undefined;
	} else {
		if (mobileSidebarActive === "nav") {
			toggleMobileNav(event);
		}
		mobileSidebarActive = "toc";
	}

	event.preventDefault();
	mobileHandleTOC.classList.toggle("active");
	document.body.classList.toggle("no-scroll");
	tocControl.highlight();
}

window.onload = function() {
	if (window.location.hash !== "") {
		scrollToHeading(window.location.hash);
	}
	tocControl.highlight();
	const script = document.createElement("script");
	script.src = "/docsearch.js";
	script.async = true;
	script.defer = true;
	script.addEventListener(
		"load",
		() => {
			return window.docsearch({
				apiKey: "66db1ad366d458c6acded7cbc23dba7e",
				indexName: "romefrontend",
				inputSelector: "#docsearch",
				debug: false, // Set debug to true if you want to inspect the dropdown
			});
		},
	);
	document.body.appendChild(script);
};

document.addEventListener(
	"click",
	function(event) {
		if (!(event.target instanceof HTMLElement)) {
			return;
		}

		if (!event.target.matches(".header-anchor")) {
			return;
		}

		event.preventDefault();

		const hash = event.target.getAttribute("href");
		window.location.hash = hash;
		scrollToHeading(hash);
	},
	false,
);

if (toc) {
	toc.addEventListener("click", tocControl.handleClick, false);
}

mobileHandleNav.addEventListener("click", toggleMobileNav, false);
mobileHandleTOC.addEventListener("click", toggleMobileTOC, false);
window.addEventListener("scroll", handleScroll, false);

//# Color scheme switcher

function toggleColorSchemeSwitch() {
	let currentScheme = window.localStorage.getItem("data-theme");
	if (currentScheme === undefined) {
		const prefersDarkMode = matchMedia("(prefers-color-scheme: dark)").matches;
		currentScheme = prefersDarkMode ? "dark" : "light";
	}

	const newScheme = currentScheme === "dark" ? "light" : "dark";
	window.localStorage.setItem("data-theme", newScheme);
	document.documentElement.setAttribute("data-theme", newScheme);
}

const colorSchemeSwitcher = document.querySelector(".color-scheme-switch");
colorSchemeSwitcher.addEventListener("click", toggleColorSchemeSwitch, false);

//# Hide WIP banner after scrolling

const wipBanner = document.querySelector(".wip-banner");
if (wipBanner) {
	let hasScrolled = false;

	window.addEventListener(
		"scroll",
		() => {
			if (hasScrolled) {
				return;
			}

			if (window.scrollY > 0) {
				hasScrolled = true;
				setTimeout(
					() => {
						wipBanner.classList.add("hidden");
					},
					2_000,
				);
			}
		},
		{
			passive: true,
		},
	);
}

//# Team list shuffle

/**
 * @template T
 * @param {Array<T>} array
 * @returns {Array<T>}
 */
function randomShuffle(array) {
	let count = array.length;
	let temp;
	let index;
	while (count) {
		index = Math.floor(Math.random() * count--);
		temp = array[count];
		array[count] = array[index];
		array[index] = temp;
	}

	return array;
}

const teamList = document.querySelector(".team-list");
const teamArr = document.querySelectorAll(".team-list li");
if (teamArr.length > 0) {
	for (const li of randomShuffle(Array.from(teamArr))) {
		teamList.appendChild(li);
	}
}

//# Homepage example expander

const homepageExample = document.querySelector(".homepage-example");
if (homepageExample != null) {
	homepageExample.addEventListener(
		"click",
		() => {
			homepageExample.classList.remove("collapsed");
		},
	);
}
