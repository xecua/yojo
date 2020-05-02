import Vue from 'vue';
import App from './App.vue';
import router from './router';
import axios from './axios';
import Buefy from 'buefy';
import 'buefy/dist/buefy.css';

Vue.config.productionTip = false;
Vue.use(axios);
Vue.use(Buefy);

Vue.prototype.$axios = axios;

new Vue({
  router,
  render: h => h(App)
}).$mount('#app');
