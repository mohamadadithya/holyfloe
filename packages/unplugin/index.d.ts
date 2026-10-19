import type { GenerateOptions } from '@holyfloe/core';
import type { UnpluginInstance } from 'unplugin';

/**
 * Give `cem` (a manifest) or `lit` (a directory of Lit sources). `cem`, `lit`, `out` and `config`
 * are resolved from the Vite root (the working directory elsewhere).
 */
export type HolyfloeOptions = GenerateOptions;

/** `holyfloe.vite(options)`, `holyfloe.rollup(options)`, `holyfloe.webpack(options)`, ... */
declare const holyfloe: UnpluginInstance<HolyfloeOptions, false>;
export default holyfloe;
