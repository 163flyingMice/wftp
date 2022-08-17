<template>
  <a-row>
    <a-col style="min-width: 100px !important; width: 100%">
      <div>
        <a-input :value="currentPath" addon-before="本地站点：" />
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
        :loading="loading"
        :pagination="false"
        :customRow="customRow"
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
              pressEnter="rename"
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
import { FileOutlined, FolderOpenOutlined } from "@ant-design/icons-vue";
import { invoke } from "@tauri-apps/api";
import { readDir } from "@tauri-apps/api/fs";
import { MouseMenuDirective } from "@howdyjs/mouse-menu";
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
      toName: "",
      fromName: "",
      selectedName: "",
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
            tips: "Check",
            fn: (...args) => console.log("check", args),
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
            label: "设置",
            tips: "Setting",
            fn: (...args) => console.log("setting", args),
          },
          {
            line: true,
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
      treeData: [],
      prevPath: "/",
      currentPath: "/",
      dataSource: [],

      columns: [
        {
          title: "文件名",
          dataIndex: "name",
          key: "name",
          width: 200,
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
      ],
    };
  },

  mounted() {
    if (this.focusInput) {
      document.querySelector(".showInput").focus();
    }
    this.getData();
    this.getTreeData();
  },
  methods: {
    getData() {
      readDir("C:/").then((response) => {
        let folder_list = new Array();
        response.forEach((elem) => {
          let temp = {};
          temp.name = {};
          temp.name.name = elem.name;
          temp.name.kind = "file";
          temp.path = elem.path;
          temp.is_directory = "文件";
          if (elem.children != undefined) {
            temp.name.kind = "folder";
            temp.is_directory = "文件夹";
          }
          folder_list.push(temp);
        });
        this.dataSource = folder_list;
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
        onContextmenu: () => {
          for (const key in this.dataSource) {
            this.dataSource[key].name.showInput = false;
          }
          this.fromName = record.name.name;
        },
      };
    },
    rename() {
      invoke("rename_file", {
        fromName: this.fromName,
        toName: this.toName,
      }).then(() => {});
    },
    getTreeData() {
      readDir("C:/").then((response) => {
        let folder_list = {};
        folder_list.title = "/";
        folder_list.key = "0";
        folder_list.children = new Array();
        response.forEach((elem) => {
          let temp = {};
          temp.title = elem.name;
          temp.key = "0-0";
          folder_list.children.push(temp);
        });
        this.treeData = [folder_list];
      });
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
  padding: 0px 5px !important;
  font-size: 10px !important;
}
.ant-table-container table > thead > tr:first-child th {
  font-weight: bolder;
}
</style>
