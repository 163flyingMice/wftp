<template>
  <a-form :model="formState" name="horizontal_login" layout="inline" autocomplete="off" @finish="onFinish"
    @finishFailed="onFinishFailed" :hideRequiredMark="true">
    <a-form-item label="主机" name="host" style="width: 200px">
      <a-input v-model:value="formState.host"> </a-input>
    </a-form-item>

    <a-form-item label="用户名" name="user" style="width: 200px">
      <a-input v-model:value="formState.user"> </a-input>
    </a-form-item>

    <a-form-item label="端口" name="port" style="width: 100px">
      <a-input v-model:value="formState.port"> </a-input>
    </a-form-item>

    <a-form-item label="密码" name="pass" style="width: 200px">
      <a-input-password v-model:value="formState.pass"> </a-input-password>
    </a-form-item>

    <a-form-item>
      <a-button :disabled="disabled" type="primary" html-type="submit">快速连接</a-button>
      <a-dropdown>
        <template #overlay>
          <a-menu>
            <a-menu-item key="1"> 清除快速连接栏 </a-menu-item>
            <a-menu-item key="2"> 清除历史记录 </a-menu-item>
          </a-menu>
        </template>
        <a-button>
          <DownOutlined />
        </a-button>
      </a-dropdown>
    </a-form-item>
  </a-form>
</template>
<script>
import { defineComponent, reactive, computed } from "vue";
import { DownOutlined } from "@ant-design/icons-vue";
import store from "@/store/index";
import { Modal } from "ant-design-vue";
export default defineComponent({
  components: { DownOutlined },

  setup() {
    const formState = reactive({
      host: "",
      port: "",
      user: "",
      pass: "",
      protocol: "",
    });

    const onFinish = (values) => {
      let protocols = ["sftp", "ftp"];
      if (values.host.search("://") != -1) {
        let temp = values.host.split("://")
        values.host = temp[1]
        values.protocol = protocols.indexOf(temp[0]) == -1 ? 1 : protocols.indexOf(temp[0]);
      } else {
        Modal.error({
          title: "填写错误提示",
          content: "主机的格式应为：ftp://主机\nsftp://主机",
        });
        return
      }
      let key = store.state.panes.length != 0 ? store.state.panes.length + 1 : 1;
      store.state.panes.push({
        title: "临时连接",
        key: key,
        data: {
          Host: values.host,
          User: values.user,
          Pass: values.pass,
          Port: values.port,
          Protocol: values.protocol,
          Name: "临时连接",
        },
      });
      store.state.listActiveKey = key;
    };

    const onFinishFailed = (errorInfo) => {
      console.log("Failed:", errorInfo);
    };

    const disabled = computed(() => {
      return !(formState.user && formState.pass && formState.host && formState.port);
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
