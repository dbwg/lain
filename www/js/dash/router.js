import Vue from 'vue';
import VueRouter from 'vue-router';

import PickServer from './screens/PickServer.vue';
import Server from './screens/Server.vue';

Vue.use(VueRouter);


export default new VueRouter({
  routes: [
    {path: '/', component: PickServer},
    {path: '/server/:id', name: 'server', component: Server, props: true}
  ],
});
