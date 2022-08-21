<template>
  <a-button type="default">
    <template #icon>
      <menu-fold-outlined />
    </template>
  </a-button>
  <a-button type="default" :onclick="changeLocalSite" :class="{ 'selected-button': localSiteComponent }">
    <template #icon>
      <PicLeftOutlined />
    </template>
  </a-button>
  <a-button type="default" :onclick="changeRemoteSite" :class="{ 'selected-button': remoteSiteComponent }">
    <template #icon>
      <PicRightOutlined />
    </template>
  </a-button>
  <a-button type="default" :onclick="reloadApp">
    <template #icon>
      <redo-outlined />
    </template>
  </a-button>
  <a-button type="default" :onclick="changeStateList" :class="{ 'selected-button': stateListComponent }">
    <template #icon>
      <PicCenterOutlined />
    </template>
  </a-button>
</template>
<script>
import store from "@/store";
import { RedoOutlined, MenuFoldOutlined, PicLeftOutlined, PicRightOutlined, PicCenterOutlined } from "@ant-design/icons-vue";
export default {
  components: {
    RedoOutlined,
    MenuFoldOutlined,
    PicLeftOutlined,
    PicRightOutlined,
    PicCenterOutlined
  },
  computed: {
    localSiteComponent() {
      return store.state.localSiteComponent
    },
    remoteSiteComponent() {
      return store.state.remoteSiteComponent
    },
    stateListComponent() {
      return store.state.stateListComponent
    }
  },
  methods: {
    reloadApp() {
      window.location.reload();
    },
    changeLocalSite() {
      store.state.localSiteComponent = !store.state.localSiteComponent
    },
    changeRemoteSite() {
      store.state.remoteSiteComponent = !store.state.remoteSiteComponent
    },
    changeStateList() {
      store.state.stateListComponent = !store.state.stateListComponent
    },
  },
};
</script>
<style>
.selected-button {
  color: #40a9ff;
  border-color: #40a9ff;
}
</style>