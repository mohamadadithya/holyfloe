<!-- Must compile: default and named slots, boolean prop. -->
<script setup lang="ts">
import { FxCard } from '../../out/vue/fx-card/index.js';
</script>

<template>
  <FxCard elevated style="--fx-card-padding: 3px">
    <template #header><h2>Head</h2></template>
    <p>Body</p>
    <template #footer><span>Foot</span></template>
  </FxCard>
</template>
