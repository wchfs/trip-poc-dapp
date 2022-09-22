<template>
  <ElForm
    @submit.prevent
    ref="addTicketFormRef"
    :model="addTicketForm"
    :rules="rules"
    size="large"
  >
    <div class="flex flex-col">
      <div class="flex flex-row">
        <ElFormItem
          prop="date"
        >
          <ElDatePicker
            v-model="addTicketForm.date"
            type="date"
            placeholder="Pick a day"
          />
        </ElFormItem>
        <ElFormItem
          prop="duration"
        >
          <ElTimeSelect
            v-model="addTicketForm.duration"
            start="00:15"
            end="23:45"
            step="00:15"
            placeholder="Pick a duration"
          />
        </ElFormItem>
      </div>
      <div class="flex flex-col lg:flex-row justify-between">
        <ElFormItem
          prop="plate_number"
        >
          <ElInput
            v-model="addTicketForm.plate_number"
            placeholder="Your plate number"
            :clearable="true"
            :formatter="(value) => formatPlateNumber(value)"
            :parser="(value) => formatPlateNumber(value)"
          />
        </ElFormItem>
        <ElFormItem>
          <ElButton
            type="danger"
            @click="submitForm(addTicketFormRef)"
          >
            Buy ticket (0.5 ETH)
          </ElButton>
        </ElFormItem>
      </div>
    </div>
  </ElForm>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue';
import type { FormInstance, FormRules } from 'element-plus';

const props = defineProps<{
  checkPrice: boolean,
}>();

const formValid = ref(false);

const addTicketFormRef = ref<FormInstance>();
const addTicketForm = reactive({
  date: '',
  duration: '',
  plate_number: '',
});

const rules = reactive<FormRules>({
  date: [
    {
      type: 'date',
      required: true,
      message: 'Please pick a date',
      trigger: 'change',
    },
  ],
  duration: [
    {
      required: true,
      message: 'Please pick a duration',
      trigger: 'change',
    },
  ],
  plate_number: [
    {
      required: true,
      message: 'Please enter your plate number',
      trigger: 'change',
    },
    {
      min: 3,
      max: 10,
      message: 'Length should be 3 to 10',
      trigger: 'blur',
    },
  ],
});

const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return
  await formEl.validate((valid, fields) => {
    if (valid) {
      console.log('submit!')
    } else {
      console.log('error submit!', fields)
    }
  })
}

function formatPlateNumber(value: string) {
  return value.toUpperCase().replace(/[\W_]+/g, '');
}
</script>
