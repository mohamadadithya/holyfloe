import { LitElement, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';

/**
 * Old alert element.
 *
 * @deprecated Use fx-card instead.
 */
@customElement('fx-deprecated')
export class FxDeprecated extends LitElement {
  /** Alert tone. */
  @property({ reflect: true })
  tone: 'info' | 'error' = 'info';

  /**
   * Old name for `tone`.
   * @deprecated Use `tone`.
   */
  @property()
  kind?: string;

  /**
   * Debug id used by tests. Must never appear in generated output.
   * @internal
   */
  @property({ attribute: 'debug-id' })
  debugId = '';

  override render() {
    return html`<div role="alert"><slot></slot></div>`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'fx-deprecated': FxDeprecated;
  }
}
