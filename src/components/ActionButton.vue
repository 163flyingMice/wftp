<template>
  <a-button type="default" :onclick="reloadApp">
    <template #icon><menu-fold-outlined /></template>
  </a-button>
  <a-button type="default" :onclick="reloadApp">
    <template #icon><redo-outlined /></template>
  </a-button>
  <a-button type="default" :onclick="reloadApp">
    <template #icon><redo-outlined /></template>
  </a-button>
  <a-button type="default" :onclick="reloadApp">
    <template #icon><redo-outlined /></template>
  </a-button>
</template>
<script>
import { RedoOutlined, MenuFoldOutlined } from "@ant-design/icons-vue";
export default {
  components: {
    RedoOutlined,
    MenuFoldOutlined,
  },
  methods: {
    reloadApp() {
      window.location.reload();
    },
  },
};
</script>
