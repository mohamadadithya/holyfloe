import { LitElement, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';

/**
 * A dialog whose `open` state two-way binds through an explicit `@model` tag.
 *
 * @model open open-change detail.open
 * @fires {CustomEvent<{ open: boolean }>} open-change - Fired when the dialog opens or closes.
 * @slot - Dialog content.
 * @slot footer - Dialog actions.
 * @csspart panel - The dialog panel.
 */
@customElement('fx-dialog')
export class FxDialog extends LitElement {
  /** Whether the dialog is shown. */
  @property({ type: Boolean, reflect: true })
  open = false;

  /** Opens the dialog. */
  show() {
    this.#setOpen(true);
  }

  /** Closes the dialog. */
  close() {
    this.#setOpen(false);
  }

  #setOpen(open: boolean) {
    if (this.open === open) return;
    this.open = open;
    this.dispatchEvent(new CustomEvent('open-change', { detail: { open } }));
  }

  override render() {
    return html`<div part="panel" ?hidden=${!this.open}>
      <slot></slot>
      <slot name="footer"></slot>
      <button part="close" @click=${this.close}>close</button>
    </div>`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'fx-dialog': FxDialog;
  }
}
