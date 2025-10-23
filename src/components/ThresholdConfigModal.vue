<template>
  <ModalContainer
    :visible="visible"
    title="账号状态阈值配置"
    size="medium"
    @close="handleClose"
  >
    <div class="threshold-config-container">
      <!-- 时间阈值配置 -->
      <div class="config-section">
        <div class="section-header">
          <h4 class="section-title">时间阈值</h4>
        </div>
        <div class="input-container">
          <!-- 颜色区域规则说明 -->
          <div class="color-rules">
            <span class="rule-item red">
              0天 &lt; <span class="color-dot"></span> ≤
              {{ localTimeThresholds.warning }}天
            </span>
            <span class="rule-item yellow">
              {{ localTimeThresholds.warning }}天 &lt;
              <span class="color-dot"></span> ≤ {{ localTimeThresholds.safe }}天
            </span>
            <span class="rule-item green">
              {{ localTimeThresholds.safe }}天 &lt;
              <span class="color-dot"></span> ≤ {{ localTimeMax }}天
            </span>
          </div>

          <!-- 输入框区域 -->
          <div class="threshold-inputs">
            <div class="input-group">
              <label>红色上限:</label>
              <input
                type="number"
                min="1"
                :max="localTimeMax - 2"
                v-model.number="localTimeThresholds.warning"
                @input="validateTimeThresholds"
                class="input-threshold"
              />
              <span class="unit">天</span>
            </div>
            <div class="input-group">
              <label>黄色上限:</label>
              <input
                type="number"
                :min="localTimeThresholds.warning + 1"
                :max="localTimeMax - 1"
                v-model.number="localTimeThresholds.safe"
                @input="validateTimeThresholds"
                class="input-threshold"
              />
              <span class="unit">天</span>
            </div>
            <div class="input-group">
              <label>绿色上限:</label>
              <input
                type="number"
                min="3"
                max="365"
                v-model.number="localTimeMax"
                @input="validateTimeMax"
                class="input-threshold"
              />
              <span class="unit">天</span>
            </div>
          </div>
        </div>
      </div>
      <!-- 额度阈值配置 -->
      <div class="config-section">
        <div class="section-header">
          <h4 class="section-title">额度阈值</h4>
        </div>
        <div class="input-container">
          <!-- 颜色区域规则说明 -->
          <div class="color-rules">
            <span class="rule-item red">
              0分 &lt; <span class="color-dot"></span> ≤
              {{ localBalanceThresholds.warning }}分
            </span>
            <span class="rule-item yellow">
              {{ localBalanceThresholds.warning }}分 &lt;
              <span class="color-dot"></span> ≤
              {{ localBalanceThresholds.safe }}分
            </span>
            <span class="rule-item green">
              {{ localBalanceThresholds.safe }}分 &lt;
              <span class="color-dot"></span> ≤ {{ localBalanceMax }}分
            </span>
          </div>

          <!-- 输入框区域 -->
          <div class="threshold-inputs">
            <div class="input-group">
              <label>红色上限:</label>
              <input
                type="number"
                min="1"
                :max="localBalanceMax - 2"
                step="1"
                v-model.number="localBalanceThresholds.warning"
                @input="validateBalanceThresholds"
                class="input-threshold"
              />
              <span class="unit">分</span>
            </div>
            <div class="input-group">
              <label>黄色上限:</label>
              <input
                type="number"
                :min="localBalanceThresholds.warning + 1"
                :max="localBalanceMax - 1"
                step="1"
                v-model.number="localBalanceThresholds.safe"
                @input="validateBalanceThresholds"
                class="input-threshold"
              />
              <span class="unit">分</span>
            </div>
            <div class="input-group">
              <label>绿色上限:</label>
              <input
                type="number"
                min="3"
                max="100000"
                step="1"
                v-model.number="localBalanceMax"
                @input="validateBalanceMax"
                class="input-threshold"
              />
              <span class="unit">分</span>
            </div>
          </div>
        </div>
      </div>
    </div>
    <template #footer>
      <div class="modal-actions">
        <button @click="handleReset" class="btn-reset">恢复默认</button>
        <div class="action-buttons">
          <button @click="handleClose" class="btn-cancel">取消</button>
          <button @click="handleSave" class="btn-save">保存</button>
        </div>
      </div>
    </template>
  </ModalContainer>
