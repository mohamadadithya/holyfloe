<!-- Must compile: props, event and method from the dependency mixin. -->
<script lang="ts">
  import { FxMixin } from '../../out/svelte/fx-mixin/index.js';

  let element: HTMLElementTagNameMap['fx-mixin'] | undefined = $state();
</script>

<FxMixin bind:element prefix="> " focused ringColor="red" onFocusChange={() => element?.toggleFocus()}>Label</FxMixin>
