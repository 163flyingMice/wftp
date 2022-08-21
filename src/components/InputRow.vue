<template>
  <a-form :model="formState" name="horizontal_login" layout="inline" autocomplete="off" @finish="onFinish"
    @finishFailed="onFinishFailed" :hideRequiredMark="true">
    <a-form-item label="主机" name="host" style="width: 200px">
      <a-input v-model:value="formState.host"> </a-input>
    </a-form-item>

    <a-form-item label="用户名" name="username" style="width: 200px">
      <a-input v-model:value="formState.username"> </a-input>
    </a-form-item>

    <a-form-item label="端口" name="port" style="width: 100px">
      <a-input v-model:value="formState.port"> </a-input>
    </a-form-item>

    <a-form-item label="密码" name="password" style="width: 200px">
      <a-input-password v-model:value="formState.password"> </a-input-password>
    </a-form-item>

    <a-form-item>
      <a-button :disabled="disabled" type="primary" html-type="submit">快速连接</a-button>
    </a-form-item>
  </a-form>
</template>
<script>
import { defineComponent, reactive, computed } from "vue";
import { invoke } from "@tauri-apps/api";
import store from "@/store/index";
export default defineComponent({
  components: {},

  setup() {
    const formState = reactive({
      host: "",
      port: "",
      username: "",
      password: "",
    });

    const onFinish = (values) => {
      invoke("connect", {
        addr: values.host + ":" + values.port,
        username: values.username,
        password: values.password,
      }).then((response) => {
        if (response == "连接成功！") {
          store.state.connected = true;
        }
        store.state.stateList.push("状态：" + response);
      });
    };

    const onFinishFailed = (errorInfo) => {
      console.log("Failed:", errorInfo);
    };

    const disabled = computed(() => {
      return !(formState.username && formState.password && formState.host && formState.port);
    });
    return {
      formState,
      onFinish,
      onFinishFailed,
      disabled,
    };
  },
});
</script>
<style>
.ant-form-item {
  border: none !important;
}
</style>
