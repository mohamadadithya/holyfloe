<!-- Must compile: v-model:open, the open-change event, the footer slot, and show() on the instance. -->
<script setup lang="ts">
import { ref, useTemplateRef } from 'vue';
import { FxDialog } from '../../out/vue/fx-dialog/index.js';

const open = ref(false);
const dialog = useTemplateRef<InstanceType<typeof FxDialog>>('dialog');
const showIt = () => dialog.value?.show();
const isOpen = (): boolean | undefined => dialog.value?.element?.open;
</script>

<template>
  <FxDialog ref="dialog" v-model:open="open" @open-change="(e) => e.detail.open === isOpen()">
    Body
    <template #footer><button @click="showIt">OK</button></template>
  </FxDialog>
</template>
