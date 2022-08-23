<template>
  <a-row>
    <menu-bar />
  </a-row>
  <a-row>
    <action-button :changeModelVisible="changeModelVisible" />
  </a-row>
  <a-row>
    <input-row />
  </a-row>
  <a-row v-show="stateListState">
    <a-col :span="24" style="overflow-y: auto; max-height: 50px">
      <state-list />
    </a-col>
  </a-row>
  <a-row>
    <a-col :span="12">
      <local-site :state="localSiteState" />
    </a-col>
    <a-col :span="12">
      <remote-site :state="remoteSiteState" />
    </a-col>
  </a-row>
  <a-row>
    <a-col :span="20">
      <transfe-list />
    </a-col>
  </a-row>
  <a-row> </a-row>
  <a-modal
    :visible="modelVisible"
    title="站点管理器"
    centered
    width="800px"
    cancelText="取消"
    okText="确定"
    @cancel="modelVisible = false"
    @ok="modelVisible = false"
  >
    <a-layout>
      <a-layout-sider theme="light" width="350px" style="margin-right: 6px">
        <a-row style="height: 300px">
          <a-tree
            :tree-data="treeData"
            :show-icon="true"
            @select="selectLeaf"
            :defaultExpandAll="true"
          >
            <template #title="dataRef">
              <template v-if="!dataRef.editable && !dataRef.writable">
                {{ dataRef.title }}
              </template>
              <template v-else>
                <div>
                  <a-input
                    v-model:value="title"
                    size="small"
                    @pressEnter="handleEnter(dataRef)"
                  />
                </div>
              </template>
            </template>
            <template #switcherIcon> <PlusSquareOutlined /> </template
          ></a-tree>
        </a-row>
        <a-row style="margin-top: 5px" class="button">
          <a-col :span="24" align="middle">
            <a-button type="default" @click="newLeaf">新站点</a-button>
            <a-button type="default">新文件夹</a-button>
          </a-col>
        </a-row>
        <a-row style="margin-top: 5px" class="button">
          <a-col :span="24" align="middle">
            <a-button type="default">新键书签</a-button>
            <a-button type="default" @click="rename" :disabled="selected == ''"
              >重命名</a-button
            >
          </a-col>
        </a-row>
        <a-row style="margin-top: 5px" class="button">
          <a-col :span="24" align="middle">
            <a-button type="default" @click="del" :disabled="selected == ''"
              >删除</a-button
            >
            <a-button type="default" @click="copy" :disabled="selected == ''"
              >复制</a-button
            >
          </a-col>
        </a-row>
      </a-layout-sider>
      <a-layout>
        <a-tabs v-model:activeKey="activeKey" type="card">
          <a-tab-pane key="1" tab="常规">
            <a-row>
              <a-form>
                <a-form-item label="协议" name="protocol">
                  <a-select :value="protocol">
                    <a-select-option
                      :value="elem"
                      v-for="(elem, index) in protocols"
                      :key="index"
                      >{{ elem }}</a-select-option
                    >
                  </a-select>
                </a-form-item>
                <a-form layout="inline">
                  <a-form-item label="主机" name="host">
                    <a-input :value="host" />
                  </a-form-item>
                  <a-form-item label="端口" name="port" style="width: 80px">
                    <a-input :value="port" />
                  </a-form-item>
                </a-form>
              </a-form>
            </a-row>
            <a-row>
              <a-form>
                <a-form-item label="登录类型" name="type">
                  <a-select :value="loginType">
                    <a-select-option
                      :value="elem"
                      v-for="(elem, index) in loginTypes"
                      :key="index"
                      >{{ elem }}</a-select-option
                    >
                  </a-select>
                </a-form-item>
                <a-form-item label="用户" name="user">
                  <a-input :value="user" />
                </a-form-item>
                <a-form-item label="密码" name="password">
                  <a-input-password :value="pass" />
                </a-form-item>
              </a-form>
            </a-row>
            <a-row>
              <a-form>
                <a-form-item label="背景颜色" name="color" style="width: 120px">
                  <a-select>
                    <a-select-option value="111"></a-select-option>
                  </a-select>
                </a-form-item>
                <a-form-item label="注释" name="remark" style="width: 350px">
                  <a-textarea placeholder="" :auto-size="{ minRows: 5, maxRows: 10 }" />
                </a-form-item>
              </a-form>
            </a-row>
          </a-tab-pane>
          <a-tab-pane key="2" tab="高级"></a-tab-pane>
          <a-tab-pane key="3" tab="传输设置"></a-tab-pane>
          <a-tab-pane key="4" tab="字符集"></a-tab-pane>
        </a-tabs>
      </a-layout>
    </a-layout>
  </a-modal>
