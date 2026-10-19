<!-- Each marked element must fail to type-check. -->
<script setup lang="ts">
import { FxCard } from '../../out/vue/fx-card/index.js';
</script>

<template>
  <!-- @vue-expect-error elevated is a boolean -->
  <FxCard elevated="yes" />
  <FxCard>
    <!-- @vue-expect-error the card has no `sidebar` slot -->
    <template #sidebar>x</template>
  </FxCard>
</template>
