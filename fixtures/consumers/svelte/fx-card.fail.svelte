<!-- Each marked line must fail to type-check (props objects: svelte-check has no expect-error for markup). -->
<script lang="ts">
  import type { ComponentProps } from 'svelte';
  import { FxCard } from '../../out/svelte/fx-card/index.js';

  type Props = ComponentProps<typeof FxCard>;
  // @ts-expect-error elevated is a boolean
  export const badElevated: Props = { elevated: 'yes' };
  // @ts-expect-error header is a snippet
  export const badHeader: Props = { header: 'Head' };
</script>
