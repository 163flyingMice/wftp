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
        wftpServer: [],
        localSiteComponent: true,
        remoteSiteComponent: true,
        stateListComponent: true,
        connectedName: "",
        modalVisible: false,
        panes: [{
                title: "我的站点",
                key: "1",
                data: {
                    Host: "127.0.0.1",
                    User: "root",
                    Pass: "root",
                    Port: "21",
                    Name: "我的站点",
                },
            },
            {
                title: "我的站点1",
                key: "2",
                data: {
                    Host: "127.0.0.1",
                    User: "root",
                    Pass: "root",
                    Port: "65521",
                    Name: "我的站点1",
                },
            },
        ],
        addLableVisible: true,
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