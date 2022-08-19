<template>
  <a-modal v-model:visible="visible" :wrap-style="{ overflow: 'hidden' }" @ok="handleOk">
    <a-input v-model:value="value" placeholder="Basic usage" ref="mkdir" />
    <template #title>
      <div style="width: 100%; cursor: move">{{ modalTitle }}</div>
    </template>
  </a-modal>
  <a-row>
    <a-col style="min-width: 100px !important; width: 100%">
      <div>
        <a-input :value="currentPath" addon-before="远程站点：" />
        <a-tree
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
            <template v-if="dataRef.key === '0-0-0-1'">
              <div>multiple line title</div>
              <div>multiple line title</div>
            </template>
            <template v-else>{{ dataRef.title }}</template>
          </template>
        </a-tree>
      </div></a-col
    >
  </a-row>
  <a-row style="min-height: 300px; max-height: 300px; overflow: auto">
    <a-col style="">
      <a-table
        v-mouse-menu="options"
        :columns="columns"
        :data-source="dataSource"
        :pagination="false"
        :customRow="customRow"
        :scroll="{ x: 800 }"
      >
        <template #bodyCell="{ column, text }">
          <template v-if="column.dataIndex === 'name'"
            ><folder-open-outlined
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
            <text v-else>{{ text.name }}</text>
          </template>
        </template>
      </a-table></a-col
    >
  </a-row>
</template>
<script>
import store from "@/store/index";
import {
  FileOutlined,
  FolderOpenOutlined,
  ExclamationCircleOutlined,
} from "@ant-design/icons-vue";
import { invoke } from "@tauri-apps/api";
import { MouseMenuDirective } from "@howdyjs/mouse-menu";
import { createVNode } from "vue";
import { Modal } from "ant-design-vue";
export default {
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
            fn: (...args) => console.log("download", args),
          },
          {
            label: "添加文件到队列",
            tips: "Add",
            fn: (...args) => console.log("add", args),
          },
          {
            label: "进入目录",
            tips: "Enter",
            fn: () => {
              invoke("cwd", {
                path: this.currentPath,
              }).then((response) => {
                store.state.stateList.push("状态：" + response);
              });
              this.getData();
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
              this.value = "/" + this.selected.name + "/创建目录";
            },
          },
          {
            label: "创建文件",
            tips: "Put",
            fn: () => {
              this.visible = true;
              this.value = "/" + this.selected.name + "/创建文件名";
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
                content: "确认要从服务器删除一个文件吗？",
                okText: "确认",
                cancelText: "取消",
                onOk: () => {
                  switch (this.selected.kind) {
                    case "folder":
                      invoke("remove_dir", {
                        path: this.selected.name,
                      }).then((response) => {
                        store.state.stateList.push("响应：" + response);
                        this.getData();
                      });
                      break;
                    default:
                      invoke("remove_file", {
                        filename: this.selected.name,
                      }).then((response) => {
                        store.state.stateList.push("响应：" + response);
                        this.getData();
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
              invoke("pwd", {}).then((response) => {
                let text = response + this.selected.name;
                if (response !== "/") {
                  text = response + "/" + this.selected.name;
                }
                this.$copyText(text)
                  .then(() => {
                    store.state.stateList.push("响应：已经复制到了剪贴板");
                  })
                  .catch((err) => {
                    store.state.stateList.push("响应：" + err);
                  });
              });
            },
          },
          {
            label: "文件权限",
            tips: "Permissions",
            fn: () => {},
          },
        ],
      },
      treeData: [],
      prevPath: "/",
      currentPath: "/",
      dataSource: [],

      columns: [
        {
          title: "文件名",
          dataIndex: "name",
          key: "name",
          width: 300,
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
          width: 100,
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
    this.getData();
  },
  methods: {
    getData() {
      this.getTreeData();
      document
        .querySelectorAll("tr")
        .forEach((elem) => elem.classList.remove("selected"));
      invoke("list", {
        path: this.currentPath,
      }).then((response) => {
        store.state.stateList.push("状态：列出“" + this.currentPath + "”的目录成功");
        this.dataSource = response;
      });
    },

    customRow(record) {
      return {
        align: "left",
        onDblclick: () => {
          let prevPath = Object.assign("", this.currentPath);
          this.prevPath += prevPath;
          if (record.name.name != "..") {
            this.currentPath = record.name.name;
            invoke("cwd", {
              path: this.currentPath,
            }).then((response) => {
              store.state.stateList.push("状态：" + response);
            });
          } else {
            this.currentPath = this.prevPath;
            invoke("prev").then((response) => {
              store.state.stateList.push("状态：" + response);
            });
          }
          this.getData();
        },
        onContextmenu: (event) => {
          this.currentPath = record.name.name;
          document
            .querySelectorAll("tr")
            .forEach((elem) => elem.classList.remove("selected"));
          event.target.parentElement.className = "selected";
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
          event.target.parentElement.className = "selected";
        },
      };
    },
    renameInput() {
      invoke("rename_file", {
        fromName: this.fromName,
        toName: this.toName,
      }).then((response) => {
        store.state.stateList.push("响应：" + response);
      });
      this.getData();
    },
    handleFocus(event) {
      event.target.select();
    },
    getTreeData() {
      invoke("folder_list", {}).then((response) => {
        this.treeData = [response];
      });
    },
    handleOk() {
      switch (this.modalTitle) {
        case "创建文件":
          store.state.stateList.push("命令：创建文件" + this.value);
          invoke("mk_file", {
            filename: this.value,
          }).then((response) => {
            store.state.stateList.push("响应：" + response);
            this.getData();
          });
          break;
        default:
          store.state.stateList.push("命令：创建文件夹" + this.value);
          invoke("mk_dir", {
            path: this.value,
          }).then((response) => {
            store.state.stateList.push("响应：" + response);
            this.getData();
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

.selected {
  background-color: #1890ff;
}
.ant-table-cell-row-hover {
  background-color: white !important;
}
</style>
