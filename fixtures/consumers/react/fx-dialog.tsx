// Must compile: open + onOpenChange, a ref with the element methods.
import { useRef, useState } from 'react';
import { FxDialog } from '../../out/react/fx-dialog/index.js';

export function Consumer() {
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLElementTagNameMap['fx-dialog']>(null);
  return (
    <FxDialog ref={ref} open={open} onOpenChange={(e) => setOpen(e.detail.open)} onClick={() => ref.current?.show()}>
      Body
      <button slot="footer">OK</button>
    </FxDialog>
  );
}
