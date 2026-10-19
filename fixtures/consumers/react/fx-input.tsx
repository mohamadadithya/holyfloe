// Must compile: value + change, the binding React users wire by hand (D-067).
import { useState } from 'react';
import { FxInput } from '../../out/react/fx-input/index.js';

export function Consumer() {
  const [value, setValue] = useState('');
  return <FxInput value={value} name="q" disabled={false} onChange={(e) => setValue(e.target.value)} />;
}
