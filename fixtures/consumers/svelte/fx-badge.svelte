<!-- Must compile: renamed attribute, property-only and reserved-word props, default slot. -->
<script lang="ts">
  import { FxBadge } from '../../out/svelte/fx-badge/index.js';
</script>

<FxBadge count={3} tone="success" label="3 unread" for="inbox">new</FxBadge>
