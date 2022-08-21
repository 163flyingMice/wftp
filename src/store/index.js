import {
    createStore
} from 'vuex'

export default createStore({
    /**
     * 集中存储组件的状态
     */
    state: {
        stateList: [],
        connected: false,
        localSiteComponent: true,
        remoteSiteComponent: true,
        stateListComponent: true,
    },
    /**
     * 改变数据方法的集合
     */
    mutations: {},
    /**
     * 改变mutations，不直接变更状态
     */
    actions: {},
    /**
     * 可以将store进行功能拆分，分割成不同的模块
     */
    modules: {}
})