import { createApp } from "vue";
import App from "./App.vue";
import VueSidebarMenu from 'vue-sidebar-menu';
import 'vue-sidebar-menu/dist/vue-sidebar-menu.css';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import './fontawesome';
import router from './router';

createApp(App)
  // Use the router plugin
  .use(router)
  // Use the VueSidebarMenu plugin
  .use(VueSidebarMenu)
  // Register the FontAwesomeIcon as a global component
  .component('font-awesome-icon', FontAwesomeIcon)
  // Mount the app to the DOM element with id "app"
  .mount("#app");
