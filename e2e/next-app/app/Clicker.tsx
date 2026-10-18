'use client';

import { useState } from 'react';
import { FxButton } from '../generated/fx-button/index.js';

/** Event handlers cannot cross from a server component, so the click test lives here. */
export function Clicker() {
  const [presses, setPresses] = useState(0);
  return (
    <>
      <FxButton id="client-btn" onFxPress={() => setPresses((n) => n + 1)}>Press</FxButton>
      <output id="presses">{presses}</output>
    </>
  );
}
