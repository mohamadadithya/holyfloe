import { mount } from 'svelte';
import App from './App.svelte';
import Raw from './Raw.svelte';

// `?raw`: plain tags only (OQ-004), on their own page: Svelte caches an element's setters per tag
// name, so wrappers rendered first could change how the plain tags behave.
mount(location.search.includes('raw') ? Raw : App, { target: document.getElementById('root')! });
