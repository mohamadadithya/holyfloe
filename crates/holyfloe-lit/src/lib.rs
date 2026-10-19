//! Reads Lit components with oxc and builds the Custom Elements Manifest the CEM analyzer would
//! write for them (the fields `holyfloe-ir` reads). Callers parse it with `holyfloe_cem::parse`
//! and build the IR as for analyzer output, so both inputs share discovery, module resolution
//! and flatten (D-089). Types are copied from the source text, never resolved (D-013).

mod jsdoc;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Component, Path, PathBuf};

use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_ast_visit::{Visit, walk};
use oxc_parser::Parser;
use oxc_resolver::{ResolveOptions, Resolver, TsconfigDiscovery};
use oxc_span::{GetSpan, SourceType, Span};
use serde_json::{Map, Value, json};

use jsdoc::{Doc, tag_body};

#[derive(Debug, thiserror::Error)]
pub enum LitError {
    #[error("{path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{path}:{line}: {message}")]
    Parse {
        path: PathBuf,
        line: usize,
        message: String,
    },
}

/// Lit lifecycle members the analyzer leaves out of manifests; leaving them out too keeps parity.
const DENIED: &[&str] = &[
    "requestUpdate",
    "createRenderRoot",
    "scheduleUpdate",
    "performUpdate",
    "shouldUpdate",
    "update",
    "render",
    "firstUpdated",
    "updated",
    "willUpdate",
    "properties",
    "styles",
];

/// Decorators whose fields are internal state, not props.
const INTERNAL_DECORATORS: &[&str] = &[
    "state",
    "query",
    "queryAll",
    "queryAsync",
    "queryAssignedElements",
    "queryAssignedNodes",
];

/// The manifest for `files`, with module paths relative to `root` (the package directory).
/// A superclass or mixin in another file of this package is read too, even if `files` lacks it.
pub fn extract(root: &Path, files: &[PathBuf]) -> Result<Value, LitError> {
    let root = fs::canonicalize(root).map_err(|source| LitError::Io {
        path: root.to_owned(),
        source,
    })?;
    let resolver = Resolver::new(ResolveOptions {
        extensions: [".ts", ".tsx", ".mts", ".js", ".mjs", ".jsx"]
            .map(String::from)
            .into(),
        extension_alias: vec![
            (
                ".js".into(),
                vec![".ts".into(), ".tsx".into(), ".js".into()],
            ),
            (".mjs".into(), vec![".mts".into(), ".mjs".into()]),
        ],
        condition_names: vec!["import".into(), "default".into()],
        tsconfig: Some(TsconfigDiscovery::Auto),
        ..ResolveOptions::default()
    });
    let mut queue: Vec<PathBuf> = Vec::new();
    for f in files {
        queue.push(fs::canonicalize(f).map_err(|source| LitError::Io {
            path: f.clone(),
            source,
        })?);
    }
    queue.sort();
    queue.reverse();
    let mut done = BTreeSet::new();
    let mut modules = BTreeMap::new();
    while let Some(file) = queue.pop() {
        if !done.insert(file.clone()) {
            continue;
        }
        let ctx = Ctx {
            root: &root,
            resolver: &resolver,
            file: &file,
            path: module_path(&root, &file),
        };
        let (module, follow) = ctx.module()?;
        modules.insert(ctx.path.clone(), module);
        queue.extend(follow.into_iter().filter(|f| !done.contains(f)));
    }
    let manifest =
        json!({ "schemaVersion": "2.1.0", "modules": modules.into_values().collect::<Vec<_>>() });
    Ok(finish(manifest))
}

