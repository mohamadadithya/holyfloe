import { LitElement, css, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';

/**
 * A content card with header and footer slots.
 *
 * @slot - Card body.
 * @slot header - Content for the card header.
 * @slot footer - Content for the card footer.
 * @csspart base - The card container.
 * @csspart header - The header wrapper.
 * @csspart footer - The footer wrapper.
 * @cssprop --fx-card-padding - Inner padding. Defaults to 16px.
 * @cssprop --fx-card-radius - Corner radius. Defaults to 8px.
 */
@customElement('fx-card')
export default class FxCard extends LitElement {
  static override styles = css`
    [part='base'] {
      padding: var(--fx-card-padding, 16px);
      border-radius: var(--fx-card-radius, 8px);
    }
  `;

  /** Adds a drop shadow. */
  @property({ type: Boolean, reflect: true })
  elevated = false;

  override render() {
    return html`<div part="base">
      <header part="header"><slot name="header"></slot></header>
      <slot></slot>
      <footer part="footer"><slot name="footer"></slot></footer>
    </div>`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'fx-card': FxCard;
  }
}
