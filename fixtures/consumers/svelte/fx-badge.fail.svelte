<!-- Each marked line must fail to type-check (props objects: svelte-check has no expect-error for markup). -->
<script lang="ts">
  import type { ComponentProps } from 'svelte';
  import { FxBadge } from '../../out/svelte/fx-badge/index.js';

  type Props = ComponentProps<typeof FxBadge>;
  // @ts-expect-error count is a number
  export const badCount: Props = { count: '3' };
  // @ts-expect-error overflow is read-only, not a prop
  export const badOverflow: Props = { overflow: true };
</script>
