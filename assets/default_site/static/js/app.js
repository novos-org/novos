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
document.querySelectorAll('pre').forEach((block) => {
  // 1. Create the button
  const button = document.createElement('button');
  button.innerText = 'Copy';
  button.className = 'copy-code-button';
  button.type = 'button'; // Prevent accidental form submits

  // 2. Inject it
  block.appendChild(button);

  // 3. Logic
  button.addEventListener('click', async () => {
    const code = block.querySelector('code');
    const text = code ? code.innerText : block.innerText;

    try {
      await navigator.clipboard.writeText(text);
      
      // Feedback State
      button.innerText = 'Copied!';
      button.classList.add('active');
      
      // Reset
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
