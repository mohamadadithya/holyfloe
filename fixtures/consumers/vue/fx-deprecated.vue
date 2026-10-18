<!-- Must compile: a deprecated component and prop still type-check. -->
<script setup lang="ts">
import { FxDeprecated } from '../../out/vue/fx-deprecated/index.js';
</script>

<template>
  <FxDeprecated tone="error" kind="x" />
</template>
