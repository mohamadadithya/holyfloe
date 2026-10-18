import type { LitElement } from 'lit';
import { property } from 'lit/decorators.js';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type Constructor<T> = new (...args: any[]) => T;

export declare class FocusableInterface {
  focused: boolean;
  ringColor: string;
  toggleFocus(): void;
}

/** Tracks focus. Lives in a second package so fx-mixin exercises cross-package flatten. */
export const FocusableMixin = <T extends Constructor<LitElement>>(superClass: T) => {
  // The analyzer records `focus-change` from the dispatch but ignores `@fires` on mixins, so
  // its detail stays untyped (`unknown` in wrappers).
  class Focusable extends superClass {
    /** Whether the element has focus. */
    @property({ type: Boolean, reflect: true })
    focused = false;

    /** Focus ring color. */
    @property({ attribute: 'ring-color' })
    ringColor = 'blue';

    /** Flips `focused` and fires `focus-change`. */
    toggleFocus() {
      this.focused = !this.focused;
      this.dispatchEvent(new CustomEvent('focus-change', { detail: { focused: this.focused } }));
    }
  }
  return Focusable as Constructor<FocusableInterface> & T;
};
