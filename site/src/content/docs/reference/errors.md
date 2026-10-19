---
title: Error codes
description: What each holyfloe error code means and how to fix it.
---

The CLI prints errors as `error[E0xx]: message (module)` with a `hint:` line and exits with status 1. In the [Node API](/holyfloe/guides/node-api/#errors) the code is `err.code`. File and manifest parse errors have no code (`error: ...` in the CLI, `GenericFailure` in the Node API).

| Code | Meaning | Fix |
|---|---|---|
| E002 | A module (component, definition or event detail type) matches no `package.json` `exports` entry, and no exported module re-exports the class. | Add the module to `exports`, or map source paths with [`moduleRewrite`](/holyfloe/reference/configuration/#modulerewrite). |
| E003 | Inheritance cycle. | Fix the superclass chain. |
| E004 | A tag is defined more than once. | Remove the duplicate definition or `exclude` one. |
| E005 | A prop is declared more than once. | Remove the duplicate declaration. |
| E006 | Several events map to the same prop name (`my-change` and `myChange`). | Rename one with [`overrides.<tag>.events`](/holyfloe/reference/configuration/#overrides). |
| E007 | An event has an empty name. | Name the event. |
| E008 | A tag is not a valid custom element name, or a component or event name is not an identifier. | Check the tag and event names. |
| E009 | An override names a tag, event or excluded prop that does not exist. | Fix or remove the override. |
| E010 | Unsupported config key (`tagFormatter`, `componentNameFormatter`). | Remove the key. |
| E012 | An event detail type name is imported from two modules, or clashes with a name the generated file uses. | Rename the type. |
| E013 | A prop has the same name as one the wrapper generates (an event's `on<Name>`, Svelte `element`, Vue `modelValue`). | Rename the event with `overrides.<tag>.events`, or drop the prop with `excludeProps`. |
| E014 | Two tags give the same component name, ignoring case (`fx-button` and `fx--button`). | Rename or `exclude` one. |
| E015 | An explicit binding (`@model` or config) names an unknown or read-only prop or an unknown event, has a malformed path, or repeats a prop. | Fix the binding. |
