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
        <div style="display: flex; align-items: center; gap: 16px">
          <div class="update-icon">
            <svg width="32" height="32" viewBox="0 0 24 24" fill="currentColor">
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
        <div class="release-notes-container">
          <div class="release-notes" v-html="formattedReleaseNotes"></div>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="modal-actions">
        <button @click="closeUpdateModal" class="btn secondary">取消</button>
        <button @click="goToGitHubRelease" class="btn primary">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"
            />
          </svg>
          前往下载
        </button>
      </div>
    </template>
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
  computed: {
    formattedReleaseNotes() {
      if (!this.releaseNotes) return "";

      // 只处理换行，保持简单
      return this.releaseNotes.replace(/\n/g, "<br>");
    },
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
          let errorMessage = "检查更新失败，请稍后重试";

          // 根据错误类型提供更具体的错误信息
          if (error && typeof error === "string") {
            if (error.includes("速率限制") || error.includes("rate limit")) {
              errorMessage = "GitHub API 访问频率过高，请稍后重试";
            } else if (error.includes("网络")) {
              errorMessage = "网络连接失败，请检查网络连接";
            } else if (error.includes("timeout")) {
              errorMessage = "请求超时，请检查网络连接";
            }
          }

          this.$emit("show-status", errorMessage, "error");
        }
      } finally {
        this.isChecking = false;
      }
    },

    async goToGitHubRelease() {
      try {
        // 使用 Tauri API 打开外部链接
        const releaseUrl = `https://github.com/Zheng-up/zAugment/releases/tag/v${this.latestVersion}`;
        await invoke("open_url", { url: releaseUrl });
        this.closeUpdateModal();
        this.$emit("show-status", "已打开 GitHub Release 页面", "success");
      } catch (error) {
        console.error("打开 GitHub Release 页面失败:", error);
        this.$emit("show-status", "打开 GitHub Release 页面失败", "error");
      }
    },

    closeUpdateModal() {
      this.showUpdateModal = false;
    },

    // 供外部调用的手动检查更新方法
    async manualCheckUpdate() {
      return await this.checkForUpdates(true);
    },
  },
};
</script>

<style scoped>
/* 针对更新弹窗的特殊样式覆盖 */
:deep(.modal-body) {
  overflow: hidden !important;
  padding: 24px !important;
  display: flex !important;
  flex-direction: column !important;
  height: auto !important;
  max-height: none !important;
}

.update-modal-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.update-header {
  gap: 16px;
}

.update-icon {
  color: #4caf50;
  flex-shrink: 0;
}

.update-info h3 {
  margin: 0 0 4px 0;
  font-size: 18px;
  font-weight: 600;
  color: #1e293b;
}

.current-version {
  margin: 0;
  font-size: 14px;
  color: #64748b;
}

.update-details {
  background: rgba(34, 197, 94, 0.05);
  border: 1px solid rgba(34, 197, 94, 0.1);
  border-radius: 12px;
  padding: 16px;
  display: flex;
  flex-direction: column;
}

.update-details h4 {
  margin: 0 0 12px 0;
  font-size: 16px;
  font-weight: 600;
  color: #1e293b;
}

.release-notes-container {
  background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  padding: 4px 16px 16px;
  margin: 8px 0;
}

/* 自定义滚动条样式 */
.release-notes-container::-webkit-scrollbar {
  width: 6px;
}

.release-notes-container::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.05);
  border-radius: 3px;
}

.release-notes-container::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 3px;
}

.release-notes-container::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

.release-notes {
  margin-top: 10px;
  font-size: 14px;
  line-height: 1.6;
  color: #555;
}

.release-notes h1,
.release-notes h2,
.release-notes h3,
.release-notes h4,
.release-notes h5 {
  margin: 16px 0 8px 0;
  color: #1e293b;
  font-weight: 600;
}

.release-notes h1 {
  font-size: 20px;
}
.release-notes h2 {
  font-size: 18px;
}
.release-notes h3 {
  font-size: 16px;
}
.release-notes h4 {
  font-size: 15px;
}
.release-notes h5 {
  font-size: 14px;
}

.release-notes p {
  margin: 8px 0;
  line-height: 1.6;
}

.release-notes ul,
.release-notes ol {
  margin: 8px 0;
  padding-left: 20px;
}

.release-notes li {
  margin: 4px 0;
  line-height: 1.5;
}

.release-notes code {
  background: rgba(0, 0, 0, 0.05);
  padding: 2px 6px;
  border-radius: 4px;
  font-family: "Consolas", "Monaco", monospace;
  font-size: 13px;
}

.release-notes pre {
  background: rgba(0, 0, 0, 0.05);
  padding: 12px;
  border-radius: 6px;
  overflow-x: auto;
  margin: 12px 0;
}

.release-notes pre code {
  background: none;
  padding: 0;
}

.release-notes table {
  width: 100%;
  border-collapse: collapse;
  margin: 12px 0;
}

.release-notes th,
.release-notes td {
  padding: 8px 12px;
  text-align: left;
  border: 1px solid #ddd;
}

.release-notes th {
  background: #f5f5f5;
  font-weight: 600;
}

.release-notes blockquote {
  margin: 12px 0;
  padding: 12px 16px;
  background: #f9f9f9;
  border-left: 4px solid #ddd;
  color: #666;
}

/* 按钮样式 - 统一为检查更新按钮样式 */
.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: inline-flex;
  align-items: center;
  gap: 8px;
  position: relative;
  overflow: hidden;
}

