// @ts-check
"use strict";

//# Responsive width
let isMobile = false;
window.addEventListener("DOMContentLoaded", () => {
	const mobileMatchMedia = matchMedia("(max-width: 768px)");
	isMobile = mobileMatchMedia.matches;

	mobileMatchMedia.addListener((e) => {
		isMobile = e.matches;

		// Close the mobile sidebar when switching from mobile to desktop
		if (isMobileNavVisible && !isMobile && isMobileNavVisible) {
			toggleMobileSidebar();
		}
	});
});

//# Table of Contents
const originalTitle = document.title;
const headerMobile = document.querySelector(".header-mobile");

/** @type {HTMLElement}*/
const tocSidebar = document.querySelector(".toc-sidebar");

/** @type {Array<HTMLAnchorElement>}*/
const tocLinks = Array.from(document.querySelectorAll(".toc a"));

/** @type {Array<{
 * heading: HTMLElement,
 * link: HTMLAnchorElement,
 * }>} */
const headingElements = tocLinks.map((link) => {
	return {
		heading: document.querySelector(
			`[id="${link.getAttribute("href").slice(1)}"]`,
		),
		link,
	};
});

/**
 * @typedef {Object} CalculatedHeading
 * @property {string} id
 * @property {HTMLAnchorElement} link
 * @property {Array<string>} titles
 * @property {number} level
 * @property {number} start
 * @property {number} end
 */

class Manager {
	constructor() {
		/** @type {Array<CalculatedHeading>}*/
		this.headingsCalculated = [];

		/** @type {boolean}*/
		this.hasInitializedHeadings = false;

		/** @type {undefined | number}*/
		this.lastActiveHeading = undefined;

		/** @type {boolean}*/
		this.isNavCollapsed = false;

		/** @type {undefined | number}*/
		this.navHeight = undefined;
	}

	/**
	 * @param {MouseEvent} event
	 */
	handleTOCClick(event) {
		const target = event.target;
		event.preventDefault();
		if (!(target instanceof HTMLElement)) {
			return;
		}

		if (target.hasAttribute("href")) {
			const hash = target.getAttribute("href");
			window.location.hash = hash;
			this.scrollToHeading(hash);

			if (isMobile) {
				toggleMobileSidebar();
			}
		}
	}

	/**
	 * @returns {number}
	 */
	getScrollY() {
		return scrollY + this.getScrollOffset();
	}

	/**
	 * @returns {number}
	 */
	getScrollOffset() {
		let offset = 0;

		// Account for header
		offset += isMobile ? 64 : 80;

		// Give everything a tiny bit of margin so it's not up against the edges
		offset += 20;

		return offset;
	}

	/**
	 * @param {HTMLElement} heading
	 * @returns {number}
	 */
	getHeadingTop(heading) {
		return heading.offsetTop - this.getScrollOffset();
	}

	/**
	 * @param {number} i
	 * @param {Array<CalculatedHeading>} stack
	 * @returns {CalculatedHeading}
	 */
	calculateHeading(i, stack) {
		const { heading, link } = headingElements[i];
		const id = heading.getAttribute("id");

		// Extract the level from the H tag
		const level = Number(heading.tagName[1]);

		// Get the headings above this one for us in document.title
		/** @type {Array<string>}*/
		let titles = [heading.textContent.trim()];
		for (let i = stack.length - 1; i >= 0; i--) {
			const heading = stack[i];
			if (heading.level < level) {
				titles = heading.titles.concat(titles);
				break;
			}
		}

		// Calculate when this heading ends. It's either at the beginning of the next heading, or page bottom.
		let start = this.getHeadingTop(heading);
		let end;

		const nextHeading = headingElements[i + 1];
		if (nextHeading) {
			end = this.getHeadingTop(nextHeading.heading);
		} else {
			end = document.body.clientHeight;
		}

		return { level, id, titles, link, start, end };
	}

	calculateHeadingsPositions() {
		// Don't calculate heading positions unless we've scrolled down
		if (!this.hasInitializedHeadings && scrollY <= 100) {
			return;
		}

		// If we've calculated all the headings then we just need to validate the last one
		// and if it's the same we can skip.
		if (this.hasInitializedHeadings) {
			const i = headingElements.length - 1;
			const existing = this.headingsCalculated[i];
			if (existing !== undefined) {
				const recalculated = this.calculateHeading(i, []);
				if (
					recalculated.start === existing.start &&
					recalculated.end === existing.end
				) {
					return;
				}
			}
		}

		this.hasInitializedHeadings = true;
		this.headingsCalculated = [];
		for (let i = 0; i < headingElements.length; i++) {
			this.headingsCalculated.push(
				this.calculateHeading(i, this.headingsCalculated),
			);
		}
	}

