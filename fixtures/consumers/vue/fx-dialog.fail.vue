<!-- Each marked line must fail to type-check. -->
<script setup lang="ts">
import { ref, useTemplateRef } from 'vue';
import { FxDialog } from '../../out/vue/fx-dialog/index.js';

const label = ref('closed');
const dialog = useTemplateRef<InstanceType<typeof FxDialog>>('dialog');
// @ts-expect-error the element has no `toggle` method
dialog.value?.toggle();
</script>

<template>
  <!-- @vue-expect-error open is a boolean, v-model:open needs a boolean ref -->
  <FxDialog ref="dialog" v-model:open="label" />
  <!-- @vue-expect-error open-change detail.open is a boolean -->
  <FxDialog @open-change="(e) => e.detail.open.toUpperCase()" />
</template>
