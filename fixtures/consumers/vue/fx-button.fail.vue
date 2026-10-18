<!-- Each marked element must fail to type-check. -->
<script setup lang="ts">
import { FxButton } from '../../out/vue/fx-button/index.js';
</script>

<template>
  <!-- @vue-expect-error variant is a union of strings -->
  <FxButton variant="huge" />
  <!-- @vue-expect-error disabled is a boolean -->
  <FxButton :disabled="'yes'" />
  <!-- @vue-expect-error the event target is the element; variant is a string -->
  <FxButton @fx-press="(e) => e.currentTarget.variant.toFixed()" />
</template>
