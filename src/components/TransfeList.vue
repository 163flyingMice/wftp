<template>
  <a-table
    :columns="columns"
    :data-source="dataSource"
    :pagination="false"
    :scroll="{ x: 800 }"
  >
  </a-table>
</template>

<script>
export default {
  data() {
    return {
      dataSource: [],
      columns: [
        {
          title: "服务器/本地文件",
          dataIndex: "name",
          key: "name",
          width: 300,
        },
        {
          title: "方向",
          dataIndex: "size",
          key: "size",
          width: 100,
        },
        {
          title: "远程文件",
          dataIndex: "is_directory",
          key: "is_directory",
          width: 150,
        },
        {
          title: "大小",
          dataIndex: "update_at",
          key: "update_at",
          width: 100,
        },
        {
          title: "优先级",
          dataIndex: "permissions",
          key: "permissions",
          width: 100,
        },
        {
          title: "时间",
          dataIndex: "owner",
          key: "owner",
          width: 50,
        },
      ],
    };
  },
};
</script>
