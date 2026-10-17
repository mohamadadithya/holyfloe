// Must compile: correct props, events, ref and children.
// shortcut: imports expected/ until M1 emits fixtures/out; switch the path then.
import { useRef } from 'react';
import type { FxBadge as FxBadgeElement } from '@holyfloe/fixtures/fx-badge/fx-badge.js';
import { FxBadge } from '../../expected/react/fx-badge/index.js';

export function Consumer() {
  const ref = useRef<FxBadgeElement>(null);
  return (
    <FxBadge ref={ref} count={3} tone="success" label="3 unread">
      new
    </FxBadge>
  );
}
