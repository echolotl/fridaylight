import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

// Import Quasar
import { Quasar, Notify } from 'quasar';

// Import Quasar CSS
import 'quasar/src/css/index.sass';

// Import icon libraries
import '@quasar/extras/material-icons/material-icons.css';

// Create the app
const app = createApp(App);

// Use plugins
app.use(router);
app.use(Quasar, {
  plugins: { Notify }, // import Quasar plugins you need
  config: {
    brand: {
    },
    notify: {
      position: 'bottom-right',
      timeout: 3000,
      textColor: 'white'
    }
  },
  extras: [
    'material-icons', // Add other extras if needed
    // Example: 'fontawesome-v6'
  ]
});

// Mount the app
app.mount("#app");