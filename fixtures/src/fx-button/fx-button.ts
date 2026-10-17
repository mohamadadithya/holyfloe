import { LitElement, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';

/**
 * A clickable button.
 *
 * @summary Basic action button.
 * @slot - The button label.
 * @fires fx-press - Fired when the button is activated. Has no detail.
 */
@customElement('fx-button')
export class FxButton extends LitElement {
  /** Visual style of the button. */
  @property({ reflect: true })
  variant: 'primary' | 'secondary' | 'danger' = 'primary';

  /** Prevents interaction when set. */
  @property({ type: Boolean, reflect: true })
  disabled = false;

  override render() {
    return html`<button ?disabled=${this.disabled} @click=${this.#onClick}><slot></slot></button>`;
  }

  #onClick() {
    this.dispatchEvent(new CustomEvent('fx-press', { bubbles: true, composed: true }));
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'fx-button': FxButton;
  }
}
