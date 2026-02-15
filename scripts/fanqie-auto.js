const { chromium } = require('playwright');

const FANQIE_URL = 'https://fanqienovel.com';
const WRITER_URL = 'https://fanqienovel.com/main/writer/';

async function main() {
  const action = process.argv[2] || 'create';
  const projectId = process.argv[3] || '';
  const novelTitle = process.argv[4] || 'Test Novel';
  const genre = process.argv[5] || '仙侠';
  const content = process.argv[6] || '这是测试章节内容';

  console.log(`[Fanqie Auto] Starting action: ${action}`);
  console.log(`[Fanqie Auto] Project: ${projectId}`);
  console.log(`[Fanqie Auto] Title: ${novelTitle}`);

  let browser;
  try {
    // Try using Chrome channel first (uses existing Chrome installation)
    browser = await chromium.launch({
      channel: 'chrome',
      headless: false,
      args: ['--disable-blink-features=AutomationControlled']
    });
    console.log('[Fanqie Auto] Using Chrome');
  } catch (e) {
    console.log('[Fanqie Auto] Chrome not available, trying headless...');
    try {
      browser = await chromium.launch({
        headless: true,
        args: ['--disable-blink-features=AutomationControlled']
      });
    } catch (e2) {
      console.error('[Fanqie Auto] Failed to launch browser:', e2.message);
      process.exit(1);
    }
  }

  const context = await browser.newContext({
    viewport: { width: 1280, height: 720 },
    userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'
  });

  const page = await context.newPage();

  try {
    switch (action) {
      case 'login':
        await login(page);
        break;
      case 'create':
        await createNovel(page, novelTitle, genre);
        break;
      case 'upload':
        await uploadChapter(page, content);
        break;
      case 'submit':
        await submitChapter(page);
        break;
      default:
        console.error(`Unknown action: ${action}`);
    }
  } catch (error) {
    console.error(`[Fanqie Auto] Error:`, error.message);
  } finally {
    console.log('[Fanqie Auto] Done! Closing browser...');
    await browser.close();
  }
}

async function login(page) {
  console.log('[Fanqie Auto] Navigating to writer page...');
  await page.goto(WRITER_URL, { waitUntil: 'networkidle', timeout: 30000 });

  // Check if already logged in
  const loginButton = await page.$('text="登录"');
  if (!loginButton) {
    console.log('[Fanqie Auto] Already logged in!');
    return;
  }

  console.log('[Fanqie Auto] Clicking login button...');
  await loginButton.click();

  await page.waitForTimeout(2000);

  // Wait for login form
  console.log('[Fanqie Auto] Waiting for login form...');

  // Get credentials from environment
  const username = process.env.FANQIE_USERNAME || '';
  const password = process.env.FANQIE_PASSWORD || '';

  if (!username || !password) {
    console.error('[Fanqie Auto] ERROR: FANQIE_USERNAME and FANQIE_PASSWORD not set');
    return;
  }

  // Fill in username
  console.log('[Fanqie Auto] Filling username...');
  await page.fill('input[placeholder*="手机号"], input[name="username"], input[type="text"], input#username', username);
  await page.waitForTimeout(500);

  // Fill in password
  await page.fill('input[type="password"], input#password', password);
  await page.waitForTimeout(500);

  // Click login
  console.log('[Fanqie Auto] Submitting login...');
  await page.click('button[type="submit"], button:has-text("登录")');

  // Wait for navigation
  await page.waitForTimeout(3000);

  // Check if login successful
  const url = page.url();
  if (url.includes('writer') || url.includes('fanqienovel.com/main')) {
    console.log('[Fanqie Auto] ✅ Login successful!');
  } else {
    console.log('[Fanqie Auto] ⚠️ Login may need verification');
    console.log('[Fanqie Auto] Current URL:', url);
  }
}

async function createNovel(page, title, genre) {
  console.log('[Fanqie Auto] Creating novel:', title);

  // Navigate to writer page
  await page.goto(WRITER_URL, { waitUntil: 'networkidle', timeout: 30000 });

  // Click on "创建新书" or similar button
  const createButton = await page.$('text="创建新书", text="新建书籍", text="创建书籍", text="新建"');
  if (createButton) {
    await createButton.click();
    await page.waitForTimeout(2000);
  }

  // Fill in novel info
  console.log('[Fanqie Auto] Filling novel title:', title);

  // Try different selectors for title input
  const titleInput = await page.$('input[placeholder*="书名"], input[name="title"], input#bookName, input[placeholder*="请输入书名"]');
  if (titleInput) {
    await titleInput.fill(title);
  }

  // Fill description
  const descInput = await page.$('textarea[name="description"], textarea[id="description"], textarea[placeholder*="简介"]');
  if (descInput) {
    await descInput.fill('AI generated novel - ' + title);
  }

  // Submit
  console.log('[Fanqie Auto] Submitting novel...');
  const submitButton = await page.$('button[type="submit"], button:has-text("创建"), button:has-text("确定")');
  if (submitButton) {
    await submitButton.click();
    await page.waitForTimeout(3000);
  }

  console.log('[Fanqie Auto] ✅ Novel created:', title);
}

async function uploadChapter(page, content) {
  console.log('[Fanqie Auto] Uploading chapter...');

  // Navigate to chapter management
  await page.goto(WRITER_URL, { waitUntil: 'networkidle', timeout: 30000 });

  // Click on "上传章节" or similar
  const uploadButton = await page.$('text="上传章节", text="新建章节", text="添加章节", text="写章节"');
  if (uploadButton) {
    await uploadButton.click();
    await page.waitForTimeout(2000);
  }

  // Fill chapter title
  const titleInput = await page.$('input[placeholder*="章节名"], input[name="chapterTitle"], input[placeholder*="请输入章节名"]');
  if (titleInput) {
    await titleInput.fill('第1章');
  }

  // Fill chapter content
  const contentTextarea = await page.$('textarea[name="content"], textarea[id="content"], div[contenteditable="true"]');
  if (contentTextarea) {
    await contentTextarea.fill(content);
  }

  // Submit
  console.log('[Fanqie Auto] Submitting chapter...');
  const submitButton = await page.$('button[type="submit"], button:has-text("保存"), button:has-text("发布")');
  if (submitButton) {
    await submitButton.click();
    await page.waitForTimeout(2000);
  }

  console.log('[Fanqie Auto] ✅ Chapter uploaded');
}

async function submitChapter(page) {
  console.log('[Fanqie Auto] Submitting chapter for review...');

  // Click on "提交审核" or similar
  const submitButton = await page.$('text="提交审核", text="发布", text="提交"');
  if (submitButton) {
    await submitButton.click();
    await page.waitForTimeout(2000);
  }

  console.log('[Fanqie Auto] ✅ Chapter submitted for review');
}

main().catch(console.error);
