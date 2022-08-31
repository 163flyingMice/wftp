<template>
  <a-row>
    <action-button
      :changeModelVisible="changeModelVisible"
      :refreshRemote="refreshRemote"
    />
  </a-row>
  <a-row>
    <input-row />
  </a-row>
  <a-row v-show="stateListState">
    <a-col :span="24" style="overflow-y: auto; max-height: 50px; min-height: 50px">
      <state-list />
    </a-col>
  </a-row>
  <a-tabs
    v-if="noConnect"
    @change="changeTab"
    v-model:activeKey="listActiveKey"
    type="editable-card"
    :hideAdd="true"
    @edit="editTab"
  >
    <a-tab-pane
      v-for="pane in panes"
      :key="pane.key"
      :tab="pane.title"
      :closable="pane.closable"
      ref="tabPane"
    >
      <a-row>
        <a-col :span="12">
          <local-site :state="localSiteState" :refreshRemote="refreshRemote" />
        </a-col>
        <a-col :span="12">
          <remote-site
            :state="remoteSiteState"
            :ref="'remoteSite' + pane.key"
            :data="pane"
          />
        </a-col>
      </a-row>
    </a-tab-pane>
  </a-tabs>
  <a-row v-else>
    <a-col :span="12">
      <local-site :state="localSiteState" :refreshRemote="refreshRemote" />
    </a-col>
    <a-col :span="12">
      <remote-site :state="remoteSiteState" />
    </a-col>
  </a-row>

  <a-row v-show="transfeListState">
    <a-col :span="24">
      <transfe-list />
    </a-col>
  </a-row>
  <site-manager
    ref="siteManager"
    v-if="modalVisible"
    :refreshWftpServer="getWftpServer"
  />
  <label-manager />
  <add-label />
  <folder-browser />
</template>

<script>
import store from "@/store/index";
import FolderBrowser from "./components/FolderBrowser.vue";
import RemoteSite from "./components/RemoteSite.vue";
import LocalSite from "./components/LocalSite.vue";
import ActionButton from "./components/ActionButton.vue";
import StateList from "./components/StateList.vue";
import InputRow from "./components/InputRow.vue";
import TransfeList from "./components/TransfeList.vue";
import SiteManager from "./components/SiteManager.vue";
import LabelManager from "./components/LabelManager.vue";
import AddLabel from "./components/AddLabel.vue";
import { invoke } from "@tauri-apps/api";
import { connect, getProtocol } from "./apis/index";

export default {
  name: "App",
  components: {
    FolderBrowser,
    AddLabel,
    LabelManager,
    StateList,
    TransfeList,
    RemoteSite,
    LocalSite,
    InputRow,
    ActionButton,
    SiteManager,
  },
  mounted() {
    if (this.panes.length != 0) {
      store.state.connectedName = this.panes[0].data.Name;
    }
    this.getDefaultWftp();
    this.getWftpServer();
  },
  methods: {
    getDefaultWftp() {
      if (
        !localStorage.getItem("wftp_server") ||
        localStorage.getItem("wftp_server") == "null"
      ) {
        invoke("get_default_wftp", {}).then((response) => {
          console.log(response);
          let res = JSON.parse(response);
          if (res.code == 200) {
            localStorage.setItem("wftp_server", res.list);
            this.getWftpServer();
          }
        });
      }
    },
    getWftpServer() {
      if (
        localStorage.getItem("wftp_server") &&
        localStorage.getItem("wftp_server") != "null"
      ) {
        invoke("get_wftp_server", { wftpXml: localStorage.getItem("wftp_server") }).then(
          (response) => {
            let res = JSON.parse(response);
            if (res.code == 200) {
              store.state.wftpServer = res.list;
            }
          }
        );
      }
    },
    changeModelVisible() {
      store.state.modalVisible = !store.state.modalVisible;
    },
    selectLeaf(key, elem) {
      this.selectedKey = key;
      this.selected = elem.node.dataRef;
      this.host = elem.node.Host;
      this.port = elem.node.Port;
      this.user = elem.node.User;
      this.pass = elem.node.Pass;
      this.protocol = this.protocols[elem.node.Protocol];
      this.loginType = this.loginTypes[elem.node.LogonType];
    },
    refreshRemote() {
      if (
        this.$refs["remoteSite" + this.listActiveKey] != undefined &&
        getProtocol().connectedId
      ) {
        this.$refs["remoteSite" + this.listActiveKey][0].getData();
      }
      if (getProtocol().connected != undefined && !getProtocol().connected) {
        connect();
      }
    },
    removeTab(targetKey) {
      this.panes = this.panes.filter((pane) => pane.key !== targetKey);
      if (
        this.panes.length > 0 &&
        this.listActiveKey != this.panes[this.panes.length - 1].key
      ) {
        this.listActiveKey = this.panes[this.panes.length - 1].key;
        this.refreshRemote();
      }
    },
    addTab() {},
    editTab(targetKey, action) {
      if (action === "add") {
        this.addTab();
      } else {
        this.removeTab(targetKey);
      }
    },
    changeTab(key) {
      let selectedPane;
      this.panes.forEach((pane) => {
        if (pane.key === key) {
          selectedPane = pane;
        }
      });
      this.refreshRemote();
      store.state.connectedKey = selectedPane.key;
    },
  },
  computed: {
    noConnect() {
      return store.state.panes.length;
    },
    modalVisible() {
      return store.state.modalVisible;
    },
    localSiteState() {
      return store.state.localSiteComponent;
    },
    stateListState() {
      return store.state.stateListComponent;
    },
    remoteSiteState() {
      return store.state.remoteSiteComponent;
    },
    transfeListState() {
      return store.state.transfeListComponent;
    },
    wftpServer() {
      return store.state.wftpServer;
    },
    panes: {
      get() {
        return store.state.panes;
      },
      set(value) {
        store.state.panes = value;
      },
    },
    listActiveKey: {
      get() {
        return store.state.listActiveKey;
      },
      set(value) {
        store.state.listActiveKey = value;
      },
    },
  },
  data() {
    return {};
  },
  setup() {},
};
</script>

<style>
.ant-form-item {
  margin-bottom: 0px !important;
}

.ant-row:not(.button) {
  padding: 4px 0px 4px 1px !important;
  border: 2px solid silver;
}

#app {
  font-size: 12px !important;
  overflow: hidden;
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

.ant-tabs-top > .ant-tabs-nav,
.ant-tabs-bottom > .ant-tabs-nav,
.ant-tabs-top > div > .ant-tabs-nav,
.ant-tabs-bottom > div > .ant-tabs-nav {
  margin: 0 0 6px 0 !important;
}
</style>
