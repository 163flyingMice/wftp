<template>
  <a-button type="text" size="small" @click="changeModelVisible">
    <template #icon>
      <menu-fold-outlined />
    </template>
  </a-button>
  <a-dropdown placement="bottom" trigger="click" overlayClassName="wftp-server">
    <a-button type="text" size="small" style="border-right: 1px solid black">
      <template #icon>
        <caret-down-outlined />
      </template>
    </a-button>
    <template #overlay>
      <a-menu>
        <a-menu-item v-for="(placement, index) in wftpServer" :key="index" @click="connect(index)">
          {{ placement.Name }}
        </a-menu-item>
      </a-menu>
    </template>
  </a-dropdown>

  <a-button style="margin-left: 6px" size="small" type="default" :onclick="changeStateList"
    :class="{ 'selected-button': stateListComponent }">
    <template #icon>
      <PicCenterOutlined />
    </template>
  </a-button>
  <a-button size="small" type="default" :onclick="changeLocalSite" :class="{ 'selected-button': localSiteComponent }">
    <template #icon>
      <PicLeftOutlined />
    </template>
  </a-button>
  <a-button size="small" type="default" :onclick="changeRemoteSite" :class="{ 'selected-button': remoteSiteComponent }">
    <template #icon>
      <PicRightOutlined />
    </template>
  </a-button>
  <a-button size="small" type="default" :onclick="changeTransfeList"
    :class="{ 'selected-button': transfeListComponent }">
    <template #icon>
      <swap-outlined />
    </template>
  </a-button>
  <div class="line"></div>
  <a-button style="margin-left: 6px" :disabled="noConnect" size="small" type="default" :onclick="refreshRemote">
    <template #icon>
      <redo-outlined />
    </template>
  </a-button>
</template>
<script>
import store from "@/store";
import {
  RedoOutlined,
  SwapOutlined,
  MenuFoldOutlined,
  PicLeftOutlined,
  PicRightOutlined,
  PicCenterOutlined,
  CaretDownOutlined,
} from "@ant-design/icons-vue";
export default {
  props: {
    refreshRemote: Function,
    changeModelVisible: Function,
  },
  components: {
    SwapOutlined,
    RedoOutlined,
    MenuFoldOutlined,
    PicLeftOutlined,
    PicRightOutlined,
    PicCenterOutlined,
    CaretDownOutlined,
  },
  computed: {
    wftpServer() {
      return store.state.wftpServer;
    },
    localSiteComponent() {
      return store.state.localSiteComponent;
    },
    remoteSiteComponent() {
      return store.state.remoteSiteComponent;
    },
    stateListComponent() {
      return store.state.stateListComponent;
    },
    noConnect() {
      return !store.state.panes.length;
    },
    transfeListComponent() {
      return store.state.transfeListComponent;
    },
  },
  methods: {
    changeLocalSite() {
      store.state.localSiteComponent = !store.state.localSiteComponent;
    },
    changeRemoteSite() {
      store.state.remoteSiteComponent = !store.state.remoteSiteComponent;
    },
    changeStateList() {
      store.state.stateListComponent = !store.state.stateListComponent;
    },
    changeTransfeList() {
      store.state.transfeListComponent = !store.state.transfeListComponent;
    },
    connect(index) {
      let key = store.state.panes.length != 0 ? store.state.panes.length + 1 : 1
      store.state.panes.push({
        title: this.wftpServer[index].Name,
        key: key,
        data: this.wftpServer[index],
      })
      store.state.listActiveKey = key
    },
  },
};
</script>
<style>
.selected-button {
  color: #40a9ff;
  border-color: #40a9ff;
}

.wftp-server.ant-dropdown-placement-bottom {
  left: 2px !important;
  width: 150px;
}

.line {
  height: 24px;
  margin-left: 6px;
  border-right: 1px solid black;
}
</style>
