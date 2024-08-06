import { createRouter, createWebHashHistory, RouteRecordRaw, RouteLocationNormalized } from 'vue-router';

// Define a custom interface for route metadata
interface RouteMeta {
  // Optional key function that returns a string based on the route
  key?: (route: RouteLocationNormalized) => string;
}

// Define the routes array with custom metadata
const routes: Array<RouteRecordRaw & { meta?: RouteMeta }> = [
  {
    // Dynamic route for INI file viewer
    path: '/:fileName',
    name: 'IniFileViewer',
    // Lazy-load the IniFileViewer component
    component: () => import('./components/IniFileViewer.vue'),
    // Pass the fileName from the route params as a prop to the component
    props: (route: { params: { fileName: string; }; }) => ({
      fileName: route.params.fileName
    }),
    // Define custom metadata for the route
    meta: {
      // Generate a unique key based on the fileName
      key: (route) => `${route.params.fileName as string}`
    }
  },
  {
    // Static route for the Help page
    path: '/Help',
    name: 'Help',
    // Lazy-load the Help component
    component: () => import('./pages/Help.vue'),
  },
  {
    // Root route for the Home page
    path: '/',
    name: '',
    // Lazy-load the Home component
    component: () => import('./pages/Help.vue'),
  },
];

// Create the router instance
const router = createRouter({
  // Use hash mode for routing (e.g., /#/route instead of /route)
  history: createWebHashHistory(),
  // Pass the defined routes to the router
  routes
});

// Export the router to be used in the main app
export default router;
