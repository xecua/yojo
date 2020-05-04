import Vue from 'vue';
import App from './App.vue';
import axios from './axios';
import Buefy from 'buefy';
import sanitizeHTML from 'sanitize-html';
import 'buefy/dist/buefy.css';
import '@mdi/font/css/materialdesignicons.css';

Vue.config.productionTip = false;
Vue.use(Buefy);

Vue.prototype.$axios = axios;
sanitizeHTML.defaults.allowedAttributes.blockquote = [{name: 'class', values: ['twitter-tweet']}];
Vue.prototype.$sanitize = sanitizeHTML;

new Vue({
  render: h => h(App)
}).$mount('#app');
