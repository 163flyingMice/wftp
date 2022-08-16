<template>
  <a-button type="default" :onclick="reloadApp">
    <template #icon><redo-outlined /></template>
  </a-button>
</template>
<script>
import { RedoOutlined } from "@ant-design/icons-vue";
export default {
  components: {
    RedoOutlined,
  },
  methods: {
    reloadApp() {
      window.location.reload();
    },
  },
};
</script>
