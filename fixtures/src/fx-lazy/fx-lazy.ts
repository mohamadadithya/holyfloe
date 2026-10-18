import { LitElement, html } from 'lit';
import { property } from 'lit/decorators.js';

/**
 * Not defined on import: apps call `defineFxLazy()` after their first render, so wrappers
 * meet an element that is not upgraded yet.
 *
 * @tag fx-lazy
 * @csspart label - The label.
 */
export class FxLazy extends LitElement {
  /** Rendered as a list. */
  @property({ attribute: false })
  items: string[] = [];

  /** Text above the list. */
  @property()
  label = '';

  /** Highlights the element. */
  @property({ type: Boolean, reflect: true })
  active = false;

  override render() {
    return html`<span part="label">${this.label}</span>
      <ul>
        ${this.items.map((item) => html`<li>${item}</li>`)}
      </ul>`;
  }
}

/** Registers `fx-lazy`. Safe to call twice. */
export function defineFxLazy() {
  if (!customElements.get('fx-lazy')) customElements.define('fx-lazy', FxLazy);
}

declare global {
  interface HTMLElementTagNameMap {
    'fx-lazy': FxLazy;
  }
}
