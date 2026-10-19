<!-- Each marked line must fail to type-check (props objects: svelte-check has no expect-error for markup). -->
<script lang="ts">
  import type { ComponentProps } from 'svelte';
  import { FxEvents } from '../../out/svelte/fx-events/index.js';

  type Props = ComponentProps<typeof FxEvents>;
  // @ts-expect-error my-change detail.value is a string
  export const badKebab: Props = { onMyChange: (e) => e.detail.value.toFixed() };
  // @ts-expect-error MyChangePascal detail has checked, not value
  export const badPascal: Props = { onMyChangePascal: (e) => e.detail.value };
</script>
