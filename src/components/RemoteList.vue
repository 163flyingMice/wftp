<template>
  <a-table
    :columns="columns"
    :data-source="dataSource"
    :loading="loading"
    :pagination="false"
    :customRow="customRow"
    :scroll="{ x: 300, y: 400 }"
  >
    <template #bodyCell="{ column, text }">
      <template v-if="column.dataIndex === 'name'"
        ><folder-open-outlined v-if="text.kind === 'folder'" />
        <file-outlined v-else />
        {{ text.name }}</template
      >
    </template>
  </a-table>
</template>
<script>
import { FileOutlined, FolderOpenOutlined } from "@ant-design/icons-vue";
import { invoke } from "@tauri-apps/api";
export default {
  components: {
    FileOutlined,
    FolderOpenOutlined,
  },
  data() {
    return {
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
    this.getData();
  },
  methods: {
    getData() {
      invoke("list", {
        path: this.currentPath,
      }).then((response) => {
        this.dataSource = response;
      });
    },

    customRow(record) {
      return {
        align: "left",
        onclick: () => {
          let prevPath = Object.assign("", this.currentPath);
          this.prevPath += prevPath;
          if (record.name.name != "..") {
            this.currentPath = record.name.name;
            invoke("cwd", {
              path: this.currentPath,
            }).then((response) => {
              console.log(response);
            });
          } else {
            this.currentPath = this.prevPath;
            invoke("prev").then((response) => {
              console.log(response);
            });
          }
          this.getData();
        },
      };
    },
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
