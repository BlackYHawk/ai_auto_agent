#!/bin/bash
# Fanqie Auto Recorder - 录制你的操作并生成自动化脚本

echo "=== Fanqie Playwright 录制器 ==="
echo ""
echo "将打开番茄作家网，请执行以下操作："
echo "1. 登录账号"
echo "2. 创建新书"
echo "3. 上传章节"
echo ""
echo "操作完成后，关闭浏览器窗口"
echo "脚本会自动生成自动化代码保存到 scripts/fanqie-recorded.js"
echo ""
echo "按回车开始录制..."
read

# 使用 Playwright codegen 录制
npx playwright codegen \
  --browser=chromium \
  --channel=chrome \
  --output=scripts/fanqie-recorded.js \
  "https://fanqienovel.com/main/writer/"

echo ""
echo "录制完成！"
echo "查看生成的脚本: scripts/fanqie-recorded.js"
