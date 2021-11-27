import { createApp } from "vue";
import App from "./App.vue";
import mitt from "mitt";

import { library } from "@fortawesome/fontawesome-svg-core";
import { faPlay, faPause } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

library.add(faPlay);
library.add(faPause);

const emitter = mitt();

const app = createApp(App);
app.config.globalProperties.emitter = emitter;
app.config.productionTip = false;
app.component("Fa", FontAwesomeIcon).mount("#app");
