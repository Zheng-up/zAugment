<template>
  <!-- 更新检测弹窗 -->
  <ModalContainer
    :visible="showUpdateModal"
    title="发现新版本"
    size="medium"
    @close="closeUpdateModal"
  >
    <div class="update-modal-content">
      <div class="update-header">
        <div class="update-icon">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"
            />
          </svg>
        </div>
        <div class="update-info">
          <h3>ZAugment {{ latestVersion }} 可用</h3>
          <p class="current-version">当前版本: {{ currentVersion }}</p>
        </div>
      </div>

      <div class="update-details" v-if="releaseNotes">
        <h4>更新内容:</h4>
        <div class="release-notes" v-html="releaseNotes"></div>
      </div>

      <div class="update-actions">
        <button @click="closeUpdateModal" class="btn secondary">
          稍后提醒
        </button>
        <button @click="downloadUpdate" class="btn primary">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z" />
          </svg>
          立即下载
        </button>
      </div>
    </div>
  </ModalContainer>
</template>

<script>
import ModalContainer from "./ModalContainer.vue";
import { invoke } from "@tauri-apps/api/core";

export default {
  name: "UpdateChecker",
  components: {
    ModalContainer,
  },
  data() {
    return {
      showUpdateModal: false,
      currentVersion: "",
      latestVersion: "",
      releaseNotes: "",
      downloadUrl: "",
      assetName: "",
      isChecking: false,
    };
  },
  async mounted() {
    // 获取当前版本信息
    await this.getCurrentVersion();
    // 应用启动时检查更新
    this.checkForUpdates();
  },
  methods: {
    async getCurrentVersion() {
      try {
        const versionInfo = await invoke("get_app_version");
        this.currentVersion = versionInfo.current_version;
      } catch (error) {
        console.error("获取版本信息失败:", error);
        this.currentVersion = "0.1.5"; // fallback
      }
    },

    async checkForUpdates(showNoUpdateMessage = false) {
      if (this.isChecking) return;

      this.isChecking = true;

      try {
        const updateResult = await invoke("check_for_updates");

        if (updateResult.has_update) {
          this.latestVersion = updateResult.latest_version;
          this.releaseNotes = updateResult.release_notes;
          this.downloadUrl = updateResult.download_url;
          this.assetName = updateResult.asset_name;
          this.showUpdateModal = true;
        } else if (showNoUpdateMessage) {
          this.$emit("show-status", "已是最新版本", "success");
        }
      } catch (error) {
        console.error("检查更新失败:", error);
        if (showNoUpdateMessage) {
          this.$emit("show-status", "检查更新失败，请稍后重试", "error");
        }
      } finally {
        this.isChecking = false;
      }
    },

    downloadUpdate() {
      if (this.downloadUrl) {
        window.open(this.downloadUrl, "_blank");
        this.closeUpdateModal();
        this.$emit(
          "show-status",
          `正在下载 ${this.assetName}，请查看浏览器下载`,
          "success"
        );
      } else {
        // 如果没有直接下载链接，打开 GitHub Release 页面
        window.open(
          `https://github.com/Zheng-up/zAugment/releases/tag/v${this.latestVersion}`,
          "_blank"
        );
        this.closeUpdateModal();
        this.$emit("show-status", "已打开 GitHub Release 页面", "success");
      }
    },

    closeUpdateModal() {
      this.showUpdateModal = false;
    },

    // 供外部调用的手动检查更新方法
    manualCheckUpdate() {
      this.checkForUpdates(true);
    },
  },
};
</script>

<style scoped>
.update-modal-content {
  padding: 20px;
}

.update-header {
  display: flex;
  align-items: center;
  margin-bottom: 20px;
}

.update-icon {
  margin-right: 16px;
  color: #4caf50;
}

.update-info h3 {
  margin: 0 0 4px 0;
  font-size: 18px;
  font-weight: 600;
  color: #333;
}

.current-version {
  margin: 0;
  font-size: 14px;
  color: #666;
}

.update-details {
  margin-bottom: 24px;
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
}

.update-details h4 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
  color: #333;
}

.release-notes {
  font-size: 14px;
  line-height: 1.5;
  color: #555;
}

.update-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
  transition: all 0.2s ease;
}

.btn.secondary {
  background: #f1f3f4;
  color: #5f6368;
}

.btn.secondary:hover {
  background: #e8eaed;
}

.btn.primary {
  background: #1976d2;
  color: white;
}

.btn.primary:hover {
  background: #1565c0;
}
</style>
