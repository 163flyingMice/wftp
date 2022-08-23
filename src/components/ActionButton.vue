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
        <a-menu-item
          v-for="(placement, index) in wftpServer"
          :key="index"
          @click="connect"
        >
          {{ placement.Name }}
        </a-menu-item>
      </a-menu>
    </template>
  </a-dropdown>

  <a-button
    style="margin-left: 6px"
    size="small"
    type="default"
    :onclick="changeStateList"
    :class="{ 'selected-button': stateListComponent }"
  >
    <template #icon>
      <PicCenterOutlined />
    </template>
  </a-button>
  <a-button
    size="small"
    type="default"
    :onclick="changeLocalSite"
    :class="{ 'selected-button': localSiteComponent }"
  >
    <template #icon>
      <PicLeftOutlined />
    </template>
  </a-button>
  <a-button
    size="small"
    type="default"
    :onclick="changeRemoteSite"
    :class="{ 'selected-button': remoteSiteComponent }"
  >
    <template #icon>
      <PicRightOutlined />
    </template>
  </a-button>
  <a-button size="small" type="default" :onclick="reloadApp">
    <template #icon>
      <redo-outlined />
    </template>
  </a-button>
</template>
<script>
import store from "@/store";
import {
  RedoOutlined,
  MenuFoldOutlined,
  PicLeftOutlined,
  PicRightOutlined,
  PicCenterOutlined,
  CaretDownOutlined,
} from "@ant-design/icons-vue";
export default {
  props: {
    changeModelVisible: Function,
  },
  components: {
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
  },
  methods: {
    reloadApp() {
      window.location.reload();
    },
    changeLocalSite() {
      store.state.localSiteComponent = !store.state.localSiteComponent;
    },
    changeRemoteSite() {
      store.state.remoteSiteComponent = !store.state.remoteSiteComponent;
    },
    changeStateList() {
      store.state.stateListComponent = !store.state.stateListComponent;
    },
    connect(event) {
      console.log(event);
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
</style>
