<!-- Must compile: renamed attribute, property-only and reserved-word props, default slot. -->
<script setup lang="ts">
import { FxBadge } from '../../out/vue/fx-badge/index.js';
</script>

<template>
  <FxBadge :count="3" tone="success" label="3 unread" for="inbox">new</FxBadge>
</template>
