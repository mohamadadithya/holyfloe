// Each marked line must fail to type-check.
import { FxDialog } from '../../out/react/fx-dialog/index.js';

// @ts-expect-error open is a boolean
export const BadOpen = <FxDialog open="yes" />;

// @ts-expect-error open-change detail.open is a boolean
export const BadDetail = <FxDialog onOpenChange={(e) => e.detail.open.toUpperCase()} />;
