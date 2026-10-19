<!-- Each marked line must fail to type-check (props objects: svelte-check has no expect-error for markup). -->
<script lang="ts">
  import type { ComponentProps } from 'svelte';
  import { FxInput } from '../../out/svelte/fx-input/index.js';

  type Props = ComponentProps<typeof FxInput>;
  // @ts-expect-error value is a string
  export const badValue: Props = { value: 1 };
  // @ts-expect-error the change event target is the element, whose value is a string
  export const badTarget: Props = { onChange: (e) => e.currentTarget.value.toFixed() };
</script>
