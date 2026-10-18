<!-- Must compile: props, event and method from the dependency mixin. -->
<script setup lang="ts">
import { useTemplateRef } from 'vue';
import { FxMixin } from '../../out/vue/fx-mixin/index.js';

const mixin = useTemplateRef<InstanceType<typeof FxMixin>>('mixin');
const toggle = () => mixin.value?.toggleFocus();
</script>

<template>
  <FxMixin ref="mixin" prefix="> " focused ring-color="red" @focus-change="toggle">Label</FxMixin>
</template>
