<!-- Each marked line must fail to type-check (props objects: svelte-check has no expect-error for markup). -->
<script lang="ts">
  import type { ComponentProps } from 'svelte';
  import { FxMixin } from '../../out/svelte/fx-mixin/index.js';

  type Props = ComponentProps<typeof FxMixin>;
  // @ts-expect-error focused (from the mixin) is a boolean
  export const badFocused: Props = { focused: 'yes' };
  // @ts-expect-error ringColor (from the mixin) is a string
  export const badRing: Props = { ringColor: 1 };
</script>
