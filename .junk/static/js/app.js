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

const searchInput = document.getElementById('search-input');
const resultsContainer = document.getElementById('search-results');
let searchIndex = [];

// 1. Load the search index from your .build directory
async function initSearch() {
  try {
    const response = await fetch('/search.json');
    searchIndex = await response.json();
  } catch (err) {
    console.error("Novos: Could not load search index.");
  }
}

// 2. The Search Logic
function performSearch(query) {
  const q = query.toLowerCase().trim();
  
  if (!q) {
    resultsContainer.style.display = 'none';
    return;
  }

  const matches = searchIndex.filter(post => {
    return post.title.toLowerCase().includes(q) || 
           post.tags.some(t => t.toLowerCase().includes(q)) ||
           post.snippet.toLowerCase().includes(q);
  });

  renderResults(matches);
}

// 3. Render the Glassmorphism Entries
function renderResults(results) {
  if (results.length === 0) {
    resultsContainer.innerHTML = '<div class="search-entry"><small>No results found.</small></div>';
  } else {
    resultsContainer.innerHTML = results.map(post => `
      <div class="search-entry">
        <a href="/doc/${post.slug}">${post.title}</a>
        <small>${post.date} • ${post.tags.join(', ')}</small>
        <p>${post.snippet.substring(0, 80)}...</p>
      </div>
    `).join('');
  }
  resultsContainer.style.display = 'block';
}

// 4. UX Helpers
// Close when clicking outside
document.addEventListener('click', (e) => {
  if (!searchInput.contains(e.target) && !resultsContainer.contains(e.target)) {
    resultsContainer.style.display = 'none';
  }
});

// Close on Escape key
document.addEventListener('keydown', (e) => {
  if (e.key === "Escape") {
    resultsContainer.style.display = 'none';
    searchInput.blur();
  }
});

// Run
searchInput?.addEventListener('input', e => performSearch(e.target.value));
initSearch();

document.addEventListener('keydown', (e) => {
  // 1. Check if the pressed key is '/'
  // 2. Ensure the user isn't already typing in an input/textarea/contentEditable
  if (e.key === '/' && !['INPUT', 'TEXTAREA'].includes(document.activeElement.tagName) && !document.activeElement.isContentEditable) {
    
    // Prevent the '/' character from actually being typed into the search bar
    e.preventDefault(); 
    
    searchInput.focus();
    
    // Optional: Scroll to top if your search bar is at the top
    window.scrollTo({ top: 0, behavior: 'smooth' });
  }
});


document.addEventListener('DOMContentLoaded', () => {
  const menuBtn = document.querySelector('.menu-toggle');
  const navLinks = document.querySelector('.nav-links');

  if (menuBtn && navLinks) {
    // 1. Toggle Menu
    menuBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      const isExpanded = menuBtn.getAttribute('aria-expanded') === 'true';
      
      menuBtn.setAttribute('aria-expanded', !isExpanded);
      navLinks.classList.toggle('is-active');
    });

    // 2. Click Outside to Close
    document.addEventListener('click', (e) => {
      if (navLinks.classList.contains('is-active') && !navLinks.contains(e.target) && !menuBtn.contains(e.target)) {
        navLinks.classList.remove('is-active');
        menuBtn.setAttribute('aria-expanded', 'false');
      }
    });

    // 3. Close on Escape Key
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && navLinks.classList.contains('is-active')) {
        navLinks.classList.remove('is-active');
        menuBtn.setAttribute('aria-expanded', 'false');
      }
    });
  }
});
