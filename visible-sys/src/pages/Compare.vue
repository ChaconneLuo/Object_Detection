<template>
  <div>
    <el-row :gutter="2">
      <el-col :span="12">
        <span>算法: </span>
        <el-select v-model="algorithmValue" placeholder="Select" size="large" style="width: 240px" :teleported="false" popper-class="select_popper">
          <el-option v-for="item in algorithmOptions" :key="item.algorithm" :label="item.algorithm" :value="item.algorithm" />
        </el-select>
      </el-col>
      <el-col :span="12">
        <span>模型: </span>
        <el-select v-model="modelValue" placeholder="Select" size="large" style="width: 240px" :teleported="false" popper-class="select_popper">
          <el-option v-for="item in modelOptions" :key="item" :label="item" :value="item" />
        </el-select>
      </el-col>
    </el-row>
    <el-row :gutter="20">
      <el-col :span="7">
        <el-button type="success" plain @click="openFile">打开图片</el-button>
        <el-button type="success" plain @click="useModel">识别</el-button>
      </el-col>
      <el-col :span="7">
        <div class="flex flex-row w-100% items-center">
          <span>iou:</span>
          <el-slider v-model="iou" :min="0" :max="1" :step="0.01" placement="bottom" />
        </div>
      </el-col>
      <el-col :span="7">
        <div class="flex flex-row w-100% items-center">
          <span>pred:</span>
          <el-slider v-model="pred" :min="0" :max="1" :step="0.01" placement="bottom" />
        </div>
      </el-col>
    </el-row>
    <el-row>
      <el-col :span="24">
        <div class="flex flex-col">
          <span>检测出fire级别火势: {{ checked.filter((item) => item[4] === 'fire' && getLength(item) > 70).length }}</span>
          <span>检测出fire_middle级别火势: {{ checked.filter((item) => item[4] === 'fire' && getLength(item) <= 70).length }}</span>
          <span>检测出fire_small级别火势: {{ checked.filter((item) => item[4] === 'fire_small').length }}</span>
          <span>检测出火苗个数: {{ checked.length }}</span>
        </div>
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
  </div>
</template>
<script setup lang="ts">
import { invoke, convertFileSrc } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/dialog'
import { ref, onMounted, watch } from 'vue'

type Option = {
  algorithm: string
  model: string[]
}

const algorithmOptions = ref<Option[]>([])
const algorithmValue = ref<string>('')
const modelOptions = ref<string[]>([])
const modelValue = ref<string>('')
const imgLink = ref<string>('')
const imgRef = ref()
let imgPath = ''
const checked = ref<any[]>([])
const iou = ref<number>(0.5)
const pred = ref<number>(0.5)

async function getAlgorithms() {
  const response: string = await invoke('get_algorithms')
  algorithmOptions.value = JSON.parse(response)
  algorithmValue.value = algorithmOptions.value[0].algorithm
  modelOptions.value = algorithmOptions.value[0].model
}

const useModel = async () => {
  // handledImg.value = ''
  if (imgLink.value) {
    invoke('detect', { algorithm: algorithmValue.value, model: modelValue.value, img: imgPath, iouValue: iou.value, predValue: pred.value }).then((res) => {
      // handledImg.value = convertFileSrc((res as string).slice(1, -1))
      console.log(JSON.parse(res as string))
      checked.value = JSON.parse(res as string)
      draw_image_and_boxes(imgPath, JSON.parse(res as string))
    })
  }
}

const openFile = () => {
  open({
    multiple: false,
  }).then((file) => {
    imgPath = file as string
    imgLink.value = convertFileSrc(file as string)
    draw_image_and_boxes(imgPath, [])
  })
}
const draw_image_and_boxes = (file: any, boxes: any[]) => {
  const img = new Image()
  img.src = convertFileSrc(file as string)
  img.onload = () => {
    const canvas = document.querySelector('canvas')!
    canvas.width = img.width
    canvas.height = img.height
    const ctx = canvas.getContext('2d')!
    ctx.drawImage(img, 0, 0)
    ctx.strokeStyle = '#00FF00'
    ctx.lineWidth = 3
    ctx.font = '18px serif'
    boxes.forEach(([x1, y1, x2, y2, label, prob]: [number, number, number, number, string]) => {
      if (algorithmValue.value === 'yolov5') {
        // console.log(algorithmValue.value)
        // let w = Math.abs(x2 - x1)
        // let h = Math.abs(y2 - y1)
        // console.log(x1, y1, x2, y2, w, h)
        let img_width = 320,
          img_height = 320
        // x1 = x1 * img_width - (x2 * img_width) / 2
        // y1 = y1 * img_height - (y2 * img_height) / 2
        // x2 = x2 * img_width
        // y2 = y2 * img_height
        x1 = x1 * img_width
        y1 = y1 * img_height
        x2 = x2 * img_width
        y2 = y2 * img_height
        console.log(x1, y1, x2, y2)
      }
      ctx.strokeRect(x1, y1, x2 - x1, y2 - y1)
      ctx.fillStyle = '#00ff00'
      const width = ctx.measureText(label + ' ' + prob).width
      ctx.fillRect(x1, y1, width + 10, 25)
      ctx.fillStyle = '#000000'
      ctx.fillText(label + ' ' + prob, x1, y1 + 18)
    })
  }
}

const getLength = (position: number[]) => {
  return Math.sqrt(Math.pow(position[2] - position[0], 2) + Math.pow(position[3] - position[1], 2))
}

onMounted(() => {
  getAlgorithms()
})

watch(algorithmValue, async (newValue) => {
  modelOptions.value = algorithmOptions.value.find((item) => item.algorithm === newValue)?.model || []
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