fn module_path(root: &Path, file: &Path) -> String {
    let rel = file.strip_prefix(root).unwrap_or(file);
    rel.components()
        .filter_map(|c| match c {
            Component::Normal(p) => Some(p.to_string_lossy()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

struct Ctx<'c> {
    root: &'c Path,
    resolver: &'c Resolver,
    file: &'c Path,
    /// Module path relative to the package root, as the manifest writes it.
    path: String,
}

/// Where an imported name comes from.
struct Import {
    specifier: String,
    /// Name in the exporting module; `default` for a default import.
    name: String,
}

/// Per-module state while walking the AST.
struct Module<'s> {
    src: &'s str,
    comments: &'s [Comment],
    imports: HashMap<String, Import>,
    /// Type aliases and interfaces declared here, referenced by event detail types.
    local_types: BTreeSet<String>,
    /// Files of this package that hold a superclass or mixin, to extract next.
    follow: Vec<PathBuf>,
}

impl Ctx<'_> {
    fn module(&self) -> Result<(Value, Vec<PathBuf>), LitError> {
        let src = fs::read_to_string(self.file).map_err(|source| LitError::Io {
            path: self.file.to_owned(),
            source,
        })?;
        let alloc = Allocator::default();
        let source_type = SourceType::from_path(self.file).unwrap_or_default();
        let ret = Parser::new(&alloc, &src, source_type).parse();
        if let Some(err) = ret.diagnostics.errors().next() {
            let at = err.labels.first().map_or(0, |l| l.offset());
            return Err(LitError::Parse {
                path: self.file.to_owned(),
                line: src[..(at as usize).min(src.len())].lines().count().max(1),
                message: err.message.to_string(),
            });
        }
        let program = ret.program;
        let mut m = Module {
            src: &src,
            comments: &program.comments,
            imports: HashMap::new(),
            local_types: BTreeSet::new(),
            follow: Vec::new(),
        };
        for stmt in &program.body {
            match stmt {
                Statement::ImportDeclaration(d) => {
                    for s in d.specifiers.iter().flatten() {
                        let (local, name) = match s {
                            ImportDeclarationSpecifier::ImportSpecifier(s) => {
                                (s.local.name.as_str(), s.imported.name().to_string())
                            }
                            ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => {
                                (s.local.name.as_str(), "default".to_owned())
                            }
                            ImportDeclarationSpecifier::ImportNamespaceSpecifier(_) => continue,
                        };
                        m.imports.insert(
                            local.to_owned(),
                            Import {
                                specifier: d.source.value.to_string(),
                                name,
                            },
                        );
                    }
                }
                Statement::TSTypeAliasDeclaration(d) => {
                    m.local_types.insert(d.id.name.to_string());
                }
                Statement::TSInterfaceDeclaration(d) => {
                    m.local_types.insert(d.id.name.to_string());
                }
                Statement::ExportDeclaration(d) => match &d.declaration {
                    Declaration::TSTypeAliasDeclaration(t) => {
                        m.local_types.insert(t.id.name.to_string());
                    }
                    Declaration::TSInterfaceDeclaration(t) => {
                        m.local_types.insert(t.id.name.to_string());
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let mut declarations = Vec::new();
        let mut exports = Vec::new();
        for stmt in &program.body {
            let start = stmt.span().start;
            match stmt {
                Statement::ClassDeclaration(c) => {
                    declarations.extend(self.class(&mut m, c, start, None));
                }
                Statement::ExportDeclaration(d) => match &d.declaration {
                    Declaration::ClassDeclaration(c) => {
                        if let Some(decl) = self.class(&mut m, c, start, None) {
                            exports.push(self.js_export(&decl["name"], &decl["name"]));
                            exports.extend(self.definition(&decl));
                            declarations.push(decl);
                        }
                    }
                    Declaration::VariableDeclaration(v) => {
                        for d in &v.declarations {
                            if let Some(decl) = self.mixin_var(&mut m, d, start) {
                                exports.push(self.js_export(&decl["name"], &decl["name"]));
                                declarations.push(decl);
                            }
                        }
                    }
                    Declaration::FunctionDeclaration(f) => {
                        if let Some(decl) = self.mixin_fn(&mut m, f, start) {
                            exports.push(self.js_export(&decl["name"], &decl["name"]));
                            declarations.push(decl);
                        }
                    }
                    _ => {}
                },
                Statement::ExportDefaultDeclaration(d) => {
                    if let ExportDefaultDeclarationKind::ClassDeclaration(c) = &d.declaration
                        && let Some(decl) = self.class(&mut m, c, start, None)
                    {
                        exports.push(self.js_export(&json!("default"), &decl["name"]));
                        exports.extend(self.definition(&decl));
                        declarations.push(decl);
                    }
                }
                Statement::VariableDeclaration(v) => {
                    for d in &v.declarations {
                        declarations.extend(self.mixin_var(&mut m, d, start));
                    }
                }
                Statement::FunctionDeclaration(f) => {
                    declarations.extend(self.mixin_fn(&mut m, f, start));
                }
                Statement::ExportNamedDeclaration(d) => {
                    for s in &d.specifiers {
                        let local = s.local.name().to_string();
                        let exported = s.exported.name().to_string();
                        let decl = self.reference(&mut m, &local);
                        exports
                            .push(json!({ "kind": "js", "name": exported, "declaration": decl }));
                    }
                }
                Statement::ExportFromDeclaration(d) => {
                    for s in &d.specifiers {
                        let local = s.local.name().to_string();
                        let exported = s.exported.name().to_string();
                        let decl = self.import_ref(&mut m, &d.source.value, &local);
                        exports
                            .push(json!({ "kind": "js", "name": exported, "declaration": decl }));
                    }
                }
                _ => {}
            }
        }
        // `customElements.define('x-y', XY)`, anywhere in the module (fx-lazy defines in a function).
        let mut defines = Defines::default();
        defines.visit_program(&program);
        for (tag, class) in defines.found {
            // The defined tag beats a JSDoc `@tag`, as in the analyzer's tag linking.
            if let Some(d) = declarations
                .iter_mut()
                .find(|d| d["name"] == class.as_str())
                && let Some(o) = d.as_object_mut()
            {
                o.insert("tagName".into(), json!(tag));
            }
            let declaration = self.reference(&mut m, &class);
            exports.push(json!({ "kind": "custom-element-definition", "name": tag, "declaration": declaration }));
        }
        let module = json!({
            "kind": "javascript-module",
            "path": self.path,
            "declarations": declarations,
            "exports": exports,
        });
        Ok((module, m.follow))
    }

    fn js_export(&self, name: &Value, class: &Value) -> Value {
        json!({ "kind": "js", "name": name, "declaration": { "name": class, "module": self.path } })
    }

    /// The definition export a `@customElement` class gets, as the analyzer writes it.
    fn definition(&self, decl: &Value) -> Option<Value> {
        let tag = decl.get("tagName")?;
        decl.get("x-defined-by-decorator")?;
        Some(json!({
            "kind": "custom-element-definition",
            "name": tag,
            "declaration": { "name": decl["name"], "module": self.path },
        }))
    }

    /// A `{ name, module | package }` reference for a name used in this module.
    fn reference(&self, m: &mut Module, name: &str) -> Value {
        match m.imports.get(name) {
            Some(i) => {
                let (spec, imported) = (i.specifier.clone(), i.name.clone());
                self.import_ref(m, &spec, &imported)
            }
            None => json!({ "name": name, "module": self.path }),
        }
    }

    /// `name` exported by `specifier`. A file of this package is a module path (and gets
    /// extracted too); anything else is a package specifier, which `holyfloe-ir` looks up in
    /// dependency manifests. Unresolvable relative imports stay relative, as the analyzer writes
    /// them, and the IR reports a missing class.
    fn import_ref(&self, m: &mut Module, specifier: &str, name: &str) -> Value {
        let dir = self.file.parent().unwrap_or(self.root);
        match self.resolver.resolve(dir, specifier) {
            Ok(r) if self.is_local(r.path()) => {
                m.follow.push(r.path().to_owned());
                json!({ "name": name, "module": module_path(self.root, r.path()) })
            }
            _ if specifier.starts_with('.') => json!({ "name": name, "module": specifier }),
            _ => json!({ "name": name, "package": specifier }),
        }
    }

    /// In this package: under the root, not in `node_modules`, and no other named package in
    /// between (fixtures/src/_dep is its own package).
    fn is_local(&self, path: &Path) -> bool {
        let Ok(rel) = path.strip_prefix(self.root) else {
            return false;
        };
        if rel.components().any(|c| c.as_os_str() == "node_modules") {
            return false;
        }
        let mut dir = path.parent();
        while let Some(d) = dir {
            if d == self.root {
                return true;
            }
            if fs::read_to_string(d.join("package.json")).is_ok_and(|s| {
                serde_json::from_str::<Value>(&s).is_ok_and(|v| v.get("name").is_some())
            }) {
                return false;
            }
            dir = d.parent();
        }
        false
    }

    /// The JSDoc comment attached to a node that starts at `start` (its export keyword, first
    /// decorator or own start).
    fn doc(&self, m: &Module, starts: &[u32]) -> Doc {
        m.comments
            .iter()
            .rev()
            .find(|c| {
                c.is_block()
                    && starts.contains(&c.attached_to)
                    && m.src[c.span.start as usize..].starts_with("/**")
            })
            .map(|c| Doc::parse(text(m.src, c.content_span())))
            .unwrap_or_default()
    }

    /// `const M = (base) => class extends base {…}` and the inner-class form.
    fn mixin_var(&self, m: &mut Module, d: &VariableDeclarator, start: u32) -> Option<Value> {
        let BindingPattern::BindingIdentifier(id) = &d.id else {
            return None;
        };
        let Some(init) = &d.init else {
            return None;
        };
        let (params, class) = match init.get_inner_expression() {
            Expression::ArrowFunctionExpression(f) => (
                &f.params,
                match &f.body {
                    ArrowFunctionBody::FunctionBody(b) => mixin_class(&b.statements),
                    body => match body.as_expression().map(|e| e.get_inner_expression()) {
                        Some(Expression::ClassExpression(c)) => Some(c.as_ref()),
                        _ => None,
                    },
                },
            ),
            Expression::FunctionExpression(f) => (
                &f.params,
                f.body.as_ref().and_then(|b| mixin_class(&b.statements)),
            ),
            _ => return None,
        };
        self.mixin(m, &id.name, params, class, &[start, d.span.start])
    }

    fn mixin_fn(&self, m: &mut Module, f: &Function, start: u32) -> Option<Value> {
        let (Some(id), Some(body)) = (&f.id, &f.body) else {
            return None;
        };
        self.mixin(
            m,
            &id.name,
            &f.params,
            mixin_class(&body.statements),
            &[start, f.span.start],
        )
    }

    fn mixin(
        &self,
        m: &mut Module,
        name: &str,
        params: &FormalParameters,
        class: Option<&Class>,
        starts: &[u32],
    ) -> Option<Value> {
        let class = class?;
        let param = params
            .items
            .first()
            .and_then(|p| p.pattern.get_identifier_name());
        let param = param?;
        // A mixin's class extends its parameter, possibly through other mixins.
        let (_, base) = heritage(class.heritage.as_ref().map(|h| &h.expression));
        if base.as_deref() != Some(param.as_str()) {
            return None;
        }
        let mut decl = self.class(m, class, class.span.start, Some(name))?;
        let doc = self.doc(m, starts);
        let obj = decl.as_object_mut().into_iter().next();
        if let Some(obj) = obj {
            obj.insert("kind".into(), json!("mixin"));
            obj.remove("superclass");
            obj.remove("tagName");
            obj.remove("x-defined-by-decorator");
            obj.remove("x-holyfloe-models");
            if let Some(d) = doc.description {
                obj.entry("description").or_insert(json!(d));
            }
        }
        Some(decl)
    }

    /// A class declaration in manifest form, or `None` for an anonymous class. `as_name` names a
    /// mixin's inner class after the mixin.
    fn class(
        &self,
        m: &mut Module,
        class: &Class,
        stmt_start: u32,
        as_name: Option<&str>,
    ) -> Option<Value> {
        let name = as_name.or(class.id.as_ref().map(|id| id.name.as_str()))?;
        let mut starts = vec![stmt_start, class.span.start];
        starts.extend(class.decorators.iter().map(|d| d.span.start));
        let doc = self.doc(m, &starts);

        let mut decl = Map::new();
        decl.insert("kind".into(), json!("class"));
        decl.insert("name".into(), json!(name));
        decl.insert("customElement".into(), json!(true));
        put(&mut decl, "description", doc.description.clone());
        put(&mut decl, "summary", doc.tag("summary").map(str::to_owned));
        if let Some(d) = doc.deprecated() {
            decl.insert("deprecated".into(), deprecated(d));
        }
        let tag = class.decorators.iter().find_map(|d| {
            let Expression::CallExpression(call) = &d.expression else {
                return None;
            };
            call.callee
                .is_specific_id("customElement")
                .then(|| string_arg(call))?
        });
        if let Some(tag) = &tag {
            decl.insert("tagName".into(), json!(tag));
            // Marker for `definition`, removed before the manifest is returned.
            decl.insert("x-defined-by-decorator".into(), json!(true));
        } else if let Some(tag) = doc
            .tag("tag")
            .or(doc.tag("tagname"))
            .filter(|t| !t.is_empty())
        {
            decl.insert("tagName".into(), json!(tag));
        }

        let (mixins, base) = heritage(class.heritage.as_ref().map(|h| &h.expression));
        let mixins: Vec<Value> = mixins.iter().map(|n| self.reference(m, n)).collect();
        if !mixins.is_empty() {
            decl.insert("mixins".into(), json!(mixins));
        }
        if let Some(base) = base.filter(|_| as_name.is_none()) {
            let r = self.reference(m, &base);
            decl.insert("superclass".into(), r);
        }

        let mut members = Members::new(self, class);
        members.collect(m);
        decl.insert("members".into(), json!(members.into_values()));

        let events = self.events(m, class, &doc);
        if !events.is_empty() {
            decl.insert("events".into(), json!(events));
        }
        let named = |tags: &[&str], with_default: bool| -> Vec<Value> {
            doc.all(tags)
                .map(|t| {
                    let b = tag_body(t);
                    let mut o = Map::new();
                    o.insert("name".into(), json!(b.name));
                    put(&mut o, "description", b.description);
                    if with_default {
                        put(&mut o, "default", b.default);
                        put(&mut o, "syntax", b.ty);
                    }
                    Value::Object(o)
                })
                .collect()
        };
        for (key, tags, with_default) in [
            ("slots", &["slot"][..], false),
            ("cssParts", &["csspart"][..], false),
            ("cssProperties", &["cssprop", "cssproperty"][..], true),
            ("cssStates", &["cssstate"][..], false),
        ] {
            let items = named(tags, with_default);
            if !items.is_empty() {
                decl.insert(key.into(), json!(items));
            }
        }
        let models: Vec<Value> = doc
            .all(&["model"])
            .filter_map(|t| {
                let mut parts = t.split_whitespace();
                let (prop, event) = (parts.next()?, parts.next()?);
                Some(match parts.next() {
                    Some(value) => json!({ "prop": prop, "event": event, "value": value }),
                    None => json!({ "prop": prop, "event": event }),
                })
            })
            .collect();
        if !models.is_empty() {
            decl.insert("x-holyfloe-models".into(), json!(models));
        }
        Some(Value::Object(decl))
    }

    /// `@fires`/`@event` tags, then `dispatchEvent(new X('name'))` calls not already documented.
    /// A documented event takes its type from the dispatch when the tag has none.
    fn events(&self, m: &mut Module, class: &Class, doc: &Doc) -> Vec<Value> {
        let mut dispatched = Dispatches {
            src: m.src,
            found: Vec::new(),
        };
        dispatched.visit_class_body(&class.body);
        let mut events: Vec<(String, Option<String>, Option<String>)> = doc
            .all(&["fires", "event"])
            .map(tag_body)
            .filter(|b| !b.name.is_empty())
            .map(|b| (b.name, b.ty, b.description))
            .collect();
        for (name, ty) in dispatched.found {
            match events.iter_mut().find(|(n, _, _)| *n == name) {
                Some((_, t, _)) => {
                    t.get_or_insert(ty);
                }
                None => events.push((name, Some(ty), None)),
            }
        }
        events
            .into_iter()
            .map(|(name, ty, description)| {
                let mut o = Map::new();
                o.insert("name".into(), json!(name));
                if let Some(ty) = ty {
                    let references = self.type_references(m, &ty);
                    o.insert(
                        "type".into(),
                        json!({ "text": ty, "references": references }),
                    );
                }
                put(&mut o, "description", description);
                Value::Object(o)
            })
            .collect()
    }

    /// Imported or module-local type names that appear in `text`.
    fn type_references(&self, m: &mut Module, text: &str) -> Vec<Value> {
        let mut seen = BTreeSet::new();
        let words: Vec<&str> = text
            .split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '$'))
            .filter(|w| !w.is_empty() && seen.insert(*w))
            .collect();
        let mut refs = Vec::new();
        for w in words {
            if m.imports.contains_key(w) || m.local_types.contains(w) {
                refs.push(self.reference(m, w));
            }
        }
        refs
    }
}

/// The class a mixin function returns: `=> class extends base {}` or a class declared in the
/// body and returned.
fn mixin_class<'b, 'a>(body: &'b [Statement<'a>]) -> Option<&'b Class<'a>> {
    for stmt in body {
        match stmt {
            Statement::ExpressionStatement(e) => {
                if let Expression::ClassExpression(c) = e.expression.get_inner_expression() {
                    return Some(c);
                }
            }
            Statement::ReturnStatement(r) => {
                if let Some(Expression::ClassExpression(c)) =
                    r.argument.as_ref().map(|a| a.get_inner_expression())
                {
                    return Some(c);
                }
            }
            Statement::ClassDeclaration(c) => return Some(c),
            _ => {}
        }
    }
    None
}

/// `M1(M2(Base))` gives `([M1, M2], Some(Base))`.
fn heritage(expr: Option<&Expression>) -> (Vec<String>, Option<String>) {
    let mut mixins = Vec::new();
    let mut cur = expr.map(|e| e.get_inner_expression());
    while let Some(e) = cur {
        match e {
            Expression::CallExpression(call) => {
                let Expression::Identifier(id) = call.callee.get_inner_expression() else {
                    break;
                };
                mixins.push(id.name.to_string());
                cur = call
                    .arguments
                    .first()
                    .and_then(|a| a.as_expression())
                    .map(|e| e.get_inner_expression());
            }
            Expression::Identifier(id) => return (mixins, Some(id.name.to_string())),
            _ => break,
        }
    }
    (mixins, None)
}

/// Public instance fields, getters and methods of one class, in source order.
struct Members<'c, 'a, 'x> {
    ctx: &'c Ctx<'x>,
    class: &'c Class<'a>,
    /// name -> member; `order` keeps source order.
    by_name: HashMap<String, Map<String, Value>>,
    order: Vec<String>,
    /// Defaults assigned in the constructor (`this.x = …`), for `static properties`.
    ctor_defaults: HashMap<String, String>,
}

impl<'c, 'a, 'x> Members<'c, 'a, 'x> {
    fn new(ctx: &'c Ctx<'x>, class: &'c Class<'a>) -> Self {
        Self {
            ctx,
            class,
            by_name: HashMap::new(),
            order: Vec::new(),
            ctor_defaults: HashMap::new(),
        }
    }

    fn entry(&mut self, name: &str) -> &mut Map<String, Value> {
        if !self.by_name.contains_key(name) {
            self.order.push(name.to_owned());
        }
        self.by_name.entry(name.to_owned()).or_default()
    }

    fn into_values(mut self) -> Vec<Value> {
        self.order
            .iter()
            .filter_map(|n| self.by_name.remove(n))
            .filter(|o| !o.is_empty())
            .map(Value::Object)
            .collect()
    }

    fn collect(&mut self, m: &Module) {
        let src = m.src;
        let mut static_props: Option<&ObjectExpression> = None;
        for el in &self.class.body.body {
            match el {
                ClassElement::PropertyDefinition(p) => {
                    if let Some(name) = p.key.static_name()
                        && p.r#static
                        && name == "properties"
                        && let Some(Expression::ObjectExpression(o)) =
                            p.value.as_ref().map(|v| v.get_inner_expression())
                    {
                        static_props = Some(o);
                        continue;
                    }
                    self.field(
                        m,
                        &p.key,
                        &p.decorators,
                        p.span,
                        p.r#static,
                        p.accessibility,
                        p.readonly,
                        p.optional,
                        p.type_annotation.as_deref(),
                        p.value.as_ref(),
                    );
                }
                ClassElement::AccessorProperty(p) => self.field(
                    m,
                    &p.key,
                    &p.decorators,
                    p.span,
                    p.r#static,
                    p.accessibility,
                    false,
                    false,
                    p.type_annotation.as_deref(),
                    p.value.as_ref(),
                ),
                ClassElement::MethodDefinition(md) => {
                    if md.kind == MethodDefinitionKind::Constructor {
                        self.constructor(src, &md.value);
                        continue;
                    }
                    if md.r#static
                        && md.kind == MethodDefinitionKind::Get
                        && md.key.static_name().as_deref() == Some("properties")
                    {
                        static_props = returned_object(&md.value);
                        continue;
                    }
                    self.method(m, md);
                }
                _ => {}
            }
        }
        if let Some(o) = static_props {
            self.static_properties(o);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn field(
        &mut self,
        m: &Module,
        key: &PropertyKey,
        decorators: &[Decorator],
        span: Span,
        is_static: bool,
        access: Option<TSAccessibility>,
        readonly: bool,
        optional: bool,
        annotation: Option<&TSTypeAnnotation>,
        value: Option<&Expression>,
    ) {
        let Some(name) = key.static_name() else {
            return;
        };
        if key.is_private_identifier() || DENIED.contains(&name.as_ref()) || hidden(access) {
            return;
        }
        // Statics are dropped by the IR, except `formAssociated` (D-065).
        if is_static && name != "formAssociated" {
            return;
        }
        let mut starts = vec![span.start];
        starts.extend(decorators.iter().map(|d| d.span.start));
        let doc = self.ctx.doc(m, &starts);
        if doc.has("internal")
            || decorators
                .iter()
                .any(|d| INTERNAL_DECORATORS.contains(&decorator_name(d)))
        {
            return;
        }
        let options = decorators
            .iter()
            .find(|d| decorator_name(d) == "property")
            .map(|d| match &d.expression {
                Expression::CallExpression(c) => {
                    c.arguments.first().and_then(|a| a.as_expression())
                }
                _ => None,
            });
        let src = m.src;
        let ty = annotation
            .map(|a| {
                let t = text(src, a.type_annotation.span()).to_owned();
                if optional {
                    format!("{t} | undefined")
                } else {
                    t
                }
            })
            .or_else(|| {
                doc.tag("type")
                    .map(|t| tag_body(t).ty.unwrap_or_default())
                    .filter(|t| !t.is_empty())
            })
            .or_else(|| value.and_then(literal_type))
            .or_else(|| options.flatten().and_then(|o| option_type(o)));
        let field = self.entry(&name);
        field.insert("kind".into(), json!("field"));
        field.insert("name".into(), json!(name));
        put(field, "type", ty.map(|t| json!({ "text": t })));
        put(
            field,
            "default",
            value.map(|v| text(src, v.span()).to_owned()),
        );
        put(field, "description", doc.description.clone());
        if let Some(d) = doc.deprecated() {
            field.insert("deprecated".into(), deprecated(d));
        }
        if is_static {
            field.insert("static".into(), json!(true));
        }
        if readonly {
            field.insert("readonly".into(), json!(true));
        }
        if let Some(options) = options {
            property_options(field, &name, options);
        }
    }

    /// Getters and setters become one field (read-only without a setter); methods stay methods.
    fn method(&mut self, m: &Module, md: &MethodDefinition) {
        let Some(name) = md.key.static_name() else {
            return;
        };
        if md.key.is_private_identifier()
            || md.r#static
            || hidden(md.accessibility)
            || DENIED.contains(&name.as_ref())
        {
            return;
        }
        let mut starts = vec![md.span.start];
        starts.extend(md.decorators.iter().map(|d| d.span.start));
        let doc = self.ctx.doc(m, &starts);
        if doc.has("internal") {
            return;
        }
        let src = m.src;
        match md.kind {
            MethodDefinitionKind::Get | MethodDefinitionKind::Set => {
                let ty = match md.kind {
                    MethodDefinitionKind::Get => md
                        .value
                        .return_type
                        .as_ref()
                        .map(|t| text(src, t.type_annotation.span()).to_owned()),
                    _ => md
                        .value
                        .params
                        .items
                        .first()
                        .and_then(|p| p.type_annotation.as_ref())
                        .map(|t| text(src, t.type_annotation.span()).to_owned()),
                };
                let is_get = md.kind == MethodDefinitionKind::Get;
                let options = md
                    .decorators
                    .iter()
                    .find(|d| decorator_name(d) == "property")
                    .map(|d| match &d.expression {
                        Expression::CallExpression(c) => {
                            c.arguments.first().and_then(|a| a.as_expression())
                        }
                        _ => None,
                    });
                let field = self.entry(&name);
                let had_setter = field.get("x-setter").is_some();
                field.insert("kind".into(), json!("field"));
                field.insert("name".into(), json!(name));
                if is_get || !field.contains_key("type") {
                    put(field, "type", ty.map(|t| json!({ "text": t })));
                }
                if field.get("description").is_none() {
                    put(field, "description", doc.description.clone());
                }
                if let Some(d) = doc.deprecated() {
                    field.insert("deprecated".into(), deprecated(d));
                }
                if is_get && !had_setter {
                    field.insert("readonly".into(), json!(true));
                } else {
                    field.remove("readonly");
                    field.insert("x-setter".into(), json!(true));
                }
                if let Some(options) = options {
                    property_options(field, &name, options);
                }
            }
            MethodDefinitionKind::Method => {
                let method = self.entry(&name);
                method.insert("kind".into(), json!("method"));
                method.insert("name".into(), json!(name));
                put(method, "description", doc.description.clone());
                if let Some(d) = doc.deprecated() {
                    method.insert("deprecated".into(), deprecated(d));
                }
            }
            MethodDefinitionKind::Constructor => {}
        }
    }

    /// `this.x = <default>` in the constructor.
    fn constructor(&mut self, src: &str, f: &Function) {
        let Some(body) = &f.body else {
            return;
        };
        for stmt in &body.statements {
            let Statement::ExpressionStatement(e) = stmt else {
                continue;
            };
            let Expression::AssignmentExpression(a) = &e.expression else {
                continue;
            };
            if let AssignmentTarget::StaticMemberExpression(target) = &a.left
                && matches!(target.object, Expression::ThisExpression(_))
            {
                self.ctor_defaults.insert(
                    target.property.name.to_string(),
                    text(src, a.right.span()).to_owned(),
                );
            }
        }
    }

    /// `static properties = { name: { type, attribute, reflect, state } }`.
    fn static_properties(&mut self, props: &ObjectExpression) {
        for p in &props.properties {
            let ObjectPropertyKind::ObjectProperty(p) = p else {
                continue;
            };
            let Some(name) = p.key.static_name() else {
                continue;
            };
            let options = p.value.get_inner_expression();
            if let Expression::ObjectExpression(o) = options
                && option(o, "state")
                    .is_some_and(|v| matches!(v, Expression::BooleanLiteral(b) if b.value))
            {
                continue;
            }
            let default = self.ctor_defaults.get(name.as_ref()).cloned();
            let field = self.entry(&name);
            field.insert("kind".into(), json!("field"));
            field.insert("name".into(), json!(name));
            if !field.contains_key("default") {
                put(field, "default", default.clone());
            }
            if !field.contains_key("type") {
                let ty = option_type(options).or_else(|| {
                    default.as_deref().and_then(|d| {
                        let alloc = Allocator::default();
                        Parser::new(&alloc, d, SourceType::mjs())
                            .parse_expression()
                            .ok()
                            .and_then(|e| literal_type(&e))
                    })
                });
                put(field, "type", ty.map(|t| json!({ "text": t })));
            }
            property_options(field, &name, Some(options));
        }
    }
}

/// Lit's `attribute`, `reflect` and `type` options. Without `attribute`, Lit observes the
/// lowercased property name (D-091).
fn property_options(field: &mut Map<String, Value>, name: &str, options: Option<&Expression>) {
    let o = match options.map(|o| o.get_inner_expression()) {
        Some(Expression::ObjectExpression(o)) => Some(o.as_ref()),
        _ => None,
    };
    match o.and_then(|o| option(o, "attribute")) {
        Some(Expression::BooleanLiteral(b)) if !b.value => {
            field.remove("attribute");
        }
        Some(Expression::StringLiteral(s)) => {
            field.insert("attribute".into(), json!(s.value.as_str()));
        }
        _ => {
            field.insert("attribute".into(), json!(name.to_lowercase()));
        }
    }
    if o.and_then(|o| option(o, "reflect"))
        .is_some_and(|v| matches!(v, Expression::BooleanLiteral(b) if b.value))
    {
        field.insert("reflects".into(), json!(true));
    }
}

fn option<'o, 'a>(o: &'o ObjectExpression<'a>, key: &str) -> Option<&'o Expression<'a>> {
    o.properties.iter().find_map(|p| match p {
        ObjectPropertyKind::ObjectProperty(p) if p.key.static_name().as_deref() == Some(key) => {
            Some(p.value.get_inner_expression())
        }
        _ => None,
    })
}

/// `{ type: Boolean | Number | String }` as a type.
fn option_type(options: &Expression) -> Option<String> {
    let Expression::ObjectExpression(o) = options.get_inner_expression() else {
        return None;
    };
    let Expression::Identifier(id) = option(o, "type")? else {
        return None;
    };
    match id.name.as_str() {
        "Boolean" => Some("boolean".into()),
        "Number" => Some("number".into()),
        "String" => Some("string".into()),
        _ => None,
    }
}

/// The type a checker would infer for a literal initializer.
fn literal_type(e: &Expression) -> Option<String> {
    Some(
        match e.get_inner_expression() {
            Expression::StringLiteral(_) | Expression::TemplateLiteral(_) => "string",
            Expression::NumericLiteral(_) => "number",
            Expression::BooleanLiteral(_) => "boolean",
            Expression::BigIntLiteral(_) => "bigint",
            _ => return None,
        }
        .to_owned(),
    )
}

fn returned_object<'b, 'a>(f: &'b Function<'a>) -> Option<&'b ObjectExpression<'a>> {
    f.body.as_ref()?.statements.iter().find_map(|s| match s {
        Statement::ReturnStatement(r) => match r.argument.as_ref()?.get_inner_expression() {
            Expression::ObjectExpression(o) => Some(o.as_ref()),
            _ => None,
        },
        _ => None,
    })
}

fn decorator_name<'a>(d: &'a Decorator) -> &'a str {
    match &d.expression {
        Expression::CallExpression(c) => match &c.callee {
            Expression::Identifier(id) => id.name.as_str(),
            _ => "",
        },
        Expression::Identifier(id) => id.name.as_str(),
        _ => "",
    }
}

fn hidden(access: Option<TSAccessibility>) -> bool {
    matches!(
        access,
        Some(TSAccessibility::Private | TSAccessibility::Protected)
    )
}

fn string_arg(call: &CallExpression) -> Option<String> {
    match call
        .arguments
        .first()?
        .as_expression()?
        .get_inner_expression()
    {
        Expression::StringLiteral(s) => Some(s.value.to_string()),
        _ => None,
    }
}

fn deprecated(reason: &str) -> Value {
    if reason.is_empty() {
        json!(true)
    } else {
        json!(reason)
    }
}

fn put<T: Into<Value>>(o: &mut Map<String, Value>, key: &str, v: Option<T>) {
    if let Some(v) = v {
        o.insert(key.into(), v.into());
    }
}

fn text(src: &str, span: Span) -> &str {
    &src[span.start as usize..span.end as usize]
}

/// `dispatchEvent(new X<T>('name', …))`: the event name and the type text `X<T>`.
struct Dispatches<'s> {
    src: &'s str,
    found: Vec<(String, String)>,
}

