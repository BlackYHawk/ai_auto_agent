const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');

const WRITER_URL = 'https://fanqienovel.com/main/writer/';
const COOKIE_FILE = path.join(__dirname, 'fanqie-cookies.json');
const TRACE_FILE = path.join(__dirname, 'fanqie-trace.zip');
const VIDEO_DIR = path.join(__dirname, 'videos');

async function main() {
  const action = process.argv[2] || 'create';
  const projectId = process.argv[3] || '';
  const novelTitle = process.argv[4] || 'Test Novel';
  const genre = process.argv[5] || '仙侠';
  const content = process.argv[6] || '';

  console.log(`[Fanqie Auto] Starting action: ${action}`);
  console.log(`[Fanqie Auto] Title: ${novelTitle}, Genre: ${genre}`);

  const username = process.env.FANQIE_USERNAME || '';
  const password = process.env.FANQIE_PASSWORD || '';

  let browser;
  try {
    browser = await chromium.launch({
      channel: 'chrome',
      headless: false,
      args: ['--disable-blink-features=AutomationControlled']
    });
    console.log('[Fanqie Auto] Using Chrome');
  } catch (e) {
    console.error('[Fanqie Auto] Failed to launch browser:', e.message);
    process.exit(1);
  }

  // 创建录屏目录
  if (!fs.existsSync(VIDEO_DIR)) {
    fs.mkdirSync(VIDEO_DIR, { recursive: true });
  }

  const contextOptions = {
    viewport: { width: 1280, height: 720 },
    userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
    // 录制视频
    recordVideo: {
      dir: VIDEO_DIR,
      size: { width: 1280, height: 720 }
    }
  };

  // 尝试加载已有cookies
  if (fs.existsSync(COOKIE_FILE)) {
    try {
      const cookies = JSON.parse(fs.readFileSync(COOKIE_FILE, 'utf-8'));
      contextOptions.cookies = cookies;
      console.log('[Fanqie Auto] Loaded existing cookies');
    } catch (e) {
      console.log('[Fanqie Auto] Could not load cookies');
    }
  }

  const context = await browser.newContext(contextOptions);

  // 开启追踪（用于调试回放）
  await context.tracing.start({
    screenshots: true,
    snapshots: true
  });

  const page = await context.newPage();

  page.on('console', msg => {
    if (!msg.text().includes('x-tt-zhal')) {
      console.log('[Browser]:', msg.text());
    }
  });
  page.on('pageerror', error => console.log('[Browser Error]:', error.message));

  try {
    switch (action) {
      case 'login':
        await login(page, username, password);
        await saveCookies(context);
        break;
      case 'create':
        await login(page, username, password);
        await saveCookies(context);
        await createNovel(page, novelTitle, genre);
        break;
      case 'upload':
        await login(page, username, password);
        await saveCookies(context);
        await uploadChapter(page, content);
        break;
      case 'submit':
        await login(page, username, password);
        await saveCookies(context);
        await submitChapter(page);
        break;
      default:
        console.error(`Unknown action: ${action}`);
    }
  } catch (error) {
    console.error(`[Fanqie Auto] Error:`, error.message);
  } finally {
    // 保存追踪文件
    const tracePath = path.join(__dirname, `fanqie-${action}-trace.zip`);
    await context.tracing.stop({ path: tracePath });
    console.log(`[Fanqie Auto] Trace saved to ${tracePath}`);

    // 保存视频
    await context.close();
    console.log(`[Fanqie Auto] Video saved to ${VIDEO_DIR}`);

    console.log('[Fanqie Auto] Done! Browser will close in 30 seconds...');
    await new Promise(r => setTimeout(r, 30000));
    await browser.close();
  }
}

async function saveCookies(context) {
  try {
    const cookies = await context.cookies();
    fs.writeFileSync(COOKIE_FILE, JSON.stringify(cookies, null, 2));
    console.log('[Fanqie Auto] Cookies saved');
  } catch (e) {
    console.log('[Fanqie Auto] Could not save cookies');
  }
}

async function login(page, username, password) {
  console.log('[Fanqie Auto] Logging in...');
  await page.goto(WRITER_URL, { waitUntil: 'networkidle', timeout: 30000 });
  await page.waitForTimeout(2000);

  console.log('[Fanqie Auto] URL:', page.url());

  if (page.url().includes('writer/work') || page.url().includes('writer/index')) {
    console.log('[Fanqie Auto] Already logged in!');
    return true;
  }

  // 点击密码登录
  try {
    const pwdTab = await page.locator('text=密码登录').first();
    if (await pwdTab.isVisible()) {
      await pwdTab.click();
      console.log('[Fanqie Auto] Clicked password login');
      await page.waitForTimeout(1000);
    }
  } catch (e) {}

  // 填写手机号
  try {
    const phone = await page.locator('input[placeholder*="手机号"]').first();
    if (await phone.isVisible({ timeout: 3000 })) {
      await phone.fill(username);
      console.log('[Fanqie Auto] Filled phone');
    }
  } catch (e) {}

  // 填写密码
  try {
    const pwd = await page.locator('input[type="password"]').first();
    if (await pwd.isVisible({ timeout: 3000 })) {
      await pwd.fill(password);
      console.log('[Fanqie Auto] Filled password');
    }
  } catch (e) {}

  // 等待验证
  console.log('[Fanqie Auto] Waiting for verification...');
  console.log('[Fanqie Auto] 请手动完成验证，完成后脚本会自动继续');

  for (let i = 0; i < 60; i++) {
    await page.waitForTimeout(2000);
    const url = page.url();
    if (url.includes('writer/work') || url.includes('writer/index')) {
      console.log('[Fanqie Auto] Logged in!');
      return true;
    }
  }

  return false;
}

async function createNovel(page, title, genre) {
  console.log('[Fanqie Auto] Creating novel:', title);
  await page.waitForTimeout(2000);

  // 截图当前页面
  await page.screenshot({ path: 'fanqie-current.png' });

  // 查找创建按钮
  try {
    const createBtn = await page.locator('text=创建新书').first();
    if (await createBtn.isVisible({ timeout: 5000 })) {
      await createBtn.click();
      console.log('[Fanqie Auto] Clicked create');
      await page.waitForTimeout(3000);
    }
  } catch (e) {
    console.log('[Fanqie Auto] Create button not found');
  }

  await page.screenshot({ path: 'fanqie-create-form.png' });
}

async function uploadChapter(page, content) {
  await page.screenshot({ path: 'fanqie-upload.png' });
}

async function submitChapter(page) {
  await page.screenshot({ path: 'fanqie-submit.png' });
}

main().catch(console.error);
