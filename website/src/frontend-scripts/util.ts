export type ThemeName = "dark" | "light";

export function matchesDark() {
	return window.matchMedia("(prefers-color-scheme: dark)");
}

export function getCurrentTheme(): ThemeName {
	let currentScheme = window.localStorage.getItem("data-theme");
	if (currentScheme == null) {
		const prefersDarkMode = matchesDark().matches;
		currentScheme = prefersDarkMode ? "dark" : "light";
	}
	return currentScheme === "dark" ? "dark" : "light";
}
