/**
 * app.js
 * The main entry point that orchestrates modules.
 */

import { initTheme, initNav, initCopyButtons } from './main.js';
import { initSearch } from './search.js';

/**
 * Initialize all site features
 */
const bootstrap = () => {
  try {
    // 1. Core UI features
    initTheme();
    initNav();
    initCopyButtons();

    // 2. Search functionality
    initSearch();

    console.log('App initialized successfully');
  } catch (error) {
    console.error('Entry point initialization failed:', error);
  }
};

// Run when the DOM is fully loaded
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', bootstrap);
} else {
  bootstrap();
}
