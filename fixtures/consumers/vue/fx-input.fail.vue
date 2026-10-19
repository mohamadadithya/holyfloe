<!-- Each marked element must fail to type-check. -->
<script setup lang="ts">
import { ref } from 'vue';
import { FxInput } from '../../out/vue/fx-input/index.js';

const count = ref(0);
</script>

<template>
  <!-- @vue-expect-error value is a string, v-model needs a string ref -->
  <FxInput v-model="count" />
  <!-- @vue-expect-error the wrapper binds value through v-model (modelValue), not a `value` prop -->
  <FxInput value="x" />
</template>
