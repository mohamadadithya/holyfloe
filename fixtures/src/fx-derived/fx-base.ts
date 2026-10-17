import { LitElement } from 'lit';
import { property } from 'lit/decorators.js';

/**
 * Base class with shared sizing and loading state. Not registered as an element.
 *
 * @fires fx-ready - Fired after first render.
 */
export class FxBase extends LitElement {
  /** Size variant shared by subclasses. */
  @property({ reflect: true })
  size: 'sm' | 'md' | 'lg' = 'md';

  /** Shows a loading state. */
  @property({ type: Boolean })
  loading = false;

  override firstUpdated() {
    this.dispatchEvent(new CustomEvent('fx-ready'));
  }
}
