<template>
  <a-modal v-model:visible="visible" :wrap-style="{ overflow: 'hidden' }" @ok="handleOk">
    <a-input v-model:value="value" placeholder="Basic usage" ref="mkdir" />
    <template #title>
      <div style="width: 100%; cursor: move">{{ modalTitle }}</div>
    </template>
  </a-modal>
  <a-row
    v-show="state"
    style="max-height: 144px !important; min-height: 144px !important"
  >
    <a-col style="min-width: 100px !important; width: 100%">
      <div>
        <a-input :value="remotePath" addon-before="远程站点：" />
        <a-tree
          v-if="treeData.length > 0"
          style="
            overflow-y: auto;
            max-height: 100px !important;
            min-height: 100px !important;
          "
          :default-expanded-keys="['0']"
          :show-line="true"
          :tree-data="treeData"
          @select="onSelect"
          :showIcon="false"
        >
          <template #title="{ dataRef }">
            {{ dataRef.title }}
          </template>
        </a-tree>
      </div>
    </a-col>
  </a-row>
  <a-row
    style="min-height: 300px !important; max-height: 300px !important; overflow: auto"
    class="remoteTable"
  >
    <a-col style="">
      <a-table
        :customHeaderRow="customHeaderRow"
        v-mouse-menu="options"
        :columns="columns"
        :data-source="dataSource"
        :pagination="false"
        :customRow="customRow"
        :scroll="{ x: 800 }"
      >
        <template #bodyCell="{ column, text }">
          <template v-if="column.dataIndex === 'name'">
            <folder-open-outlined
              :style="{ color: '#ffe896' }"
              v-if="text.kind === 'folder'"
            />
            <file-outlined v-else />
            <a-input
              class="showInput"
              v-if="text.showInput"
              v-model:value="toName"
              :bordered="false"
              placeholder=""
              @pressEnter.prevent="renameInput"
              @focus.prevent="handleFocus"
              style="display: inline-block; width: 80px"
            />
            <text v-else :title="text.name">{{
              text.name.length > 20 ? text.name.slice(0, 20) + "..." : text.name
            }}</text>
          </template>
        </template>
      </a-table>
    </a-col>
  </a-row>
