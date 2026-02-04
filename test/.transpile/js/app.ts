/**
 * Custom Interface for HTMX Request Events
 * This tells TypeScript that HTMX events have a 'detail' object 
 * with a 'parameters' record.
 */
interface HTMXConfigRequestEvent extends CustomEvent {
  detail: {
    parameters: Record<string, string>;
  };
}

/**
 * 1. HTMX Configuration
 * Listens for requests and injects the 'variant' parameter 
 * if the user is on a mobile device.
 */
document.addEventListener('htmx:configRequest', (event: Event) => {
  const htmxEvent = event as HTMXConfigRequestEvent;
  const userAgent: string = navigator.userAgent;

  if (/Mobi|Android/i.test(userAgent)) {
    htmxEvent.detail.parameters['variant'] = 'mobile';
  }
});

/**
 * 2. Theme Management Logic
 */
const THEME_KEY = 'theme';
const DARK_QUERY = '(prefers-color-scheme: dark)';

/**
 * Sets the theme attribute on the document root and saves to localStorage
 */
const applyTheme = (theme: 'dark' | 'light'): void => {
  document.documentElement.setAttribute('data-theme', theme);
  localStorage.setItem(THEME_KEY, theme);
};

/**
 * Toggles between dark and light modes.
 * Attached to the window object so it's accessible from HTML onclick.
 */
(window as any).toggleTheme = (): void => {
  const currentTheme = document.documentElement.getAttribute('data-theme');
  const targetTheme = currentTheme === 'dark' ? 'light' : 'dark';
  applyTheme(targetTheme);
};

/**
 * 3. Initialization
 * Checks localStorage first, then falls back to system preferences.
 */
const initTheme = (): void => {
  const savedTheme = localStorage.getItem(THEME_KEY) as 'dark' | 'light' | null;

  if (savedTheme) {
    applyTheme(savedTheme);
  } else if (window.matchMedia && window.matchMedia(DARK_QUERY).matches) {
    // Optional: Follow system preference if no manual choice was made
    applyTheme('dark');
  }
};

// Fire initialization when the script loads
initTheme();
