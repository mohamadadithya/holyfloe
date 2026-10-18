import { fileURLToPath } from 'node:url';

/** @type {import('next').NextConfig} */
export default {
  // The repo root: generated/ is a copy, but @holyfloe/fixtures links to ../../fixtures.
  turbopack: { root: fileURLToPath(new URL('../..', import.meta.url)) },
};
