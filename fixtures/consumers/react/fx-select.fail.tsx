// Each marked line must fail to type-check.
import { FxSelect } from '../../out/react/fx-select/index.js';

// @ts-expect-error options are strings
export const BadOptions = <FxSelect options={[1, 2]} />;

// @ts-expect-error fx-change detail.value is a string
export const BadDetail = <FxSelect onFxChange={(e) => e.detail.value.toFixed()} />;
