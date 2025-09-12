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

export default {
  name: "UpdateChecker",
  components: {
    ModalContainer,
  },
  data() {
    return {
      showUpdateModal: false,
      currentVersion: "0.1.1",
      latestVersion: "",
      releaseNotes: "",
      downloadUrl: "",
      isChecking: false,
    };
  },
  mounted() {
    // 应用启动时检查更新
    this.checkForUpdates();
  },
  methods: {
    async checkForUpdates(showNoUpdateMessage = false) {
      if (this.isChecking) return;

      this.isChecking = true;

      try {
        const response = await fetch(
          "https://api.github.com/repos/Zheng-up/zAugment/releases/latest"
        );

        if (!response.ok) {
          throw new Error("Failed to fetch release info");
        }

        const release = await response.json();
        const latestVersion = release.tag_name.replace("v", "");

        if (this.isNewerVersion(latestVersion, this.currentVersion)) {
          this.latestVersion = latestVersion;
          this.releaseNotes = this.parseReleaseNotes(release.body);
          this.downloadUrl = this.getDownloadUrl(release.assets);
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

    isNewerVersion(latest, current) {
      const latestParts = latest.split(".").map(Number);
      const currentParts = current.split(".").map(Number);

      for (
        let i = 0;
        i < Math.max(latestParts.length, currentParts.length);
        i++
      ) {
        const latestPart = latestParts[i] || 0;
        const currentPart = currentParts[i] || 0;

        if (latestPart > currentPart) return true;
        if (latestPart < currentPart) return false;
      }

      return false;
    },

    parseReleaseNotes(body) {
      if (!body) return "";

      // 提取主要更新内容，移除过多的技术细节
      const lines = body.split("\n");
      const importantLines = lines.filter((line) => {
        return (
          line.includes("✨") ||
          line.includes("🐛") ||
          line.includes("⚡") ||
          line.includes("🔧") ||
          line.includes("📦") ||
          line.includes("🎯")
        );
      });

      return importantLines.slice(0, 5).join("<br>");
    },

    getDownloadUrl(assets) {
      if (!assets || assets.length === 0) return "";

      // 根据平台选择合适的下载链接
      const platform = this.detectPlatform();

      const platformAssets = {
        windows: assets.find(
          (asset) =>
            asset.name.includes("windows") && asset.name.endsWith(".exe")
        ),
        "macos-intel": assets.find(
          (asset) =>
            asset.name.includes("macos-x64") && asset.name.endsWith(".dmg")
        ),
        "macos-arm": assets.find(
          (asset) =>
            asset.name.includes("macos-arm64") && asset.name.endsWith(".dmg")
        ),
        linux: assets.find(
          (asset) =>
            asset.name.includes("linux") && asset.name.endsWith(".AppImage")
        ),
      };

      const selectedAsset = platformAssets[platform] || assets[0];
      return selectedAsset ? selectedAsset.browser_download_url : "";
    },

    detectPlatform() {
      const userAgent = navigator.userAgent.toLowerCase();

      if (userAgent.includes("win")) return "windows";
      if (userAgent.includes("mac")) {
        // 检测是否为 Apple Silicon
        return navigator.platform === "MacIntel" ? "macos-intel" : "macos-arm";
      }
      if (userAgent.includes("linux")) return "linux";

      return "windows"; // 默认
    },

    downloadUpdate() {
      if (this.downloadUrl) {
        window.open(this.downloadUrl, "_blank");
        this.closeUpdateModal();
        this.$emit("show-status", "下载已开始，请查看浏览器下载", "success");
      } else {
        // 如果没有直接下载链接，打开 GitHub Release 页面
        window.open(
          `https://github.com/Zheng-up/zAugment/releases/tag/v${this.latestVersion}`,
          "_blank"
        );
        this.closeUpdateModal();
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
