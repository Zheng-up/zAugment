import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";

console.log("开始初始化应用...");

// 添加最基础的错误捕获
window.addEventListener("error", (event) => {
  console.error("运行时错误:", event.error);
  console.error("错误详情:", event.filename, event.lineno, event.colno);
});

window.addEventListener("unhandledrejection", (event) => {
  console.error("未处理的Promise错误:", event.reason);
  event.preventDefault();
});

try {
  const app = createApp(App);

  // 简单的错误处理，只记录不阻断
  app.config.errorHandler = (err, instance, info) => {
    console.error("Vue运行时错误:", err);
    console.error("错误信息:", info);
  };

  app.mount("#app");
  console.log("应用挂载成功");
} catch (error) {
  console.error("应用初始化失败:", error);
}

console.log("应用启动流程完成");
