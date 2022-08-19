<template>
  <a-list size="small" :data-source="data" class="stateScroll">
    <template #renderItem="{ item }">
      <a-list-item>{{ item }}</a-list-item>
    </template>
  </a-list>
</template>
<script>
import { defineComponent, ref } from "vue";
import store from "@/store/index";
export default defineComponent({
  computed: {
    stateList() {
      return ref(store.state.stateList.length);
    },
  },
  watch: {
    stateList(val) {
      this.$nextTick(() => {
        document.querySelector(".stateScroll").parentElement.scrollTop =
          document.querySelector(".ant-list-item").scrollHeight * val.value;
      });
    },
  },
  setup() {
    let data = store.state.stateList;
    return {
      data,
    };
  },
});
</script>
<style>
.ant-list-sm .ant-list-item {
  font-size: 12px !important;
  padding: 4px 2px !important;
}
.ant-list-split .ant-list-item {
  border-bottom: none !important;
}
</style>
