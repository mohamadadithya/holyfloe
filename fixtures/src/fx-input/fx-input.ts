import { LitElement, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';

/**
 * Text field that takes part in forms. `value` follows every keystroke.
 *
 * @summary Form-associated text input.
 * @fires change - Fired when the user commits a value. Has no detail; read `value`.
 * @csspart input - The native input.
 */
@customElement('fx-input')
export class FxInput extends LitElement {
  static formAssociated = true;

  #internals = this.attachInternals();

  /** The current text. */
  @property()
  value = '';

  /** Field name used when the form submits. */
  @property({ reflect: true })
  name = '';

  /** Disables the field. */
  @property({ type: Boolean, reflect: true })
  disabled = false;

  override updated() {
    this.#internals.setFormValue(this.value);
  }

  override render() {
    return html`<input
      part="input"
      .value=${this.value}
      ?disabled=${this.disabled}
      @input=${this.#onInput}
      @change=${this.#onChange}
    />`;
  }

  #onInput(e: Event) {
    this.value = (e.target as HTMLInputElement).value;
  }

  #onChange() {
    // The native `change` is not composed, so the host re-dispatches it. Not bubbling (the
    // `new Event` default) on purpose: Svelte delegates `onchange` to the root, so wrappers
    // must still see it (D-075).
    this.dispatchEvent(new Event('change'));
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'fx-input': FxInput;
  }
}
