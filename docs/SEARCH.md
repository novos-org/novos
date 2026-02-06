# search.json Schema

search.json consists of 3 main parts:
	    The date, slug, a snippet (a few paragraphs), tags, and the title.
    Below is a snippet of search.json

```json
[
  {
    "date": "2026-02-04",
    "slug": "shortcodes",
    "snippet": "Shortcodes are reusable HTML components stored in your includes/shortcodes/ directory. They allow you to inject complex UI elements into Mar",
    "tags": [
      "markup",
      "shortcodes",
      "templates"
    ],
    "title": "shortcodes"
  },
  {
    "date": "2026-02-04",
    "slug": "cli",
    "snippet": "novos has various commands. Examples include: \"build\", \"init\", and \"serve\". serve  starts a local development server, whilst  build ... well",
    "tags": [
      "cli"
    ],
    "title": "Command Line Interface"
  }
]
```

The default `novos init` includes javascript snippets and a shortcode (`<% .search %>`) for usage. The snippets are shown below.

## Shortcode
> Usage: `{% .search %}`

```html
      <div class="flex items-center gap-4">
        <div class="search-area">
          <input
            type="text"
            id="search-input"
            placeholder="Search {%% 1 %%}..."
            autocomplete="off"
            aria-label="Search"
          >
          <div id="search-results" class="results-dropdown"></div>
        </div>
```

## JavaScript

```javascript
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

```