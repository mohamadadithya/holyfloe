import holyfloeModels from '@holyfloe/core/cem-plugin';

// Per-fixture --globs and --outdir come from `just gen-fixtures-cem`.
export default { litelement: true, packagejson: false, plugins: [holyfloeModels()] };
