<!-- Each marked element must fail to type-check. -->
<script setup lang="ts">
import { FxMixin } from '../../out/vue/fx-mixin/index.js';
</script>

<template>
  <!-- @vue-expect-error focused (from the mixin) is a boolean -->
  <FxMixin focused="yes" />
  <!-- @vue-expect-error ringColor (from the mixin) is a string -->
  <FxMixin :ring-color="1" />
</template>
