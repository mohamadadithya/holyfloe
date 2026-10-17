import { LitElement, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { ListItem, ListOptions } from './types.js';

/**
 * Renders a list of items. All props are complex and set as properties.
 */
@customElement('fx-list')
export class FxList extends LitElement {
  /** Items to render. */
  @property({ attribute: false })
  items: ListItem[] = [];

  /** Display options. */
  @property({ attribute: false })
  options: ListOptions = { dense: false };

  /** Optional custom label renderer. */
  @property({ attribute: false })
  renderLabel?: (item: ListItem) => string;

  override render() {
    const items = this.items.slice(0, this.options.maxItems);
    return html`<ul>${items.map((i) => html`<li>${this.renderLabel ? this.renderLabel(i) : i.label}</li>`)}</ul>`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'fx-list': FxList;
  }
}
