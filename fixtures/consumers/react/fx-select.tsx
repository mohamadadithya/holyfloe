// Must compile: value + options, the new value in detail.value.
import { useState } from 'react';
import { FxSelect } from '../../out/react/fx-select/index.js';

export function Consumer() {
  const [value, setValue] = useState('a');
  return <FxSelect value={value} options={['a', 'b']} onFxChange={(e) => setValue(e.detail.value)} />;
}
