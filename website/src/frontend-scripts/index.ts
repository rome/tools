import {matchesDark, getCurrentTheme} from "./util";
import "./toc.ts";

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

//# Team list shuffle

/**
 * @template T
 * @param {Array<T>} array
 * @returns {Array<T>}
 */
function randomShuffle<T>(array: T[]): T[] {
	let count = array.length;
	let index;
	while (count) {
		index = Math.floor(Math.random() * count--);
		const temp = array[count]!;
		array[count] = array[index]!;
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

function toggleColorSchemeSwitch(evt: Event) {
	const currentScheme = getCurrentTheme();
	const newScheme = currentScheme === "dark" ? "light" : "dark";
	window.localStorage.setItem("data-theme", newScheme);
	
	if (evt.currentTarget instanceof Element) {
		evt.currentTarget.setAttribute("aria-checked", String(newScheme === "dark"));
	}

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
	if (mobileSidebarHandle != null) {
		mobileSidebarHandle.classList.toggle("active");
	}
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
