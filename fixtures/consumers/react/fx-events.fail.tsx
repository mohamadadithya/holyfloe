// Each marked line must fail to type-check.
// shortcut: imports expected/ until M1 emits fixtures/out; switch the path then.
import { FxEvents } from '../../expected/react/fx-events/index.js';

// @ts-expect-error my-change detail.value is a string
export const BadKebab = <FxEvents onMyChange={(e) => { const n: number = e.detail.value; void n; }} />;

// @ts-expect-error MyChange detail has checked, not value
export const BadPascal = <FxEvents onMyChangePascal={(e) => e.detail.value} />;

// @ts-expect-error raw DOM names are not wrapper props
export const RawName = <FxEvents onmyChange={() => {}} />;
