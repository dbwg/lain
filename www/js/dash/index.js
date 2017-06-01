import Vue from 'vue';
import VueRouter from 'vue-router';

import App from './App.vue';
import router from './router';

const vm = new Vue({
  el: '#app',
  router,
  render: h => h(App),
});