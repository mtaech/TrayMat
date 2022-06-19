<template>
  <v-app>
    <v-main>
      <ImageSwiper/>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import ImageSwiper from "./components/ImageSwiper.vue";

export default defineComponent({
  name: 'App',
  components: {ImageSwiper},
  data () {
    return {
      //
    }
  },
})
</script>
