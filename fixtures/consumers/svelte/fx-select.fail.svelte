<!-- Each marked line must fail to type-check (props objects: svelte-check has no expect-error for markup). -->
<script lang="ts">
  import type { ComponentProps } from 'svelte';
  import { FxSelect } from '../../out/svelte/fx-select/index.js';

  type Props = ComponentProps<typeof FxSelect>;
  // @ts-expect-error options are strings
  export const badOptions: Props = { options: [1, 2] };
  // @ts-expect-error fx-change detail.value is a string
  export const badDetail: Props = { onFxChange: (e) => e.detail.value.toFixed() };
</script>
