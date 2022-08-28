<template>
  <a-tabs tab-position="bottom" type="card">
    <a-tab-pane key="1" tab="列队的文件" :border="false">
      <a-table :columns="lineFileHeader" :data-source="transfeList" :pagination="false" :scroll="{ x: 800 }"
        class="overflowTable">
      </a-table>
    </a-tab-pane>
    <a-tab-pane key="2" tab="传输失败">
      <a-table :columns="failedTransfeHeader" :data-source="transfeList" :pagination="false" :scroll="{ x: 800 }"
        class="overflowTable">
      </a-table>
    </a-tab-pane>
    <a-tab-pane key="3" tab="成功的传输">
      <a-table :columns="successTransfeHeader" :data-source="transfeList" :pagination="false" :scroll="{ x: 800 }"
        class="overflowTable">
      </a-table>
    </a-tab-pane>
  </a-tabs>
</template>

<script>
import store from '@/store';

export default {
  computed: {
    transfeList: {
      get() {
        return store.state.transfeList
      },
      set(value) {
        store.state.transfeList = value
      }
    }
  },
  data() {
    return {
      successTransfeHeader: [
        {
          title: "服务器/本地文件",
          dataIndex: "localName",
          key: "localName",
          width: 300,
        },
        {
          title: "方向",
          dataIndex: "direction",
          key: "direction",
          width: 100,
        },
        {
          title: "远程文件",
          dataIndex: "remoteName",
          key: "remoteName",
          width: 150,
        },
        {
          title: "大小",
          dataIndex: "size",
          key: "size",
          width: 100,
        },
        {
          title: "优先级",
          dataIndex: "priority",
          key: "priority",
          width: 100,
        },
        {
          title: "时间",
          dataIndex: "time",
          key: "time",
          width: 50,
        },
        {
          title: "原因",
          dataIndex: "reason",
          key: "reason",
          width: 50,
        },
      ],
      failedTransfeHeader: [
        {
          title: "服务器/本地文件",
          dataIndex: "localName",
          key: "localName",
          width: 300,
        },
        {
          title: "方向",
          dataIndex: "direction",
          key: "direction",
          width: 100,
        },
        {
          title: "远程文件",
          dataIndex: "remoteName",
          key: "remoteName",
          width: 150,
        },
        {
          title: "大小",
          dataIndex: "size",
          key: "size",
          width: 100,
        },
        {
          title: "优先级",
          dataIndex: "priority",
          key: "priority",
          width: 100,
        },
        {
          title: "时间",
          dataIndex: "time",
          key: "time",
          width: 50,
        },
      ],
      lineFileHeader: [
        {
          title: "服务器/本地文件",
          dataIndex: "localName",
          key: "localName",
          width: 300,
        },
        {
          title: "方向",
          dataIndex: "direction",
          key: "direction",
          width: 100,
        },
        {
          title: "远程文件",
          dataIndex: "remoteName",
          key: "remoteName",
          width: 150,
        },
        {
          title: "大小",
          dataIndex: "size",
          key: "size",
          width: 100,
        },
        {
          title: "优先级",
          dataIndex: "priority",
          key: "priority",
          width: 100,
        },
        {
          title: "状态",
          dataIndex: "status",
          key: "status",
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

.overflowTable {
  min-height: 180px;
  max-height: 180px;
  overflow-y: auto;
}
</style>
