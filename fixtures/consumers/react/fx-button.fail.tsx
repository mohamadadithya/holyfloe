// Each line below must fail to type-check, exactly once.
import { FxButton } from '../../out/react/fx-button/index.js';

// @ts-expect-error variant is a closed union
export const BadVariant = <FxButton variant="nope" />;

// @ts-expect-error disabled is boolean, not string
export const BadDisabled = <FxButton disabled="yes" />;

// @ts-expect-error fx-press detail is undeclared, so it is unknown and not usable as string
export const BadDetail = <FxButton onFxPress={(e) => e.detail.toUpperCase()} />;

// @ts-expect-error unknown props are rejected
export const BadProp = <FxButton notAProp />;
