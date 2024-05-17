<template>
  <div>
    <el-row :gutter="2">
      <el-col :span="12">
        <span>Algorithm: </span>
        <el-select v-model="algorithmValue" placeholder="Select" size="large" style="width: 240px" :teleported="false"
          popper-class="select_popper">
          <el-option v-for="item in algorithmOptions" :key="item.algorithm" :label="item.algorithm"
            :value="item.algorithm" />
        </el-select>
      </el-col>
      <el-col :span="12">
        <span>Model: </span>
        <el-select v-model="modelValue" placeholder="Select" size="large" style="width: 240px" :teleported="false"
          popper-class="select_popper">
          <el-option v-for="item in modelOptions" :key="item" :label="item" :value="item" />
        </el-select>
      </el-col>
    </el-row>
    <el-row>
      <el-col :span="12">
        <el-button type="success" plain @click="openFile">Open Picture</el-button>
        <el-button type="success" plain @click="useModel">Check</el-button>
      </el-col>
    </el-row>
    <el-row>
      <el-col :span="12">
        <img :src="imgLink" class="w-100%" />
      </el-col>
      <el-col :span="12">
        <canvas ref="imgRef" class="w-100%"></canvas>
      </el-col>
    </el-row>
    <el-row>
      <el-col :span="24">
        检测火势个数：{{ checkedCnt }}
      </el-col>
    </el-row>
  </div>
</template>
<script setup lang="ts">
import { invoke, convertFileSrc } from '@tauri-apps/api/tauri';
import { open } from "@tauri-apps/api/dialog"
import { ref, onMounted, watch } from 'vue';

type Option = {
  algorithm: string;
  model: string[];
}

const algorithmOptions = ref<Option[]>([]);
const algorithmValue = ref<string>('');
const modelOptions = ref<string[]>([]);
const modelValue = ref<string>('');
const imgLink = ref<string>('');
const imgRef = ref();
let imgPath = '';
const checkedCnt = ref<number>(0)

async function getAlgorithms() {
  const response: string = await invoke('get_algorithms');
  algorithmOptions.value = JSON.parse(response);
  algorithmValue.value = algorithmOptions.value[0].algorithm;
  modelOptions.value = algorithmOptions.value[0].model;
}

const useModel = async () => {
  // handledImg.value = ''
  if (imgLink.value) {
    invoke('detect',
      { algorithm: algorithmValue.value, model: modelValue.value, img: imgPath }).then((res) => {
        // handledImg.value = convertFileSrc((res as string).slice(1, -1))
        console.log(JSON.parse(res))
        checkedCnt.value = JSON.parse(res)?.length
        draw_image_and_boxes(imgPath, JSON.parse(res))
      })
  }
}

const openFile = () => {
  open({
    multiple: false
  }).then((file) => {
    imgPath = file as string;
    imgLink.value = convertFileSrc(file as string)
    draw_image_and_boxes(imgPath, [])
  })
}
const draw_image_and_boxes = (file, boxes: any[]) => {
  const img = new Image()
  img.src = convertFileSrc(file as string);
  img.onload = () => {
    const canvas = document.querySelector("canvas")!;
    canvas.width = img.width;
    canvas.height = img.height;
    const ctx = canvas.getContext("2d")!;
    ctx.drawImage(img, 0, 0);
    ctx.strokeStyle = "#00FF00";
    ctx.lineWidth = 3;
    ctx.font = "18px serif";
    boxes.forEach(([x1, y1, x2, y2, label]: [number, number, number, number, string]) => {
      ctx.strokeRect(x1, y1, x2 - x1, y2 - y1);
      ctx.fillStyle = "#00ff00";
      const width = ctx.measureText(label).width;
      ctx.fillRect(x1, y1, width + 10, 25);
      ctx.fillStyle = "#000000";
      ctx.fillText(label, x1, y1 + 18);
    });
  }
}

onMounted(() => {
  getAlgorithms();

})

watch(algorithmValue, async (newValue) => {
  modelOptions.value = algorithmOptions.value.find((item) => item.algorithm === newValue)?.model || [];
  modelValue.value = modelOptions.value[0]
})
</script>

<style scoped>
.select_popper {
  font-family: PingFang SC, sans-serif;
}

.el-row {
  margin-bottom: 20px;
}

.el-row:last-child {
  margin-bottom: 0;
}
</style>
