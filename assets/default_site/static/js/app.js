const THEME_KEY = 'theme';
const DARK_QUERY = '(prefers-color-scheme: dark)';
const applyTheme = (theme) => {
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem(THEME_KEY, theme);
};
window.toggleTheme = () => {
    const currentTheme = document.documentElement.getAttribute('data-theme');
    const targetTheme = currentTheme === 'dark' ? 'light' : 'dark';
    applyTheme(targetTheme);
};
const initTheme = () => {
    const savedTheme = localStorage.getItem(THEME_KEY);
    if (savedTheme) {
        applyTheme(savedTheme);
    }
    else if (window.matchMedia && window.matchMedia(DARK_QUERY).matches) {
        applyTheme('dark');
    }
};
initTheme();
