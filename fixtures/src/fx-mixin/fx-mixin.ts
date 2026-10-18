import { LitElement, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { FocusableMixin } from '@holyfloe/fixtures-dep/focusable-mixin.js';

/**
 * Gets `focused`, `ringColor` and the `focus-change` event from a mixin in another package.
 *
 * @slot - The label.
 */
@customElement('fx-mixin')
export class FxMixin extends FocusableMixin(LitElement) {
  /** Shown before the slot. */
  @property()
  prefix = '';

  override render() {
    return html`<button part="base">${this.prefix}<slot></slot></button>`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'fx-mixin': FxMixin;
  }
}
