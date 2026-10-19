<!-- Must compile: props, event, default slot and the element through a template ref. -->
<script setup lang="ts">
import { useTemplateRef } from 'vue';
import { FxButton } from '../../out/vue/fx-button/index.js';

const button = useTemplateRef<InstanceType<typeof FxButton>>('button');
const onPress = (e: Event) => void button.value?.element?.focus();
</script>

<template>
  <FxButton ref="button" variant="danger" :disabled="false" @fx-press="(e) => e.currentTarget.variant.toUpperCase()">Go</FxButton>
  <FxButton @fx-press="onPress">Click</FxButton>
</template>
