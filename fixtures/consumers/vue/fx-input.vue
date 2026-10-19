<!-- Must compile: v-model on value, change event, raw tag. -->
<script setup lang="ts">
import { ref } from 'vue';
import { FxInput } from '../../out/vue/fx-input/index.js';

const text = ref('');
</script>

<template>
  <FxInput v-model="text" name="q" :disabled="false" @change="(e) => e.currentTarget.value.toUpperCase()" />
</template>
