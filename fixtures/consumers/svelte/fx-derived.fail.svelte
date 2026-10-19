<!-- Each marked line must fail to type-check (props objects: svelte-check has no expect-error for markup). -->
<script lang="ts">
  import type { ComponentProps } from 'svelte';
  import { FxDerived } from '../../out/svelte/fx-derived/index.js';

  type Props = ComponentProps<typeof FxDerived>;
  // @ts-expect-error size (inherited) is a union of strings
  export const badSize: Props = { size: 'xxl' };
  // @ts-expect-error loading (inherited) is a boolean
  export const badLoading: Props = { loading: 1 };
</script>
