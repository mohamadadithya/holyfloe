import { html } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { FxBase } from './fx-base.js';

/**
 * Element that inherits `size`, `loading` and `fx-ready` from FxBase.
 *
 * @slot - Content.
 */
@customElement('fx-derived')
export class FxDerived extends FxBase {
  /** Own property, not inherited. */
  @property()
  label = '';

  override render() {
    return html`<span>${this.label}</span><slot></slot>`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'fx-derived': FxDerived;
  }
}
