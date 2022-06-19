<script setup lang="ts">
import {ref} from "vue";
import {Swiper, SwiperSlide} from 'swiper/vue';
import 'swiper/css';
import 'swiper/css/navigation';
import 'swiper/css/pagination';
import 'swiper/css/scrollbar';
import {invoke} from "@tauri-apps/api";
import {A11y, Navigation, Pagination, Scrollbar} from "swiper";

class ImageInfo {
  title: string | undefined
  start_date: string | undefined
  url: string | undefined
}

interface ResultApi {
  code: string,
  msg: string,
  data: string
}

const modules = [Navigation, Pagination, Scrollbar, A11y];
const BING_DOMAIN = "https://www.bing.com";
const imageList = ref<ImageInfo[]>([])
const curWallpaper = ref<ImageInfo>({url:"",title:"",start_date:""});
invoke<ImageInfo[]>("get_bing_list").then((info: ImageInfo[]) => {
  console.log(info)
  imageList.value = info;
  let image = info.at(0);
  if (image) {
    curWallpaper.value = image;
  }
})

function switchSwiper(swiper: any) {
  curWallpaper.value = imageList.value[swiper.activeIndex];
}

function handlerWallpaper() {
  console.log("cick")
  invoke<ResultApi>("set_wallpaper",{image: curWallpaper.value}).then((rst) => {
    console.log("info",rst)
  })
}
</script>
<template>
  <v-btn @click="handlerWallpaper()" class="bing-pic-btn" size="small">设置壁纸</v-btn>

  <swiper
      :modules="modules"
      :slides-per-view="1"
      :space-between="1"
      :pagination="{clickable: true }"
      @swiper="onSwiper"
      @slideChange="switchSwiper"
  >
    <swiper-slide v-for="image in imageList">
      <div class="bing-pic-box">
        <img class="bing-pic" :src="BING_DOMAIN + image.url" alt={props.title}/>
        <p class="bing-pic-title">{{image.title}}</p>
      </div>
    </swiper-slide>
  </swiper>
</template>

<script lang="ts">
export default {
  name: "ImageSwiper"
}
</script>

<style scoped>


.bing-pic-box {
  width: 90%;
  margin-left: auto;
  margin-right: auto;
}

.bing-pic {
  width: 95%;
  border-radius: 12px;
}

.bing-pic-title {
  position: absolute;
  left: 7%;
  bottom: 10px;
  box-sizing: border-box;
  color: #fff;
  text-shadow: 1px 1px 1px #000;
  line-height: 1;
  border-radius: 0 0 16px 16px;
}

.bing-pic-btn {
  margin: 10px 0 10px 0 !important;
}

</style>