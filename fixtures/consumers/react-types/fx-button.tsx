// Must compile with the React 19 JSX augmentation from out/react-types (included via tsconfig).
import type { FxButton as FxButtonElement } from '@holyfloe/fixtures/fx-button/fx-button.js';

export const Ok = (
  <fx-button variant="danger" disabled onfx-press={(e) => { const el: FxButtonElement = e.currentTarget; void el; }}>
    Save
  </fx-button>
);
