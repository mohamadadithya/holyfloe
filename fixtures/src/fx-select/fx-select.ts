import { LitElement, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';

/**
 * Picks one option. The new value travels in `detail.value`; the binding comes from config.
 *
 * @fires {CustomEvent<{ value: string }>} fx-change - Fired when the user picks an option.
 */
@customElement('fx-select')
export class FxSelect extends LitElement {
  /** The picked option. */
  @property({ reflect: true })
  value = '';

  /** Options to pick from. */
  @property({ attribute: false })
  options: string[] = [];

  override render() {
    return html`${this.options.map(
      (option) =>
        html`<button part="option" aria-pressed=${option === this.value} @click=${() => this.#pick(option)}>
          ${option}
        </button>`,
    )}`;
  }

  #pick(value: string) {
    this.value = value;
    this.dispatchEvent(new CustomEvent('fx-change', { detail: { value } }));
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'fx-select': FxSelect;
  }
}