</template>

<script>
import store from "@/store/index";
import RemoteSite from "./components/RemoteSite.vue";
import LocalSite from "./components/LocalSite.vue";
import ActionButton from "./components/ActionButton.vue";
import StateList from "./components/StateList.vue";
import InputRow from "./components/InputRow.vue";
import TransfeList from "./components/TransfeList.vue";
import MenuBar from "./components/MenuBar.vue";
import { invoke } from "@tauri-apps/api";
import { PlusSquareOutlined } from "@ant-design/icons-vue";
setInterval(() => {
  invoke("alive", {}).then((response) => {
    console.log(response);
  });
}, 10000);
invoke("connect", {
  addr: "127.0.0.1:21",
  username: "root",
  password: "root",
}).then((response) => {
  if (response == "连接成功！") {
    store.state.connected = true;
  }
  store.state.stateList.push("状态：" + response);
});

export default {
  name: "App",
  components: {
    PlusSquareOutlined,
    StateList,
    MenuBar,
    TransfeList,
    RemoteSite,
    LocalSite,
    InputRow,
    ActionButton,
  },
  mounted() {
    invoke("get_wftp_server", {}).then((response) => {
      let temp = {};
      temp.title = "我的站点";
      temp.key = "0";
      temp.children = [];
      temp.editable = false;
      temp.writable = false;
      response.map((elem, index) => {
        temp.children.push({
          title: elem.Name,
          key: temp.key + "-" + index,
          host: elem.Host,
          user: elem.User,
          pass: elem.Pass,
          port: elem.Port,
          protocol: elem.Protocol,
          loginType: elem.LogonType,
          editable: false,
          writable: false,
        });
      });
      this.treeData = [temp];
      store.state.wftpServer = response;
    });
  },
  methods: {
    changeModelVisible() {
      this.modelVisible = !this.modelVisible;
    },
    selectLeaf(key, elem) {
      this.selected = elem.node.dataRef;
      this.host = elem.node.host;
      this.port = elem.node.port;
      this.user = elem.node.user;
      this.pass = elem.node.pass;
      this.protocol = this.protocols[elem.node.protocol];
      this.loginType = this.loginTypes[elem.node.loginType];
    },
    rename() {
      this.title = this.selected.title;
      this.selected.editable = true;
    },
    handleEnter() {
      this.selected.title = this.title;
      this.selected.editable = false;
      this.selected.writable = false;
      this.selected = "";
    },
    newLeaf() {
      this.title = "新站点";
      this.treeData[0].children.push({
        key: "0-" + this.treeData[0].children.length,
        title: "新站点",
        editable: false,
        writable: true,
        protocol: "1",
        loginType: "1",
      });
    },
    del() {},
  },
  computed: {
    localSiteState() {
      return store.state.localSiteComponent;
    },
    stateListState() {
      return store.state.stateListComponent;
    },
    remoteSiteState() {
      return store.state.remoteSiteComponent;
    },
    wftpServer() {
      return store.state.wftpServer;
    },
  },
  data() {
    return {
      title: "",
      selected: "",
      protocol: "FTP - 文件传输协议",
      protocols: ["SFTP - SSH FILE Transfer Protocol", "FTP - 文件传输协议"],
      loginType: "正常",
      loginTypes: ["询问", "正常"],
      activeKey: "1",
      modelVisible: false,
      host: "",
      user: "",
      port: "",
      pass: "",
      treeData: [],
    };
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
</style>
