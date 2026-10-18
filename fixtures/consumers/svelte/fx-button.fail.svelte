<!-- Each marked line must fail to type-check. svelte-check has no expect-error comment for
     markup, so the cases are props objects checked against the component's props type. -->
<script lang="ts">
  import type { ComponentProps } from 'svelte';
  import type { SvelteHTMLElements } from 'svelte/elements';
  import { FxButton } from '../../out/svelte/fx-button/index.js';

  type Props = ComponentProps<typeof FxButton>;
  // @ts-expect-error variant is a union of strings
  export const badVariant: Props = { variant: 'huge' };
  // @ts-expect-error disabled is a boolean
  export const badDisabled: Props = { disabled: 'yes' };
  // @ts-expect-error the event target is the element; variant is a string
  export const badHandler: Props = { onFxPress: (e) => e.currentTarget.variant.toFixed() };
  // @ts-expect-error raw tag: variant is a union of strings
  export const badRaw: SvelteHTMLElements['fx-button'] = { variant: 'huge' };
</script>
