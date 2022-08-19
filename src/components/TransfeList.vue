<template>
  <a-tabs v-model:activeKey="activeKey" tab-position="bottom" type="card">
    <a-tab-pane key="1" tab="列队的文件" :border="false">
      <a-table
        :columns="columns"
        :data-source="dataSource"
        :pagination="false"
        :scroll="{ x: 800 }"
      >
      </a-table
    ></a-tab-pane>
    <a-tab-pane key="2" tab="传输失败">
      <a-table
        :columns="columns"
        :data-source="dataSource"
        :pagination="false"
        :scroll="{ x: 800 }"
      >
      </a-table
    ></a-tab-pane>
    <a-tab-pane key="3" tab="成功的传输">
      <a-table
        :columns="columns"
        :data-source="dataSource"
        :pagination="false"
        :scroll="{ x: 800 }"
      >
      </a-table
    ></a-tab-pane>
  </a-tabs>
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
<style>
.ant-tabs-tab {
  padding: 2px 2px !important;
  text-decoration: none;
  font-size: 12px;
}
.ant-tabs-tab.ant-tabs-tab-active .ant-tabs-tab-btn {
  color: black !important;
}

.ant-tabs-tab-btn:hover {
  color: black !important;
}
</style>
