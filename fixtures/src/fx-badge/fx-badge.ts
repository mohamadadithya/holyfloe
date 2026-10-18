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

  /** Accessible label. Property only (`attribute: false`): never rendered as an attribute. */
  @property({ attribute: false })
  label = '';

  /** Id of the element this badge describes. `for` is a reserved word, so wrappers bind it to another local name. */
  @property({ reflect: true })
  for = '';

  /** True when the count is shown as 99+. Read-only. */
  get overflow(): boolean {
    return this.count > 99;
  }

  override render() {
    return html`<span aria-label=${this.label}>${this.overflow ? '99+' : this.count}</span><slot></slot>`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'fx-badge': FxBadge;
  }
}
