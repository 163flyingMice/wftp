<template>
    <a-modal :visible="labelManagerVisible" title="书签" centered width="800px" @cancel="labelManagerVisible = false"
        :maskClosable="false" cancelText="取消" okText="确定">
        <a-layout>
            <a-layout-sider theme="light" width="350px" style="margin-right: 6px">
                <a-row class="button">书签</a-row>
                <a-row style="height: 300px">
                    <a-tree :tree-data="treeData" :show-icon="true" @select="selectLeaf" :defaultExpandAll="true">
                        <template #title="dataRef">
                            <template v-if="!dataRef.editable && !dataRef.writable">
                                {{ dataRef.Name }}
                            </template>
                            <template v-else>
                                <div>
                                    <a-input @focus="handleFocus" v-model:value="title" size="small"
                                        @pressEnter="handleEnter(dataRef)" />
                                </div>
                            </template>
                        </template>
                        <template #switcherIcon>
                            <PlusSquareOutlined />
                        </template>
                    </a-tree>
                </a-row>
                <a-row style="margin-top: 5px" class="button">
                    <a-col :span="24" align="middle">
                        <a-button type="default">新键书签</a-button>
                        <a-button type="default" @click="rename" :disabled="!hasSelected">重命名</a-button>
                    </a-col>
                </a-row>
                <a-row style="margin-top: 5px" class="button">
                    <a-col :span="24" align="middle">
                        <a-button type="default" @click="del" :disabled="!hasSelected">删除</a-button>
                        <a-button type="default" @click="copy" :disabled="!hasSelected">复制</a-button>
                    </a-col>
                </a-row>
            </a-layout-sider>
            <a-layout>
                <a-tabs v-model:activeKey="activeKey" type="card">
                    <a-tab-pane key="1" tab="书签" :disabled="!hasSelected">
                        <a-row class="button">
                            <a-form layout="inline">
                                <a-form-item label="本地目录">
                                    <a-input v-model:value="localDirectory" />
                                </a-form-item>
                                <a-form-item>
                                    <a-button @click="handleClick">
                                        浏览
                                    </a-button>
                                </a-form-item>
                            </a-form>
                        </a-row>
                        <a-row class="button">
                            <a-form>
                                <a-form-item label="远程目录">
                                    <a-input v-model:value="name" />
                                </a-form-item>
                                <a-form-item>
                                    <a-checkbox name="type">使用同步浏览</a-checkbox>
                                </a-form-item>
                                <a-form-item>
                                    <a-checkbox name="type">目录对比</a-checkbox>
                                </a-form-item>
                            </a-form>
                        </a-row>
                    </a-tab-pane>
                </a-tabs>
            </a-layout>
        </a-layout>
    </a-modal>
</template>

<script>
import store from "@/store/index";
import { PlusSquareOutlined } from "@ant-design/icons-vue";

export default {
    props: {
        refreshWftpServer: Function,
    },
    components: {
        PlusSquareOutlined,
    },
    mounted() {
    },
    methods: {
    },
    computed: {
        labelManagerVisible: {
            get() {
                return store.state.labelManagerVisible
            },
            set(value) {
                store.state.labelManagerVisible = value
            }
        },
    },
    data() {
        return {
        };
    },
};
</script>
