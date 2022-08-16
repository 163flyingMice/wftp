<template>
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
            ><folder-open-outlined v-if="text.kind === 'folder'" />
            <file-outlined v-else />
            <a-input
              class="showInput"
              v-if="text.showInput"
              v-model:value="toName"
              :bordered="false"
              placeholder=""
              @keydown.enter="rename"
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
    if (this.focusInput) {
      document.querySelector(".showInput").focus();
    }
    this.getData();
    this.getTreeData();
  },
  methods: {
    getData() {
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
      }).then((response) => {
        console.log(response);
      });
    },
    getTreeData() {
      invoke("folder_list", {
        path: "/",
      }).then((response) => {
        console.log(response);
        this.treeData = [response];
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
