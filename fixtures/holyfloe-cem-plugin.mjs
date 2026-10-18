// CEM analyzer plugin: copies `@model <prop> <event> [<path>]` class JSDoc tags into the
// manifest as `x-holyfloe-models`, which the analyzer would otherwise drop (D-062, D-063).
// holyfloe validates the values (E015); this only splits the tag text.
export default function holyfloeModels() {
  const found = new Map(); // `${module path}#${class name}` -> models
  return {
    name: 'holyfloe-models',
    analyzePhase({ ts, node, moduleDoc }) {
      if (!ts.isClassDeclaration(node) || !node.name) return;
      const models = ts
        .getJSDocTags(node)
        .filter((tag) => tag.tagName.text === 'model')
        .map((tag) => {
          const [prop, event, value] = (ts.getTextOfJSDocComment(tag.comment) ?? '').trim().split(/\s+/);
          return value ? { prop, event, value } : { prop, event };
        });
      if (models.length) found.set(`${moduleDoc.path}#${node.name.text}`, models);
    },
    packageLinkPhase({ customElementsManifest }) {
      for (const mod of customElementsManifest.modules ?? []) {
        for (const decl of mod.declarations ?? []) {
          const models = found.get(`${mod.path}#${decl.name}`);
          if (models) decl['x-holyfloe-models'] = models;
        }
      }
    },
  };
}