impl<'a> Visit<'a> for Dispatches<'_> {
    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        if let Some(member) = call.callee.as_member_expression()
            && member.static_property_name() == Some("dispatchEvent")
            && let Some(Expression::NewExpression(new)) =
                call.arguments.first().and_then(|a| a.as_expression())
            && let Some(Expression::StringLiteral(name)) =
                new.arguments.first().and_then(|a| a.as_expression())
            && let Expression::Identifier(_) = &new.callee
        {
            let end = new
                .type_arguments
                .as_ref()
                .map_or(new.callee.span().end, |t| t.span.end);
            let ty = self.src[new.callee.span().start as usize..end as usize].to_owned();
            if !self.found.iter().any(|(n, _)| n == name.value.as_str()) {
                self.found.push((name.value.to_string(), ty));
            }
        }
        walk::walk_call_expression(self, call);
    }
}

/// `customElements.define('tag', Class)`, also through `window.` or `globalThis.`.
#[derive(Default)]
struct Defines {
    found: Vec<(String, String)>,
}

impl<'a> Visit<'a> for Defines {
    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        if let Some(member) = call.callee.as_member_expression()
            && member.static_property_name() == Some("define")
            && is_custom_elements(member.object())
            && let Some(tag) = string_arg(call)
            && let Some(Expression::Identifier(class)) = call
                .arguments
                .get(1)
                .and_then(|a| a.as_expression())
                .map(|e| e.get_inner_expression())
        {
            self.found.push((tag, class.name.to_string()));
        }
        walk::walk_call_expression(self, call);
    }
}

fn is_custom_elements(e: &Expression) -> bool {
    match e.get_inner_expression() {
        Expression::Identifier(id) => id.name == "customElements",
        e => e
            .as_member_expression()
            .is_some_and(|m| m.static_property_name() == Some("customElements")),
    }
}

/// Removes the internal markers this crate adds while building declarations.
fn finish(mut manifest: Value) -> Value {
    if let Some(modules) = manifest["modules"].as_array_mut() {
        for module in modules {
            if let Some(decls) = module["declarations"].as_array_mut() {
                for decl in decls {
                    if let Some(o) = decl.as_object_mut() {
                        o.remove("x-defined-by-decorator");
                        if let Some(members) = o.get_mut("members").and_then(Value::as_array_mut) {
                            for m in members {
                                if let Some(m) = m.as_object_mut() {
                                    m.remove("x-setter");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    manifest
}
