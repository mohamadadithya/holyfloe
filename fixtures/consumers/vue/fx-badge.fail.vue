<!-- Each marked element must fail to type-check. -->
<script setup lang="ts">
import { FxBadge } from '../../out/vue/fx-badge/index.js';
</script>

<template>
  <!-- @vue-expect-error count is a number -->
  <FxBadge count="3" />
  <!-- @vue-expect-error overflow is read-only, not a prop -->
  <FxBadge :overflow="true" />
</template>
