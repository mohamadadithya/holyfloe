// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// GitHub Pages project site (D-099). Internal links in content must include the base.
export default defineConfig({
  site: 'https://mohamadadithya.github.io',
  base: '/holyfloe',
  integrations: [
    starlight({
      title: 'holyfloe',
      logo: { src: './src/assets/logo.svg' },
      customCss: ['./src/styles/theme.css'],
      // Starlight already writes og:title/description/url and twitter:card. Social sites need a raster image.
      head: [
        { tag: 'meta', attrs: { property: 'og:image', content: 'https://mohamadadithya.github.io/holyfloe/og.png' } },
        { tag: 'meta', attrs: { property: 'og:image:width', content: '1200' } },
        { tag: 'meta', attrs: { property: 'og:image:height', content: '630' } },
        { tag: 'meta', attrs: { property: 'og:image:alt', content: 'holyfloe: three violet petals inside a gold halo' } },
        { tag: 'meta', attrs: { name: 'twitter:image', content: 'https://mohamadadithya.github.io/holyfloe/og.png' } },
        { tag: 'link', attrs: { rel: 'apple-touch-icon', href: '/holyfloe/apple-touch-icon.png' } },
        { tag: 'meta', attrs: { name: 'theme-color', content: '#6D28D9' } },
      ],
      description: 'Typed React, Vue and Svelte wrappers for Lit web components.',
      social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/mohamadadithya/holyfloe' }],
      editLink: { baseUrl: 'https://github.com/mohamadadithya/holyfloe/edit/main/site/' },
      sidebar: [
        { label: 'Getting started', slug: 'getting-started' },
        {
          label: 'Guides',
          items: [
            'guides/cli',
            'guides/bundler-plugin',
            'guides/node-api',
            'guides/documenting-components',
          ],
        },
        { label: 'Frameworks', items: ['frameworks/react', 'frameworks/vue', 'frameworks/svelte'] },
        {
          label: 'Reference',
          items: ['reference/configuration', 'reference/errors', 'reference/known-limits'],
        },
      ],
    }),
  ],
});
