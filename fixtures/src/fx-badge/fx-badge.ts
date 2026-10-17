import { LitElement, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';

/**
 * A small count badge.
 *
 * @slot - Optional label shown next to the count.
 */
@customElement('fx-badge')
export class FxBadge extends LitElement {
  /** Number shown in the badge. The attribute name differs from the property name. */
  @property({ type: Number, attribute: 'badge-count', reflect: true })
  count = 0;

  /** Colour tone, reflected so it can be styled from outside. */
  @property({ reflect: true })
  tone: 'neutral' | 'success' | 'warning' = 'neutral';

  override render() {
    return html`<span>${this.count}</span><slot></slot>`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'fx-badge': FxBadge;
  }
}
