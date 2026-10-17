import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

/**
 * Dispatches events whose names use every common casing.
 *
 * @fires {CustomEvent<{ value: string }>} my-change - Kebab-case name.
 * @fires {CustomEvent<{ value: number }>} myChange - camelCase name.
 * @fires {CustomEvent<{ checked: boolean }>} MyChange - PascalCase name.
 * @fires {CustomEvent<{ count: number }>} MYEVENT - Uppercase name.
 */
@customElement('fx-events')
export class FxEvents extends LitElement {
  override render() {
    return html`<button @click=${this.#emitAll}>emit</button>`;
  }

  #emitAll() {
    this.dispatchEvent(new CustomEvent('my-change', { detail: { value: 'a' } }));
    this.dispatchEvent(new CustomEvent('myChange', { detail: { value: 1 } }));
    this.dispatchEvent(new CustomEvent('MyChange', { detail: { checked: true } }));
    this.dispatchEvent(new CustomEvent('MYEVENT', { detail: { count: 4 } }));
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'fx-events': FxEvents;
  }
}