</template>
<script>
import store from "@/store/index";
import {
  FileOutlined,
  FolderOpenOutlined,
  ExclamationCircleOutlined,
} from "@ant-design/icons-vue";
import { MouseMenuDirective } from "@howdyjs/mouse-menu";
import { createVNode } from "vue";
import { Modal } from "ant-design-vue";
import {
  connect,
  readdir,
  getTree,
  cwd,
  prev,
  create,
  unlink,
  rmdir,
  mk_dir,
  rename_file,
  pwd,
  size_sort,
  getProtocol,
  download,
  dir_download,
} from "../apis/index";
import { writeBinaryFile } from "@tauri-apps/api/fs";
export default {
  props: {
    state: Boolean,
    data: Object,
    getLocalPath: Function,
    refreshLocal: Function,
  },
  directives: {
    MouseMenu: MouseMenuDirective,
  },
  components: {
    FileOutlined,
    FolderOpenOutlined,
  },
  data() {
    return {
      modalTitle: "",
      visible: false,
      toName: "",
      fromName: "",
      value: "",
      selected: {},
      options: {
        useLongPressInMobile: false,
        menuWrapperCss: {
          background: "white",
          boxShadow: "0 0 10px #ccc",
        },
        menuItemCss: {
          arrowSize: "5px",
          labelColor: "black",
          tipsColor: "#ccc",
          arrowColor: "#aaa",
          disabledColor: "#aaa",
        },
        menuList: [
          {
            label: "下载",
            tips: "Download",
            fn: () => {
              switch (this.selected.kind) {
                case "folder":
                  dir_download(this.getLocalPath(), this.selected.name).then(
                    (response) => {
                      let res = JSON.parse(response);
                      if (res.code == 200) {
                        console.log(response);
                        // writeBinaryFile(path, new Uint8Array(res.list)).then(() => {
                        //   this.refreshLocal();
                        //   store.state.stateList.push(
                        //     "响应：下载文件“" + this.selected.name + "”成功！"
                        //   );
                        // });
                      }
                    }
                  );
                  break;
                default:
                  store.state.stateList.push(
                    "命令：下载文件“" + this.selected.name + "”！"
                  );
                  download(this.selected.name).then((response) => {
                    let res = JSON.parse(response);
                    if (res.code == 200) {
                      let path = "";
                      let rootPath = this.getLocalPath();
                      if (rootPath != "/") {
                        path = rootPath + "/" + this.selected.name;
                      } else {
                        path = rootPath + this.selected.name;
                      }
                      writeBinaryFile(path, new Uint8Array(res.list)).then(() => {
                        this.refreshLocal();
                        store.state.stateList.push(
                          "响应：下载文件“" + this.selected.name + "”成功！"
                        );
                      });
                    }
                  });
                  break;
              }
            },
            disabled: () => {
              if (!this.selected) {
                return true;
              }
            },
          },
          {
            label: "添加文件到队列",
            tips: "Add",
            fn: (...args) => console.log("add", args),
            disabled: () => {
              return true;
            },
          },
          {
            label: "进入目录",
            tips: "Enter",
            fn: () => {
              cwd(this.currentPath).then((response) => {
                let res = JSON.parse(response);
                if (res.code == 200) {
                  this.getData();
                }
              });
            },
            disabled: () => {
              if (this.selected.kind != "folder") {
                return true;
              }
            },
          },
          {
            label: "查看/编辑",
            tips: "Check",
            fn: (...args) => console.log("check", args),
            disabled: () => true,
          },
          {
            line: true,
          },
          {
            label: "创建目录",
            tips: "Mkdir",
            fn: () => {
              this.visible = true;
              this.modalTitle = "创建目录";
              this.value = "/" + this.selected.name + "/new folder";
            },
          },
          {
            label: "创建文件",
            tips: "Put",
            fn: () => {
              this.visible = true;
              this.value = "/" + this.selected.name + "/new file";
              this.modalTitle = "创建文件";
            },
          },
          {
            label: "刷新",
            tips: "Refresh",
            fn: () => {
              this.getData();
            },
          },
          {
            line: true,
          },
          {
            label: "删除",
            tips: "Remove",
            fn: () => {
              store.state.stateList.push("命令：删除" + this.selected.name);
              Modal.confirm({
                title: "需要确认",
                icon: createVNode(ExclamationCircleOutlined),
                content: "确认要从服务器删除吗？",
                okText: "确认",
                cancelText: "取消",
                onOk: () => {
                  switch (this.selected.kind) {
                    case "folder":
                      rmdir(this.selected.name).then((response) => {
                        let res = JSON.parse(response);
                        if (res.code == 200) {
                          store.state.stateList.push("响应：" + res.msg);
                          this.getData();
                        }
                      });
                      break;
                    default:
                      unlink(this.selected.name).then((response) => {
                        let res = JSON.parse(response);
                        if (res.code == 200) {
                          store.state.stateList.push("响应：" + res.msg);
                          this.getData();
                        }
                      });
                      break;
                  }
                },
              });
            },
          },
          {
            label: "重命名",
            tips: "Rename",
            fn: () => {
              for (const key in this.dataSource) {
                if (this.dataSource[key].name.name == this.fromName) {
                  store.state.stateList.push("命令：修改文件夹" + this.fromName);
                  this.dataSource[key].name.showInput = true;
                }
              }
            },
          },
          {
            label: "复制 URL 到剪贴板",
            tips: "Copy",
            fn: () => {
              store.state.stateList.push("命令：复制URL");
              pwd().then((response) => {
                let res = JSON.parse(response);
                if (res.code == 200) {
                  let text = res.list + this.selected.name;
                  if (res.list !== "/") {
                    text = res.list + "/" + this.selected.name;
                  }
                  this.$copyText(text)
                    .then(() => {
                      store.state.stateList.push("响应：已经复制到了剪贴板");
                    })
                    .catch((err) => {
                      store.state.stateList.push("响应：" + err);
                    });
                } else {
                  store.state.stateList.push("状态：" + res.msg);
                }
              });
            },
          },
          {
            label: "文件权限",
            tips: "Permissions",
            fn: () => {},
            disabled: () => {
              return true;
            },
          },
        ],
      },
      remotePath: "/",
      treeData: [],
      sortWay: true,
      prevPath: "/",
      currentPath: "/",
      dirBool: true,
      dataSource: [],

      columns: [
        {
          title: "文件名",
          dataIndex: "name",
          key: "name",
          width: 150,
        },
        {
          title: "文件大小",
          dataIndex: "size",
          key: "size",
          width: 100,
        },
        {
          title: "文件类型",
          dataIndex: "is_directory",
          key: "is_directory",
          width: 150,
        },
        {
          title: "最近修改",
          dataIndex: "update_at",
          key: "update_at",
          width: 150,
        },
        {
          title: "权限",
          dataIndex: "permissions",
          key: "permissions",
          width: 100,
        },
        {
          title: "所有者",
          dataIndex: "owner",
          key: "owner",
          width: 50,
        },
        {
          title: "组",
          dataIndex: "group",
          key: "group",
          width: 50,
        },
      ],
    };
  },
  mounted() {
    store.state.connected = false;
    if (this.data) {
      connect();
    }
    this.$watch(
      () => {
        return store.state.connected;
      },
      function (newVal) {
        if (newVal) {
          this.getData();
        }
      }
    );
  },
  watch: {},
  methods: {
    getData() {
      this.selected = {};
      if (
        getProtocol().connectedId &&
        this.data.connectedId == getProtocol().connectedId
      ) {
        pwd().then((response) => {
          let res = JSON.parse(response);
          if (res.code == 200) {
            this.remotePath = res.list;
            store.state.stateList.push("命令：列出“" + res.list + "”的目录");
          } else {
            store.state.stateList.push("状态：" + res.msg);
          }
        });
        this.getTreeData();
        document
          .querySelectorAll("tr")
          .forEach((elem) => elem.classList.remove("selected"));
        readdir(this.currentPath).then((response) => {
          let res = JSON.parse(response);
          if (res.code == 200) {
            store.state.stateList.push("命令：列出“" + this.remotePath + "”的目录成功！");
            this.dataSource = [];
            this.dataSource = res.list;
          } else {
            store.state.stateList.push("状态：" + res.msg);
            prev().then(() => {
              pwd().then((response) => {
                let res = JSON.parse(response);
                if (res.code == 200) {
                  this.remotePath = res.list;
                }
              });
            });
            download(this.currentPath).then((response) => {
              let res = JSON.parse(response);
              if (res.code == 200) {
                let path = "";
                let rootPath = this.getLocalPath();
                if (rootPath != "/") {
                  path = rootPath + "/" + this.currentPath;
                } else {
                  path = rootPath + this.currentPath;
                }
                writeBinaryFile(path, new Uint8Array(res.list)).then(() => {
                  this.refreshLocal();
                  store.state.stateList.push(
                    "响应：下载文件“" + this.currentPath + "”成功！"
                  );
                });
              }
            });
          }
        });
      }
    },
    customHeaderRow() {
      return {
        onClick: (event) => {
          this.sortWay = !this.sortWay;
          if (
            event.target.innerText == "文件名" ||
            event.target.innerText == "文件大小"
          ) {
            let first = this.dataSource.shift();
            size_sort(this.dataSource, this.sortWay).then((response) => {
              let res = JSON.parse(response);
              if (res.code == 200) {
                this.dataSource = [].concat(first, res.list);
              }
              store.state.stateList.push("响应：" + res.msg);
            });
          }
        },
      };
    },
    customRow(record) {
      return {
        align: "left",
        onDblclick: () => {
          let prevPath = Object.assign("", this.currentPath);
          this.prevPath += prevPath;
          if (record.name.name != "..") {
            this.currentPath = record.name.name;
            if (record.name.path && record.name.path != "") {
              this.currentPath = record.name.path;
            }
            cwd(this.currentPath).then((response) => {
              let res = JSON.parse(response);
              if (res.code == 200) {
                this.getData();
              }
              store.state.stateList.push("响应：" + res.msg);
            });
          } else {
            this.currentPath = this.prevPath;
            prev().then((response) => {
              let res = JSON.parse(response);
              if (res.code == 200) {
                this.getData();
              }
              store.state.stateList.push("响应：" + res.msg);
            });
          }
        },
        onContextmenu: (event) => {
          this.currentPath = record.name.name;
          document
            .querySelectorAll("tr")
            .forEach((elem) => elem.classList.remove("selected"));
          event.target.parentNode.classList.add("selected");
          for (const key in this.dataSource) {
            this.dataSource[key].name.showInput = false;
          }
          this.toName = this.fromName = record.name.name;
          this.selected = record.name;
        },
        onclick: (event) => {
          this.selected = record.name;
          document
            .querySelectorAll("tr")
            .forEach((elem) => elem.classList.remove("selected"));
          event.target.parentNode.classList.add("selected");
        },
      };
    },
    renameInput() {
      rename_file(this.fromName, this.toName).then((response) => {
        let res = JSON.parse(response);
        if (res.code == 200) {
          store.state.stateList.push("响应：" + res.msg);
          this.getData();
        }
      });
    },
    handleFocus(event) {
      event.target.select();
    },
    getTreeData() {
      getTree().then((response) => {
        let res = JSON.parse(response);
        if (res.code == 200) {
          this.treeData = [res.list];
        } else {
          store.state.stateList.push("状态：" + res.msg);
        }
      });
    },
    handleOk() {
      switch (this.modalTitle) {
        case "创建文件":
          store.state.stateList.push("命令：创建文件" + this.value);
          create(this.value).then((response) => {
            let res = JSON.parse(response);
            if (res.code == 200) {
              store.state.stateList.push("响应：" + res.msg);
              this.getData();
            }
          });
          break;
        default:
          store.state.stateList.push("命令：创建文件夹" + this.value);
          mk_dir(this.value).then((response) => {
            let res = JSON.parse(response);
            if (res.code == 200) {
              store.state.stateList.push("响应：" + res.msg);
              this.getData();
            }
          });
          break;
      }
      this.visible = false;
    },
  },

  setup() {
    const onSelect = (selectedKeys, info) => {
      console.log("selected", selectedKeys, info);
    };
    return {
      onSelect,
    };
  },
};
</script>

<style>
.ant-table-cell {
  padding: 2px 5px !important;
  font-size: 10px !important;
}

.ant-table-thead > tr > th,
.ant-table-tbody > tr > td,
.ant-table tfoot > tr > th,
.ant-table tfoot > tr > td {
  padding: 2px 5px !important;
}

.ant-table-container table > thead > tr:first-child th {
  font-weight: bolder;
}

tr.selected {
  background-color: #1890ff;
}

.ant-table-cell-row-hover {
  background-color: white;
}

.remoteTable .ant-table-content {
  min-height: 290px !important;
}
</style>
