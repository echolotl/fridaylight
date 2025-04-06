import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

// Import Quasar
import { Quasar } from 'quasar';

// Import Quasar CSS
import 'quasar/src/css/index.sass';

// Import icon libraries
import '@quasar/extras/material-icons/material-icons.css';

// Create the app
const app = createApp(App);

// Use plugins
app.use(router);
app.use(Quasar, {
  plugins: {}, // import Quasar plugins you need
  config: {
    brand: {
      primary: '#1976d2',
      // ... other brand colors
    },
    extras: {
      font: {
        phantomMuffEmpty: 'PhantomMuffEmpty',
        phantomMuffFull: 'PhantomMuffFull'
      }
    }
  }
});

// Mount the app
app.mount("#app");