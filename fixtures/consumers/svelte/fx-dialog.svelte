<!-- Must compile: bind:open, the open-change event, the footer snippet, and show() through bind:element. -->
<script lang="ts">
  import { FxDialog } from '../../out/svelte/fx-dialog/index.js';

  let open = $state(false);
  let element: HTMLElementTagNameMap['fx-dialog'] | undefined = $state();
</script>

<FxDialog bind:open bind:element onOpenChange={(e) => e.detail.open === open}>
  Body
  {#snippet footer()}<button onclick={() => element?.show()}>OK</button>{/snippet}
</FxDialog>
