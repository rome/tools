export let isMobile = false;
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

//# Mobile navigation

const mobileSidebarHandle = document.querySelector(".mobile-handle");
const mobileActiveTargets = document.querySelectorAll(
	".page-header, .page-header-mobile, .docs-sidebar",
);
let isMobileNavVisible = false;
export function toggleMobileSidebar() {
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
