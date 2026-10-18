// Each line below must fail to type-check, exactly once.
import { useRef } from 'react';
import { FxButton } from '../../out/react-ref-prop/fx-button/index.js';

export function Consumer() {
  const inputRef = useRef<HTMLInputElement>(null);
  // @ts-expect-error the ref is typed to the element class, not any element
  return <FxButton ref={inputRef} />;
}

// @ts-expect-error variant is a closed union
export const BadVariant = <FxButton variant="nope" />;
