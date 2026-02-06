/**
 * search.js
 * Smart Search with Typo Tolerance and UX Refinements
 */

let searchIndex = [];
const searchInput = document.getElementById('search-input');
const resultsContainer = document.getElementById('search-results');

/**
 * 1. FUZZY LOGIC (Levenshtein Distance)
 */
function getEditDistance(a, b) {
  if (a.length === 0) return b.length;
  if (b.length === 0) return a.length;
  const matrix = [];
  for (let i = 0; i <= b.length; i++) matrix[i] = [i];
  for (let j = 0; j <= a.length; j++) matrix[0][j] = j;

  for (let i = 1; i <= b.length; i++) {
    for (let j = 1; j <= a.length; j++) {
      if (b.charAt(i - 1) === a.charAt(j - 1)) {
        matrix[i][j] = matrix[i - 1][j - 1];
      } else {
        matrix[i][j] = Math.min(
          matrix[i - 1][j - 1] + 1,
          matrix[i][j - 1] + 1,
          matrix[i - 1][j] + 1
        );
      }
    }
  }
  return matrix[b.length][a.length];
}

/**
 * 2. SCORING ENGINE
 */
function calculateScore(query, item) {
  const q = query.toLowerCase();
  let score = 0;

  if (item.title.toLowerCase().includes(q)) score += 100;
  if (item.tags?.some(t => t.toLowerCase().includes(q))) score += 70;
  if (item.snippet?.toLowerCase().includes(q)) score += 30;

  const words = item.title.toLowerCase().split(' ');
  words.forEach(word => {
    const distance = getEditDistance(q, word);
    if (distance <= 2 && q.length > 3) {
      score += (25 - distance * 10); 
    }
  });

  return score;
}

/**
 * 3. SEARCH EXECUTION & RENDERING
 */
function performSearch(query) {
  const q = query.toLowerCase().trim();
  if (!q) {
    resultsContainer.style.display = 'none';
    return;
  }

  const scoredMatches = searchIndex
    .map(post => ({ ...post, score: calculateScore(q, post) }))
    .filter(post => post.score > 0)
    .sort((a, b) => b.score - a.score);

  if (scoredMatches.length === 0) {
    resultsContainer.innerHTML = '<div class="search-entry"><small>No results found.</small></div>';
  } else {
    resultsContainer.innerHTML = scoredMatches.map(post => `
      <div class="search-entry">
        <a href="/posts/${post.slug}">${post.title}</a>
        <small>${post.date} ${post.score > 80 ? '*' : ''}</small>
        <p>${post.snippet?.substring(0, 80)}...</p>
      </div>
    `).join('');
  }
  resultsContainer.style.display = 'block';
}

/**
 * 4. UTILS & LISTENERS
 */
const debounce = (func, delay = 250) => {
  let timeout;
  return (...args) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => func.apply(this, args), delay);
  };
};

function setupListeners() {
  const debouncedSearch = debounce(val => performSearch(val));

  searchInput?.addEventListener('input', (e) => {
    if (!e.target.value.trim()) {
      resultsContainer.style.display = 'none';
    } else {
      debouncedSearch(e.target.value);
    }
  });

  document.addEventListener('keydown', (e) => {
    if (e.key === '/' && !['INPUT', 'TEXTAREA'].includes(document.activeElement.tagName) && !document.activeElement.isContentEditable) {
      e.preventDefault();
      window.scrollTo({ top: 0, behavior: 'smooth' });
      searchInput.focus();
    }

    if (e.key === 'Escape') {
      searchInput.value = '';
      searchInput.blur();
      resultsContainer.style.display = 'none';
    }
  });

  document.addEventListener('click', (e) => {
    if (!searchInput.contains(e.target) && !resultsContainer.contains(e.target)) {
      resultsContainer.style.display = 'none';
    }
  });
}

/**
 * 5. EXPORT INITIALIZATION
 */
export async function initSearch() {
  if (!searchInput) return;
  try {
    const response = await fetch('/search.json');
    searchIndex = await response.json();
    setupListeners();
  } catch (err) {
    console.error("Search index failed:", err);
  }
}