	/**
	 * Check if a heading is currently in view
	 *
	 * @param {number} i
	 * @returns {boolean}
	 */
	isVisibleHeading(i) {
		const { start, end } = this.headingsCalculated[i];
		const scrollY = this.getScrollY();
		return scrollY >= start && scrollY <= end;
	}

	/**
	 * @param {number} i
	 * @param {boolean} activating
	 */
	toggleActiveHeading(i, activating) {
		const { link, titles } = this.headingsCalculated[i];

		// Only automatically rewrite the heading on the homepage
		if (location.pathname === "/") {
			if (activating) {
				document.title = `${titles.join(": ")} — ${originalTitle}`;
				history.replaceState({}, "", link.getAttribute("href"));
			} else {
				document.title = originalTitle;
			}
		}

		/** @type {null | Element}*/
		let target = link;
		while (target != null && target.tagName !== "DIV") {
			if (target.tagName === "LI") {
				target.classList.toggle("active");
			}
			target = target.parentElement;
		}
	}

	/**
	 * Computing the heading positions is expensive so we only do it when we absolutely have to.
	 */
	ensureCalculatedHeadings() {
		if (!this.hasInitializedHeadings) {
			this.calculateHeadingsPositions();
		}
	}

	/**
	 * Triggered on window resize and scroll. This needs to be very fast, avoiding DOM inspection completely by caching
	 * and then validating.
	 *
	 * This checks if we should collapse the navigation, and what heading to highlight in the table of contents
	 */

	refresh() {
		if (this.lastActiveHeading !== undefined) {
			if (this.isVisibleHeading(this.lastActiveHeading)) {
				return;
			} else {
				this.toggleActiveHeading(this.lastActiveHeading, false);
				this.lastActiveHeading = undefined;
			}
		}

		this.ensureCalculatedHeadings();
		for (let i = 0; i < this.headingsCalculated.length; i++) {
			if (this.isVisibleHeading(i)) {
				// Set the heading as active
				this.lastActiveHeading = i;
				this.toggleActiveHeading(i, true);

				// Make sure TOC link is visible
				let linkTop =
					this.headingsCalculated[i].link.offsetTop - tocSidebar.offsetTop;
				if (i === 0) {
					linkTop = 0;
				}
				const visibleStart = tocSidebar.scrollTop;
				const visibleEnd = tocSidebar.scrollTop + tocSidebar.clientHeight;
				const isVisible = linkTop > visibleStart && linkTop < visibleEnd;
				if (!isVisible) {
					tocSidebar.scrollTop = linkTop;
				}

				break;
			}
		}
	}

