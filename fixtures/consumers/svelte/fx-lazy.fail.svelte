<!-- Each marked line must fail to type-check (props objects: svelte-check has no expect-error for markup). -->
<script lang="ts">
  import type { ComponentProps } from 'svelte';
  import { FxLazy } from '../../out/svelte/fx-lazy/index.js';

  // @ts-expect-error items are strings
  export const badItems: ComponentProps<typeof FxLazy> = { items: [1] };
</script>