</template>
<script setup>
import { ref, watch } from "vue";
import ModalContainer from "./ModalContainer.vue";
const props = defineProps({
  visible: {
    type: Boolean,
    default: false,
  },
  timeThresholds: {
    type: Object,
    default: () => ({ warning: 3, safe: 5 }),
  },
  balanceThresholds: {
    type: Object,
    default: () => ({ warning: 10, safe: 30 }),
  },
  timeMax: {
    type: Number,
    default: 7,
  },
  balanceMax: {
    type: Number,
    default: 50,
  },
});
const emit = defineEmits(["close", "save", "reset"]);
// 本地状态
const localTimeThresholds = ref({ ...props.timeThresholds });
const localBalanceThresholds = ref({ ...props.balanceThresholds });
const localTimeMax = ref(props.timeMax);
const localBalanceMax = ref(props.balanceMax);
// 监听 props 变化
watch(
  () => props.timeThresholds,
  (newVal) => {
    localTimeThresholds.value = { ...newVal };
  },
  { deep: true }
);
watch(
  () => props.balanceThresholds,
  (newVal) => {
    localBalanceThresholds.value = { ...newVal };
  },
  { deep: true }
);
// 监听弹窗打开，重置所有状态
watch(
  () => props.visible,
  (newVal) => {
    if (newVal) {
      // 弹窗打开时，恢复为上次保存的配置
      localTimeThresholds.value = { ...props.timeThresholds };
      localBalanceThresholds.value = { ...props.balanceThresholds };
      // 恢复绿色上限为上次保存的配置
      localTimeMax.value = props.timeMax;
      localBalanceMax.value = props.balanceMax;
    }
  }
);
// 验证时间阈值
const validateTimeThresholds = () => {
  const max = localTimeMax.value;

  // 红色上限（warning）的约束：最小值为1，最大值为 max-2
  if (localTimeThresholds.value.warning < 1) {
    localTimeThresholds.value.warning = 1;
  }
  const warningMaxLimit = max - 2;
  if (localTimeThresholds.value.warning > warningMaxLimit) {
    localTimeThresholds.value.warning = warningMaxLimit;
  }

  // 黄色上限（safe）的约束：最小值为 warning+1，最大值为 max-1
  const safeMinLimit = localTimeThresholds.value.warning + 1;
  const safeMaxLimit = max - 1;

  if (localTimeThresholds.value.safe < safeMinLimit) {
    localTimeThresholds.value.safe = safeMinLimit;
  }
  if (localTimeThresholds.value.safe > safeMaxLimit) {
    localTimeThresholds.value.safe = safeMaxLimit;
  }
};
// 验证额度阈值
const validateBalanceThresholds = () => {
  const max = localBalanceMax.value;

  // 红色上限（warning）的约束：最小值为1，最大值为 max-2
  if (localBalanceThresholds.value.warning < 1) {
    localBalanceThresholds.value.warning = 1;
  }
  const warningMaxLimit = max - 2;
  if (localBalanceThresholds.value.warning > warningMaxLimit) {
    localBalanceThresholds.value.warning = warningMaxLimit;
  }

  // 黄色上限（safe）的约束：最小值为 warning+1，最大值为 max-1
  const safeMinLimit = localBalanceThresholds.value.warning + 1;
  const safeMaxLimit = max - 1;

  if (localBalanceThresholds.value.safe < safeMinLimit) {
    localBalanceThresholds.value.safe = safeMinLimit;
  }
  if (localBalanceThresholds.value.safe > safeMaxLimit) {
    localBalanceThresholds.value.safe = safeMaxLimit;
  }
};
// 验证最大值
const validateTimeMax = () => {
  if (localTimeMax.value < 3) localTimeMax.value = 3;
  if (localTimeMax.value > 365) localTimeMax.value = 365;
  validateTimeThresholds();
};
const validateBalanceMax = () => {
  if (localBalanceMax.value < 3) localBalanceMax.value = 3;
  if (localBalanceMax.value > 100000) localBalanceMax.value = 100000;
  validateBalanceThresholds();
};
// 处理保存
const handleSave = () => {
  emit("save", {
    timeThresholds: { ...localTimeThresholds.value },
    balanceThresholds: { ...localBalanceThresholds.value },
    timeMax: localTimeMax.value,
    balanceMax: localBalanceMax.value,
  });
  emit("close");
};
// 处理关闭
const handleClose = () => {
  // 恢复原始值（恢复为上次保存的配置）
  localTimeThresholds.value = { ...props.timeThresholds };
  localBalanceThresholds.value = { ...props.balanceThresholds };
  // 恢复最大值为上次保存的配置
  localTimeMax.value = props.timeMax;
  localBalanceMax.value = props.balanceMax;
  emit("close");
};
// 恢复默认值
const handleReset = () => {
  localTimeThresholds.value = { warning: 3, safe: 5 };
  localBalanceThresholds.value = { warning: 10, safe: 30 };
  localTimeMax.value = 365;
  localBalanceMax.value = 100000;
  emit("reset");
};
</script>
<style scoped>
.threshold-config-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding-bottom: 30px;
}
.config-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.section-header {
  display: flex;
  flex-direction: column;
  /* gap: 4px; */
}
.section-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #1e293b;
}
.section-description {
  margin: 0;
  font-size: 13px;
  color: #64748b;
}
.input-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.color-rules {
  display: flex;
  flex-direction: row;
  gap: 16px;
  padding: 12px 16px;
  background: rgba(248, 250, 252, 0.8);
  border-radius: 8px;
  flex-wrap: wrap;
}
.rule-item {
  font-size: 13px;
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 8px;
  white-space: nowrap;
  color: #475569;
}
.color-dot {
  width: 12px;
  height: 12px;
  border-radius: 3px;
  flex-shrink: 0;
}
.rule-item.red .color-dot {
  background: #b91c1c;
}
.rule-item.yellow .color-dot {
  background: #a16207;
}
.rule-item.green .color-dot {
  background: #15803d;
}
.threshold-inputs {
  display: flex;
  flex-direction: row;
  gap: 16px;
  flex-wrap: wrap;
}
.input-group {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  min-width: 150px;
}
.input-group label {
  font-size: 13px;
  font-weight: 500;
  color: #64748b;
  white-space: nowrap;
}
.input-threshold {
  flex: 1;
  min-width: 90px;
  padding: 8px 12px;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 14px;
  color: #1e293b;
  background: white;
  transition: border-color 0.15s ease;
}
.input-threshold:focus {
  outline: none;
  border-color: #6366f1;
  box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.1);
}
.input-group .unit {
  font-size: 13px;
  color: #94a3b8;
  white-space: nowrap;
}
.modal-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}
.action-buttons {
  display: flex;
  gap: 12px;
}
.btn-reset,
.btn-cancel,
.btn-save {
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  border: none;
  position: relative;
  overflow: hidden;
}
.btn-reset::before,
.btn-cancel::before,
.btn-save::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.2),
    transparent
  );
  transition: left 0.5s;
}
.btn-reset:hover::before,
.btn-cancel:hover::before,
.btn-save:hover::before {
  left: 100%;
}
.btn-reset {
  background: rgba(248, 250, 252, 0.8);
  color: #64748b;
  border: 1px solid #e2e8f0;
}
.btn-reset:hover {
  background: #f1f5f9;
  color: #475569;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(100, 116, 139, 0.15);
}
.btn-cancel {
  background: rgba(248, 250, 252, 0.8);
  color: #64748b;
  border: 1px solid #e2e8f0;
}
.btn-cancel:hover {
  background: #f1f5f9;
  color: #475569;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(100, 116, 139, 0.15);
}
.btn-save {
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
  box-shadow: 0 4px 14px rgba(99, 102, 241, 0.25);
}
.btn-save:hover {
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(99, 102, 241, 0.35);
}
</style>