	/**
	 * @param {string} hash
	 * @param {undefined | (() => void)} callback
	 * @returns {boolean}
	 */
	scrollToHeading(hash, callback) {
		// Allow passing in raw link href
		const id = hash.replace(/^(#)/, "");

		const heading = document.getElementById(id);
		if (!heading) {
			return false;
		}

		// rome-ignore lint/js/preferOptionalChaining: netlify's node version does not support optional call expressions
		if (callback !== undefined) {
			callback();
		}

		heading.setAttribute("tabindex", "-1");
		heading.focus();

		this.ensureCalculatedHeadings();
		window.scrollTo(0, this.getHeadingTop(heading));

		return true;
	}

	/**
	 * Fully scroll and copy hash to tech when clicking an anchor next to a heading
	 *
	 * @param {MouseEvent} event
	 * @param {HTMLElement} target
	 */
	handleHeadingAnchorClick(event, target) {
		event.preventDefault();

		const hash = target.getAttribute("href");
		window.location.hash = hash;
		this.scrollToHeading(hash);

		// rome-ignore lint/js/preferOptionalChaining: netlify's node version does not support optional call expressions
		if (navigator.clipboard !== undefined) {
			navigator.clipboard.writeText(window.location.href);
		}

		// Only another copied text can appear here so delete it if it exists
		// rome-ignore lint/js/preferOptionalChaining: netlify's node version does not support optional call expressions
		if (target.nextElementSibling != null) {
			target.nextElementSibling.remove();
		}

		const copied = document.createElement("span");
		copied.classList.add("header-copied");
		copied.textContent = "Copied to clipboard";
		target.parentElement.appendChild(copied);
		requestAnimationFrame(() => {
			requestAnimationFrame(() => {
				copied.style.opacity = "0";
			});
		});
		copied.addEventListener("transitionend", () => {
			copied.remove();
		});
	}

	/**
	 * Intercept link clicks, if they are just hashes on the current page then
	 * just scroll
	 *
	 * @param {MouseEvent} event
	 * @param {HTMLElement} target
	 */
	handleAnchorClick(event, target) {
		let href = target.getAttribute("href");
		if (href === undefined) {
			return;
		}

		// Remove current origin
		if (href.startsWith(location.origin)) {
			href = href.slice(location.origin.length);
		}

		// Remove current pathname
		if (href.startsWith(location.pathname)) {
			href = href.slice(location.pathname.length);
		}

		// If href starts with a hash then it's referring to the current page
		if (href[0] !== "#") {
			return;
		}

		this.scrollToHeading(href, function () {
			event.preventDefault();
			location.hash = href;
		});
	}

	/**
	 * @param {MouseEvent} event
	 */
	handleGlobalClick(event) {
		const { target } = event;
		if (!(target instanceof HTMLElement)) {
			return;
		}

		if (event.ctrlKey || event.metaKey) {
			return;
		}

		if (target.closest(".toc") != null) {
			this.handleTOCClick(event);
		} else if (target.matches('.content a[href^="#"]')) {
			this.handleHeadingAnchorClick(event, target);
		} else if (target.matches("a")) {
			this.handleAnchorClick(event, target);
		}
	}

	attach() {
		this.refresh();

		if (window.location.hash !== "") {
			this.scrollToHeading(window.location.hash);
		}

		window.addEventListener("scroll", this.refresh.bind(this), {
			passive: true,
		});
		window.addEventListener("resize", this.refresh.bind(this), {
			passive: true,
		});
		window.addEventListener(
			"resize",
			this.calculateHeadingsPositions.bind(this),
			{ passive: true },
		);

		document.addEventListener(
			"click",
			this.handleGlobalClick.bind(this),
			false,
		);
	}
}

if (tocLinks.length > 0) {
	const manager = new Manager();

	window.addEventListener("DOMContentLoaded", () => {
		manager.attach();
	});
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

const creditsPeopleLists = document.querySelectorAll(".credits-people-list");
for (const list of creditsPeopleLists) {
	const items = list.querySelectorAll("li");
	for (const li of randomShuffle(Array.from(items))) {
		list.appendChild(li);
	}
}

//# Code expanders

const collapsed = document.querySelectorAll("pre.collapsed");
for (const elem of collapsed) {
	elem.addEventListener("click", () => {
		elem.classList.remove("collapsed");
	});
}

//# Color scheme switcher

function matchesDark() {
	return window.matchMedia("(prefers-color-scheme: dark)");
}

function getCurrentTheme() {
	let currentScheme = window.localStorage.getItem("data-theme");
	if (currentScheme == null) {
		const prefersDarkMode = matchesDark().matches;
		currentScheme = prefersDarkMode ? "dark" : "light";
	}
	return currentScheme;
}

function toggleColorSchemeSwitch(evt) {
	const currentScheme = getCurrentTheme();
	const newScheme = currentScheme === "dark" ? "light" : "dark";
	window.localStorage.setItem("data-theme", newScheme);
	evt.currentTarget.setAttribute("aria-checked", newScheme === "dark");
	document.documentElement.classList.add("transition");
	document.documentElement.setAttribute("data-theme", newScheme);
	onColorSchemeChange();
}

function onColorSchemeChange() {
	window.dispatchEvent(new Event("colorschemechange"));
}

const colorSchemeSwitcher = document.querySelector(".color-scheme-switch");
// rome-ignore lint/js/preferOptionalChaining: netlify's node version does not support optional call expressions
if (colorSchemeSwitcher != null) {
	colorSchemeSwitcher.addEventListener("click", toggleColorSchemeSwitch, false);
}

matchesDark().addEventListener("change", () => {
	onColorSchemeChange();
});

//# Mobile navigation

const mobileSidebarHandle = document.querySelector(".mobile-handle");
const mobileActiveTargets = document.querySelectorAll(
	".page-header, .page-header-mobile, .docs-sidebar",
);
let isMobileNavVisible = false;
function toggleMobileSidebar() {
	isMobileNavVisible = !isMobileNavVisible;
	mobileSidebarHandle.classList.toggle("active");
	document.body.classList.toggle("no-scroll");
	if (isMobileNavVisible) {
		for (const elem of mobileActiveTargets) {
			elem.classList.add("mobile-active");
		}
	} else {
		for (const elem of mobileActiveTargets) {
			elem.classList.remove("mobile-active");
		}
	}
}
// rome-ignore lint/js/preferOptionalChaining: netlify's node version does not support optional call expressions
if (mobileSidebarHandle != null) {
	mobileSidebarHandle.addEventListener(
		"click",
		(event) => {
			event.preventDefault();
			toggleMobileSidebar();
		},
		false,
	);
}

//# Header scrolls to top
let topAnchors = Array.from(document.querySelectorAll("[href='#top']"));
if (location.pathname === "/") {
	topAnchors = [...topAnchors, ...document.querySelectorAll(".logo")];
}
for (const elem of topAnchors) {
	elem.addEventListener("click", (e) => {
		if (window.scrollY > 0) {
			e.preventDefault();

			if (tocSidebar != null) {
				tocSidebar.scrollTop = 0;
			}

			window.scrollTo(0, 0);

			// Remove the hash
			history.pushState(
				"",
				document.title,
				window.location.pathname + window.location.search,
			);
		}
	});
}
