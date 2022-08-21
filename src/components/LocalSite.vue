<template>
  <a-modal v-model:visible="visible" :wrap-style="{ overflow: 'hidden' }" @ok="handleOk">
    <a-input v-model:value="value" placeholder="" ref="mkdir" />
    <template #title>
      <div style="width: 100%; cursor: move">{{ modalTitle }}</div>
    </template>
  </a-modal>
  <a-row v-show="state">
    <a-col style="min-width: 100px !important; width: 100%">
      <div>
        <a-input :value="currentPath" addon-before="本地站点：" />
        <a-tree style="
            overflow-y: auto;
            max-height: 100px !important;
            min-height: 100px !important;" :show-icon="true" :tree-data="treeData" @select="changeLocal"
          :defaultExpandAll="true">
          <template #title="dataRef">
            {{ dataRef.title }}
          </template>
          <template #switcherIcon="dataRef">
            <PlusSquareOutlined @click="searchTree(dataRef)" />
          </template>
        </a-tree>
      </div>
    </a-col>
  </a-row>
  <a-row style="min-height: 300px; max-height: 300px; overflow: auto">
    <a-col>
      <a-table class="localTable" v-mouse-menu="options" :columns="columns" :data-source="dataSource"
        :pagination="false" :customRow="customRow" :scroll="{ x: 600 }" style="" :customHeaderRow="customHeaderRow">
        <template #bodyCell="{ column, text }">
          <template v-if="column.dataIndex === 'name'">
            <folder-open-outlined :style="{ color: '#ffe896' }" v-if="text.kind === 'folder'" />
            <file-outlined v-else />
            <a-input class="showInput" v-if="text.showInput" v-model:value="toName" :bordered="false" placeholder=""
              @pressEnter.prevent="renameInput" @focus.prevent="handleFocus"
              style="display: inline-block; width: 80px" />
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
import {
  FileOutlined,
  FolderOpenOutlined,
  ExclamationCircleOutlined,
  PlusSquareOutlined,
} from "@ant-design/icons-vue";
import {
  readDir,
  createDir,
  removeDir,
  renameFile,
  writeBinaryFile,
  removeFile,
  readBinaryFile,
  BaseDirectory,
} from "@tauri-apps/api/fs";
import { MouseMenuDirective } from "@howdyjs/mouse-menu";
import { createVNode, ref } from "vue";
import { Modal } from "ant-design-vue";
import { invoke } from "@tauri-apps/api/tauri";
import store from "@/store";
export default {
  props: {
    state: Boolean,
  },
  directives: {
    MouseMenu: MouseMenuDirective,
  },
  components: {
    FileOutlined,
    FolderOpenOutlined,
    PlusSquareOutlined,
  },
  data() {
    return {
      selectedKey: "",
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
            label: "上传",
            tips: "Upload",
            fn: () => {
              switch (this.selected.kind) {
                case "folder":
                  break;
                default:
                  readBinaryFile(this.selected.path)
                    .then((response) => {
                      invoke("upload", {
                        filename: this.selected.name,
                        content: this.arrayBufferToBase64(response),
                      })
                        .then((response) => {
                          console.log(response);
                        })
                        .catch((err) => {
                          console.log(err);
                        });
                    })
                    .catch((err) => {
                      console.log(err);
                    });
                  break;
              }
            },
            disabled: () => {
              if (store.state.connected) {
                return false
              }
              return true
            },
          },
          {
            label: "添加文件到队列",
            tips: "Add",
            fn: (...args) => console.log("add", args),
            disabled: () => {
              if (store.state.connected) {
                return false
              }
              return true
            },
          },
          {
            label: "进入目录",
            tips: "Enter",
            fn: () => {
              this.currentPath = this.selected.path;
              this.getData();
            },
            disabled: () => {
              switch (this.selected.kind) {
                case "folder":
                  return false
                default:
                  return true
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
            },
          },
          {
            label: "创建文件",
            tips: "Put",
            fn: () => {
              this.visible = true;
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
              Modal.confirm({
                title: "需要确认",
                icon: createVNode(ExclamationCircleOutlined),
                content: "确认要从服务器删除一个文件吗？",
                okText: "确认",
                cancelText: "取消",
                onOk: () => {
                  switch (this.selected.kind) {
                    case "folder":
                      removeDir(this.selected.path).then(() => {
                        this.getData();
                      });
                      break;
                    default:
                      removeFile(this.selected.path).then(() => {
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
                  this.dataSource[key].name.showInput = true;
                }
              }
            },
          },
        ],
      },
      prevPath: "/",
      currentPath: "/",
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
          dataIndex: "length",
          key: "length",
          width: 100,
        },
        {
          title: "文件类型",
          dataIndex: "is_directory",
          key: "is_directory",
          width: 100,
        },
        {
          title: "最近修改",
          dataIndex: "update_at",
          key: "update_at",
          width: 100,
        },
      ],
    };
  },

  mounted() {
    this.getData();
    this.getTreeData();
    this.folderSort();
  },
  methods: {
    arrayBufferToBase64(buffer) {
      var binary = "";
      var bytes = new Uint8Array(buffer);
      var len = bytes.byteLength;
      for (var i = 0; i < len; i++) {
        binary += String.fromCharCode(bytes[i]);
      }
      return window.btoa(binary);
    },
    getData() {
      this.toName = "";
      let option = {};
      let path = JSON.parse(JSON.stringify(this.currentPath));
      if (this.currentPath == "文档") {
        path = "";
        option = { dir: BaseDirectory.Document };
      }
      readDir(path, option).then((response) => {
        let folder_list = [
          {
            name: { name: "..", kind: "folder", path: ".." },
            length: "",
            is_directory: "",
          },
        ];
        response.forEach((elem) => {
          let temp = {};
          temp.name = {};
          temp.name.name = elem.name;
          temp.name.kind = "file";
          temp.name.path = elem.path.replaceAll("\\", "/");
          temp.length = "";
          temp.is_directory = "文件";
          if (elem.children != undefined) {
            temp.name.kind = "folder";
            temp.is_directory = "文件夹";
          }
          //  else {
          //   readBinaryFile(elem.path.replaceAll("\\", "/")).then((response) => {
          //     temp.length = response.length;
          //   }).catch((err) => {
          //     console.log(err)
          //   })
          // }
          folder_list.push(temp);
        });
        this.dataSource = folder_list;
      });
    },
    folderSort() {
      let folder_list = this.dataSource
      for (var i = 0; i < folder_list.length - 1; i++) {
        for (var j = 0; j < folder_list.length - i - 1; j++) {
          if (folder_list[j].is_directory == "文件") {
            var temp = folder_list[j]
            folder_list[j] = folder_list[j + 1]
            folder_list[j + 1] = temp
          }
        }
      }
      this.dataSource = folder_list;
    },
    customHeaderRow() {
      return {
        onClick: (event) => {
          if (event.target.innerText == '文件名') {
            this.folderSort();
          }
        },
      };
    },
    customRow(record) {
      return {
        align: "left",
        onDblclick: () => {
          document
            .querySelectorAll("tr")
            .forEach((elem) => elem.classList.remove("selected"));
          for (const key in this.dataSource) {
            this.dataSource[key].name.showInput = false;
          }
          let path = record.name.path;
          if (path == "..") {
            let temp = this.currentPath.split("/");
            temp.pop();
            path = temp.join("/");
          }
          if (!path) {
            path = "/";
          }
          this.currentPath = path;
          this.getData();
        },
        onContextmenu: (event) => {
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
      let path = "";
      if (this.currentPath != "/") {
        path = this.currentPath + "/";
      } else {
        path = this.currentPath;
      }
      renameFile(path + this.fromName, path + this.toName).then(() => {
        this.getData();
      });
    },
    handleFocus(event) {
      event.target.select();
    },
    getTreeData() {
      let dir = ["C:/", "D:/", "E:/", "F:/"];
      let folder_list = {};
      folder_list.key = "0";
      folder_list.title = "/";
      folder_list.path = "/";
      folder_list.children = new Array(5);
      folder_list.children.push({
        title: "文档",
        key: "0-0",
        path: "文档",
      });
      dir.forEach((elem_dir, index) => {
        let temp = [];
        async function f() {
          await readDir(elem_dir)
            .then((response) => {
              for (let i = 0; i < response.length; i++) {
                if (response[i].children != undefined) {
                  temp.push({
                    title: response[i].name,
                    key: folder_list.key + "-" + (index + 1) + "-" + i,
                    path: response[i].path,
                    children: [{
                      title: "",
                      path: "",
                      key: folder_list.key + "-" + (index + 1) + "-" + i + "-0",
                    }]
                  });
                }
              }
              folder_list.children.push({
                title: elem_dir,
                path: elem_dir,
                key: folder_list.key + "-" + (index + 1),
                children: temp,
              });
            })
            .catch((err) => {
              console.log(err)
            });
        }
        f().then(() => {
          this.treeData = [folder_list];
        })
      });
    },
    handleOk() {
      let path = "";
      if (this.currentPath != "/") {
        path = this.currentPath + "/" + this.value;
      } else {
        path = this.currentPath + this.value;
      }
      switch (this.modalTitle) {
        case "创建文件":
          writeBinaryFile(path, new Uint8Array([])).then(() => {
            this.getData();
          });
          break;
        default:
          createDir(path)
            .then(() => {
              this.getData();
            })
            .catch((err) => {
              console.log(err);
            });
          break;
      }
      this.visible = false;
    },
    changeLocal(key, event) {
      this.currentPath = event.node.dataRef.path;
      this.getData();
    },
    searchTree(elem) {
      if (elem.path != "/") {
        let temp = [];
        readDir(elem.path).then((response) => {
          for (let i = 0; i < 1; i++) {
            temp.push({
              title: response[i].name,
              key: elem.key + "-" + i,
              path: response[i].path.replaceAll("\\", "/"),
              children: [{
                title: "",
                path: "",
                key: elem.key + "-" + i + "-0",
              }]
            });
          }
        })
        console.log(elem)
        elem.dataRef.children = temp;
        elem.loading = true
        elem.expanded = true
      }
    }
  },

  setup() {
    let treeData = ref([]);
    return {
      treeData
    }
  },
};
</script>

<style>
.ant-table-cell {
  padding: 2px 5px !important;
  font-size: 10px !important;
}

.ant-table-thead>tr>th,
.ant-table-tbody>tr>td,
.ant-table tfoot>tr>th,
.ant-table tfoot>tr>td {
  padding: 2px 5px !important;
}

.ant-table-container table>thead>tr:first-child th {
  font-weight: bolder;
}

.selected {
  background-color: #1890ff;
}

.ant-table-cell-row-hover {
  background-color: white !important;
}

.localTable .ant-table-content {
  min-height: 290px !important;
}
</style>
