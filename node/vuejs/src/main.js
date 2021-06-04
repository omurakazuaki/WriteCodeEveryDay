import { createApp, h } from 'vue'
import index from './pages/index.vue'
import counter from './pages/counter.vue'

const NotFoundComponent = { template: '<p>Page not found</p>' }

const routes = {
  '/': index,
  '/counter': counter
}

const router = {
  data: () => ({
    currentRoute: window.location.pathname
  }),

  computed: {
    CurrentComponent() {
      console.log(this.currentRoute)
      return routes[this.currentRoute] || NotFoundComponent
    }
  },

  render() {
    return h(this.CurrentComponent)
  }
}

createApp(router).mount('#app')
