<template>
  <a-modal
    :visible="modalVisible"
    title="站点管理器"
    centered
    width="800px"
    @cancel="modalVisible = false"
    :maskClosable="false"
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
                {{ dataRef.Name }}
              </template>
              <template v-else>
                <div>
                  <a-input
                    @focus="handleFocus"
                    v-model:value="title"
                    size="small"
                    @pressEnter="handleEnter(dataRef)"
                  />
                </div>
              </template>
            </template>
            <template #switcherIcon>
              <PlusSquareOutlined />
            </template>
          </a-tree>
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
                  <a-select v-model:value="protocol">
                    <a-select-option
                      :value="elem"
                      v-for="(elem, index) in protocols"
                      :key="index"
                    >
                      {{ elem }}
                    </a-select-option>
                  </a-select>
                </a-form-item>
                <a-form layout="inline">
                  <a-form-item label="主机" name="host">
                    <a-input v-model:value="host" />
                  </a-form-item>
                  <a-form-item label="端口" name="port" style="width: 100px">
                    <a-input v-model:value="port" />
                  </a-form-item>
                </a-form>
              </a-form>
            </a-row>
            <a-row>
              <a-form>
                <a-form-item label="登录类型" name="type">
                  <a-select v-model:value="loginType">
                    <a-select-option
                      :value="elem"
                      v-for="(elem, index) in loginTypes"
                      :key="index"
                    >
                      {{ elem }}
                    </a-select-option>
                  </a-select>
                </a-form-item>
                <a-form-item label="用户" name="user">
                  <a-input v-model:value="user" />
                </a-form-item>
                <a-form-item label="密码" name="password">
                  <a-input-password v-model:value="pass" />
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
    <template #footer>
      <a-button>连接</a-button>
      <a-button @click="handleOk">确定</a-button>
      <a-button @click="modalVisible = false">取消</a-button>
    </template>
  </a-modal>
</template>

<script>
import store from "@/store/index";
import { invoke } from "@tauri-apps/api";
import { PlusSquareOutlined } from "@ant-design/icons-vue";

export default {
  components: {
    PlusSquareOutlined,
  },
  mounted() {
    this.getDefaultWftp();
    this.getWftpServer();
  },
  methods: {
    getDefaultWftp() {
      if (!localStorage.getItem("wftp_server")) {
        invoke("get_default_wftp", {}).then((response) => {
          localStorage.setItem("wftp_server", response);
        });
      }
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
    rename() {
      this.title = this.selected.Name;
      this.selected.editable = true;
    },
    handleEnter() {
      this.selected.Name = this.title;
      this.selected.editable = false;
      this.selected.writable = false;
      this.selected = "";
      this.saveXml();
    },
    saveXml() {
      invoke("wftp_xml_string", {
        xmlString: JSON.stringify(this.treeData[0].children),
      }).then((response) => {
        localStorage.setItem("wftp_server", response);
      });
    },
    getWftpServer() {
      invoke("get_wftp_server", { wftpXml: localStorage.getItem("wftp_server") }).then(
        (response) => {
          let temp = {};
          temp.Name = "我的站点";
          temp.key = "0";
          temp.children = [];
          response.map((elem, index) => {
            temp.children.push({
              Name: elem.Name,
              key: temp.key + "-" + index,
              Host: elem.Host,
              User: elem.User,
              Pass: elem.Pass,
              Port: elem.Port,
              Protocol: elem.Protocol,
              LogonType: elem.LogonType,
            });
          });
          this.treeData = [temp];
          store.state.wftpServer = response;
        }
      );
    },
    newLeaf() {
      this.title = "新站点";
      this.treeData[0].children.push({
        key: "0-" + this.treeData[0].children.length,
        Name: "新站点",
        editable: false,
        writable: true,
        Protocol: "1",
        LogonType: "1",
      });
    },
    del() {
      this.treeData[0].children.map((elem, key) => {
        if (elem.key == this.selectedKey[0]) {
          this.treeData[0].children.splice(key, 1);
        }
      });
      this.saveXml();
    },
    copy() {
      this.title = this.selected.Name;
      this.treeData[0].children.push({
        key: "0-" + this.treeData[0].children.length,
        Name: this.selected.Name,
        editable: false,
        writable: true,
        Protocol: "1",
        LogonType: "1",
      });
    },
    handleFocus(event) {
      event.target.select();
    },
    handleOk() {
      this.modelVisible = false;
      this.selected.Host = this.host;
      this.selected.Port = this.port;
      this.selected.User = this.user;
      this.selected.Pass = this.pass;
      this.selected.Protocol = this.protocol;
      this.selected.LogonType = this.loginType;
      this.saveXml();
    },
  },
  computed: {
    modalVisible: {
      get() {
        return store.state.modalVisible;
      },
      set(value) {
        store.state.modalVisible = value;
      },
    },
  },
  data() {
    return {
      selectedKey: [],
      title: "",
      selected: "",
      protocol: "FTP - 文件传输协议",
      protocols: ["SFTP - SSH FILE Transfer Protocol", "FTP - 文件传输协议"],
      loginType: "正常",
      loginTypes: ["询问", "正常"],
      activeKey: "1",
      host: "",
      user: "",
      port: "",
      pass: "",
      listActiveKey: "1",
      treeData: [],
    };
  },
};
</script>
