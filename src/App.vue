<script setup lang="ts">
  import { ref, computed } from 'vue';
  import { useRoute, RouteLocationNormalized } from 'vue-router';
  import 'vue-sidebar-menu/dist/vue-sidebar-menu.css';
  import Sidebar from "./components/Sidebar.vue";
  import { showDialog } from './composables/messageDialog';
  import MessageType from './composables/messageDialog';

  // Get the current route
  const route = useRoute();

  // Compute a unique key for the router-view based on the route
  const routeKey = computed(() => {
    if (route.meta.key && typeof route.meta.key === 'function') {
      return route.meta.key(route as RouteLocationNormalized);
    }
    return route.fullPath;
  });

  // Reactive reference to track sidebar collapse state
  const isSidebarCollapsed = ref(false);

  // Function to toggle sidebar collapse state
  const toggleSidebar = (collapsed: boolean) => {
    isSidebarCollapsed.value = collapsed;
  };

  // Define the startup message
  const startupMessage: string = `This application is currently in beta and will likely contain many bugs.

  As a precaution, it is recommended that you make a backup of the '.ini' files located in the 'bin\\scripts' folder where you have MGS2 installed just in case this editor fucks up.

  Please report any bugs to me on Github: https://github.com/mcarigs/mgs2-asi-config-editor/issues`;

  // Show the startup message dialog
  showDialog(startupMessage, 'Beta Warning', MessageType.warning);
</script>

<template>
  <div id="app" :class="{ 'sidebar-collapsed': isSidebarCollapsed }">
    <div class="sidebar">
      <!-- Sidebar component with toggle event handler -->
      <Sidebar @sidebar-toggle="toggleSidebar" />
    </div>

    <div class="main-content">
      <!-- Router view with dynamic key for proper component rerendering -->
      <router-view ::key="routeKey" />
    </div>
  </div>
</template>

<style>
  #app {
    display: flex;
    height: 100vh;
  }

  .sidebar {
    position: fixed;
    height: 100vh;
  }

  .main-content {
    flex-grow: 1;
    transition: margin-left 0.4s;
    margin-left: 350px;
    margin-right: 100px;
    padding: 0px;
    width: calc(100% - 300px);
    box-sizing: border-box;
  }

  /* Adjust main content when sidebar is collapsed */
  .sidebar-collapsed .main-content {
    margin-left: 150px;
    width: calc(100% - 150px);
  }

  /* Responsive design for smaller screens */
  @media (max-width: 768px) {
    .main-content {
      margin-left: 50px;
      width: calc(100% - 50px);
    }
  }

  /* Global scrollbar styles */
  ::-webkit-scrollbar {
    width: 10px;
  }

  ::-webkit-scrollbar-track {
    background: #2f2f2f;
  }

  ::-webkit-scrollbar-thumb {
    background-color: #4286f4ca;
    border-radius: 6px;
  }
</style>
