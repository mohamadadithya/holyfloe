import { createApp } from 'vue';
import App from './App.vue';
import Raw from './Raw.vue';

// `?raw`: plain tags only (OQ-003), on their own page so the wrappers cannot influence them.
createApp(location.search.includes('raw') ? Raw : App).mount('#root');
