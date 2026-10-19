// Must compile: complex items, primitive label, boolean active.
import { FxLazy } from '../../out/react/fx-lazy/index.js';

export const Ok = <FxLazy items={['a', 'b']} label="Fruit" active />;
