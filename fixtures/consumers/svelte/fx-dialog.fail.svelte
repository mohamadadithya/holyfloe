<!-- Each marked line must fail to type-check (props objects: svelte-check has no expect-error for markup). -->
<script lang="ts">
  import type { ComponentProps } from 'svelte';
  import { FxDialog } from '../../out/svelte/fx-dialog/index.js';

  type Props = ComponentProps<typeof FxDialog>;
  // @ts-expect-error open is a boolean
  export const badOpen: Props = { open: 'yes' };
  // @ts-expect-error open-change detail.open is a boolean
  export const badDetail: Props = { onOpenChange: (e) => e.detail.open.toUpperCase() };
</script>