.btn::before {
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

.btn:hover::before {
  left: 100%;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn.secondary {
  background: #f3f4f6;
  color: #6b7280;
}

.btn.secondary:hover:not(:disabled) {
  background: #e5e7eb;
  transform: translateY(-1px);
}

.btn.primary {
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
  color: white;
  box-shadow: 0 4px 14px rgba(99, 102, 241, 0.25);
}

.btn.primary:hover:not(:disabled) {
  background: linear-gradient(135deg, #4f46e5 0%, #4338ca 100%);
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(99, 102, 241, 0.35);
}
</style>
display: flex; align-items: center; gap: 16px; background:
linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%); border: 1px solid rgba(34,
197, 94, 0.2); border-radius: 16px; padding: 20px; margin-bottom: 20px;
position: relative; overflow: hidden; } .update-header::before { content: "";
position: absolute; top: 0; left: 0; right: 0; height: 3px; background:
linear-gradient(90deg, #22c55e, #16a34a, #15803d); } .update-icon { color:
#22c55e; flex-shrink: 0; background: rgba(34, 197, 94, 0.1); border-radius:
12px; padding: 8px; display: flex; align-items: center; justify-content: center;
} .update-info { flex: 1; } .update-info h3 { margin: 0 0 6px 0; font-size:
20px; font-weight: 700; color: #1e293b; background: linear-gradient(135deg,
#1e293b, #475569); -webkit-background-clip: text; -webkit-text-fill-color:
transparent; background-clip: text; } .current-version { margin: 0; font-size:
14px; color: #64748b; font-weight: 500; } .update-badge { background:
linear-gradient(135deg, #22c55e, #16a34a); color: white; padding: 6px 12px;
border-radius: 20px; font-size: 12px; font-weight: 600; text-transform:
uppercase; letter-spacing: 0.5px; box-shadow: 0 2px 8px rgba(34, 197, 94, 0.3);
flex-shrink: 0; } .badge-text { display: flex; align-items: center; gap: 4px; }
.badge-text::before { content: "✨"; font-size: 10px; } .update-details {
background: #ffffff; border: 1px solid rgba(0, 0, 0, 0.08); border-radius: 16px;
box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08); overflow: hidden; }
.update-details-header { background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9
100%); border-bottom: 1px solid rgba(0, 0, 0, 0.06); padding: 16px 20px;
position: sticky; top: 0; z-index: 1; } .update-details-header h4 { margin: 0;
font-size: 16px; font-weight: 600; color: #334155; display: flex; align-items:
center; gap: 8px; } .release-notes { padding: 0 20px 20px 20px; font-size: 14px;
line-height: 1.7; color: #374151; } .release-notes h2 { margin: 0 0 16px 0;
font-size: 18px; font-weight: 700; color: #1e293b; border-bottom: 2px solid
#e2e8f0; padding-bottom: 8px; } .release-notes h3 { margin: 20px 0 12px 0;
font-size: 16px; font-weight: 600; color: #334155; display: flex; align-items:
center; gap: 8px; } .release-notes h4 { margin: 16px 0 8px 0; font-size: 15px;
font-weight: 600; color: #475569; } .release-notes h5 { margin: 12px 0 6px 0;
font-size: 14px; font-weight: 600; color: #64748b; } .release-notes h5 {
font-size: 14px; } .release-notes ul { margin: 12px 0; padding-left: 24px; }
.release-notes li { margin: 6px 0; list-style-type: none; position: relative;
line-height: 1.6; } .release-notes li::before { content: "•"; color: #22c55e;
font-weight: bold; position: absolute; left: -16px; } .release-notes strong {
color: #1e293b; font-weight: 600; } .release-notes code { background:
linear-gradient(135deg, #f1f5f9, #e2e8f0); color: #1e293b; padding: 3px 8px;
border-radius: 6px; font-family: "JetBrains Mono", "Consolas", "Monaco",
monospace; font-size: 13px; font-weight: 500; border: 1px solid rgba(0, 0, 0,
0.08); } .release-notes table { width: 100%; border-collapse: collapse; margin:
16px 0; background: #ffffff; border-radius: 8px; overflow: hidden; box-shadow: 0
2px 8px rgba(0, 0, 0, 0.06); } .release-notes th, .release-notes td { padding:
12px 16px; text-align: left; border-bottom: 1px solid #e2e8f0; } .release-notes
th { background: linear-gradient(135deg, #f8fafc, #f1f5f9); font-weight: 600;
color: #334155; font-size: 13px; } .release-notes td { color: #475569; }
.release-notes tr:last-child td { border-bottom: none; } .release-notes em {
color: #64748b; font-style: italic; } .release-notes .emoji-line { margin: 8px
0; padding: 4px 0; font-weight: 500; color: #1e293b; } .release-notes p {
margin: 8px 0; line-height: 1.6; } /* 按钮样式 - 参考添加账号弹窗 */
.modal-actions { display: flex; justify-content: flex-end; gap: 12px; } .btn {
padding: 10px 20px; border: none; border-radius: 8px; cursor: pointer;
font-size: 14px; font-weight: 500; transition: all 0.2s ease; display:
inline-flex; align-items: center; gap: 8px; } .btn:disabled { opacity: 0.5;
cursor: not-allowed; } .btn.secondary { background: #f3f4f6; color: #6b7280; }
.btn.secondary:hover { background: #e5e7eb; color: #374151; } .btn.primary {
background: linear-gradient(135deg, #3b82f6, #1d4ed8); color: white; }
.btn.primary:hover { background: linear-gradient(135deg, #1d4ed8, #1e40af);
transform: translateY(-1px); box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3); }
