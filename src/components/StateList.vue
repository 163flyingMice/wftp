<template>
  <a-list size="small" bordered :data-source="data">
    <template #renderItem="{ item }">
      <a-list-item>{{ item }}</a-list-item>
    </template>
  </a-list>
</template>
<script>
import { defineComponent } from "vue";
import store from "@/store/index";
export default defineComponent({
  setup() {
    let data = store.state.stateList;
    return {
      data,
    };
  },
});
</script>
