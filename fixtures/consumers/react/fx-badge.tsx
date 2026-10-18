// Must compile: correct props, events, ref and children.
import { useRef } from 'react';
import type { FxBadge as FxBadgeElement } from '@holyfloe/fixtures/fx-badge/fx-badge.js';
import { FxBadge } from '../../out/react/fx-badge/index.js';

export function Consumer() {
  const ref = useRef<FxBadgeElement>(null);
  return (
    <FxBadge ref={ref} count={3} tone="success" label="3 unread" for="inbox">
      new
    </FxBadge>
  );
}
