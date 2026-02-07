/**
 * main.js
 * Global site features: Theme, Navigation, and Code Copying
 */

// --- Theme Management ---
const THEME_KEY = 'theme';
const DARK_QUERY = '(prefers-color-scheme: dark)';

export const initTheme = () => {
  const applyTheme = (theme) => {
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem(THEME_KEY, theme);
  };

  const savedTheme = localStorage.getItem(THEME_KEY);
  if (savedTheme) {
    applyTheme(savedTheme);
  } else if (window.matchMedia(DARK_QUERY).matches) {
    applyTheme('dark');
  }

  // Attach toggle to window so it's accessible from HTML onclick
  window.toggleTheme = () => {
    const currentTheme = document.documentElement.getAttribute('data-theme');
    applyTheme(currentTheme === 'dark' ? 'light' : 'dark');
  };
};

// --- Mobile Navigation ---
export const initNav = () => {
  const menuBtn = document.querySelector('.menu-toggle');
  const navLinks = document.querySelector('.nav-links');

  if (!menuBtn || !navLinks) return;

  const toggleMenu = (state) => {
    const isExpanded = state ?? navLinks.classList.toggle('is-active');
    menuBtn.setAttribute('aria-expanded', isExpanded);
    if (!isExpanded) navLinks.classList.remove('is-active');
  };

  menuBtn.addEventListener('click', (e) => {
    e.stopPropagation();
    toggleMenu();
  });

  document.addEventListener('click', (e) => {
    if (!navLinks.contains(e.target) && !menuBtn.contains(e.target)) toggleMenu(false);
  });

  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') toggleMenu(false);
  });
};

// --- Code Block Copy Buttons ---
export const initCopyButtons = () => {
  document.querySelectorAll('pre').forEach((block) => {
    if (block.querySelector('.copy-code-button')) return; // Avoid double injection

    const button = document.createElement('button');
    button.innerText = 'Copy';
    button.className = 'copy-code-button';
    button.type = 'button';
    block.appendChild(button);

    button.addEventListener('click', async () => {
      const code = block.querySelector('code');
      const text = code ? code.innerText : block.innerText;

      try {
        await navigator.clipboard.writeText(text);
        button.innerText = 'Copied!';
        button.classList.add('active');
        setTimeout(() => {
          button.innerText = 'Copy';
          button.classList.remove('active');
        }, 2000);
      } catch (err) {
        button.innerText = 'Error';
        console.error('Copy failed', err);
      }
    });
  });
};
