<template>
  <a-row>
    <action-button />
  </a-row>
  <a-row>
    <input-row />
  </a-row>
  <a-row>
    <a-col :span="24" style="overflow-y: auto; max-height: 100px">
      <state-list />
    </a-col>
  </a-row>
  <a-row>
    <a-col :span="12">
      <local-site />
    </a-col>
    <a-col :span="12" style="max-height: 400px">
      <remote-site />
    </a-col>
  </a-row>
  <a-row>
    <a-col :span="20">
      <transfe-list />
    </a-col>
  </a-row>
</template>

<script>
import store from "@/store/index";
import RemoteSite from "./components/RemoteSite.vue";
import LocalSite from "./components/LocalSite.vue";
import ActionButton from "./components/ActionButton.vue";
import StateList from "./components/StateList.vue";
import InputRow from "./components/InputRow.vue";
import TransfeList from "./components/TransfeList.vue";
import { invoke } from "@tauri-apps/api";
invoke("connect", {
  addr: "127.0.0.1:21",
  username: "root",
  password: "root",
}).then((response) => {
  store.state.stateList.push("状态：" + response);
});

export default {
  name: "App",
  components: {
    StateList,
    TransfeList,
    RemoteSite,
    LocalSite,
    InputRow,
    ActionButton,
  },
};
</script>

<style>
.ant-row {
  padding: 5px 0px !important;
  border: 1px solid black;
}
#app {
  font-size: 12px !important;
  overflow: hidden;
  cursor: pointer;
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
}
</style>
<style>
#components-layout-demo-basic .code-box-demo {
  text-align: center;
}
#components-layout-demo-basic .ant-layout-header,
#components-layout-demo-basic .ant-layout-footer {
  color: #fff;
  background: #7dbcea;
}
[data-theme="dark"] #components-layout-demo-basic .ant-layout-header {
  background: #6aa0c7;
}
[data-theme="dark"] #components-layout-demo-basic .ant-layout-footer {
  background: #6aa0c7;
}
#components-layout-demo-basic .ant-layout-footer {
  line-height: 1.5;
}
#components-layout-demo-basic .ant-layout-sider {
  color: #fff;
  line-height: 120px;
  background: #3ba0e9;
}
[data-theme="dark"] #components-layout-demo-basic .ant-layout-sider {
  background: #3499ec;
}
#components-layout-demo-basic .ant-layout-content {
  min-height: 120px;
  color: #fff;
  line-height: 120px;
  background: rgba(16, 142, 233, 1);
}
[data-theme="dark"] #components-layout-demo-basic .ant-layout-content {
  background: #107bcb;
}
#components-layout-demo-basic > .code-box-demo > .ant-layout + .ant-layout {
  margin-top: 48px;
}
</style>
