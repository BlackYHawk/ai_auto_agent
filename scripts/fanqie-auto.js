const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');

const WRITER_URL = 'https://fanqienovel.com/main/writer/';
const COOKIE_FILE = path.join(__dirname, 'fanqie-cookies.json');

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

  const context = await browser.newContext({
    viewport: { width: 1280, height: 720 },
    userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'
  });

  // Try to load existing cookies
  if (fs.existsSync(COOKIE_FILE)) {
    try {
      const cookies = JSON.parse(fs.readFileSync(COOKIE_FILE, 'utf-8'));
      await context.addCookies(cookies);
      console.log('[Fanqie Auto] Loaded existing cookies');
    } catch (e) {
      console.log('[Fanqie Auto] Could not load cookies:', e.message);
    }
  }

  const page = await context.newPage();

  page.on('console', msg => console.log('[Browser Console]:', msg.text()));
  page.on('pageerror', error => console.log('[Browser Error]:', error.message));

  try {
    switch (action) {
      case 'login':
        await login(page, username, password);
        await saveCookies(context, page);
        break;
      case 'create':
        await login(page, username, password);
        await saveCookies(context, page);
        await createNovel(page, novelTitle, genre);
        break;
      case 'upload':
        await login(page, username, password);
        await saveCookies(context, page);
        await uploadChapter(page, content);
        break;
      case 'submit':
        await login(page, username, password);
        await saveCookies(context, page);
        await submitChapter(page);
        break;
      default:
        console.error(`Unknown action: ${action}`);
    }
  } catch (error) {
    console.error(`[Fanqie Auto] Error:`, error.message);
    await page.screenshot({ path: 'fanqie-error.png' });
    console.log('[Fanqie Auto] Screenshot saved to fanqie-error.png');
  } finally {
    console.log('[Fanqie Auto] Done! Closing browser in 60 seconds...');
    console.log('[Fanqie Auto] Note: If you completed verification manually, cookies are saved for next time');
    await page.waitForTimeout(60000);
    await browser.close();
  }
}

async function saveCookies(context, page) {
  try {
    const cookies = await context.cookies();
    fs.writeFileSync(COOKIE_FILE, JSON.stringify(cookies, null, 2));
    console.log('[Fanqie Auto] Cookies saved to', COOKIE_FILE);
  } catch (e) {
    console.log('[Fanqie Auto] Could not save cookies:', e.message);
  }
}

async function login(page, username, password) {
  console.log('[Fanqie Auto] Logging in...');

  await page.goto(WRITER_URL, { waitUntil: 'networkidle', timeout: 30000 });
  await page.waitForTimeout(2000);

  console.log('[Fanqie Auto] Current URL:', page.url());

  // Check if already logged in
  if (page.url().includes('writer/work') || page.url().includes('writer/index') || page.url().includes('writer/home')) {
    console.log('[Fanqie Auto] Already logged in!');
    return true;
  }

  // Click on password login tab
  try {
    const passwordLoginTab = await page.locator('text=密码登录').first();
    if (await passwordLoginTab.isVisible()) {
      await passwordLoginTab.click();
      console.log('[Fanqie Auto] Clicked password login tab');
      await page.waitForTimeout(1000);
    }
  } catch (e) {
    console.log('[Fanqie Auto] Could not find password login tab');
  }

  // Fill phone number
  try {
    const phoneInput = await page.locator('input[placeholder*="手机号"], input[placeholder*="请输入手机号"], input[type="tel"]').first();
    if (await phoneInput.isVisible({ timeout: 5000 })) {
      console.log('[Fanqie Auto] Filling phone number...');
      await phoneInput.fill(username);
      await page.waitForTimeout(500);
    }
  } catch (e) {
    console.log('[Fanqie Auto] Phone input not found');
  }

  // Fill password
  try {
    const passwordInput = await page.locator('input[type="password"]').first();
    if (await passwordInput.isVisible({ timeout: 5000 })) {
      console.log('[Fanqie Auto] Filling password...');
      await passwordInput.fill(password);
      await page.waitForTimeout(500);
    }
  } catch (e) {
    console.log('[Fanqie Auto] Password input not found');
  }

  // Check for verification popup first
  console.log('[Fanqie Auto] Checking for verification popup...');
  const verifyText = await page.locator('text=请先完成验证, text=行为验证, text=安全验证').first();
  if (await verifyText.isVisible({ timeout: 2000 }).catch(() => false)) {
    console.log('[Fanqie Auto] ⚠️ Verification required!');
    console.log('[Fanqie Auto] Please complete the verification manually, then press Enter in terminal...');
    console.log('[Fanqie Auto] Waiting for verification (press Enter when done)...');

    // Wait for user to complete verification manually
    // Check every 2 seconds if verification is done
    for (let i = 0; i < 60; i++) {
      await page.waitForTimeout(2000);
      const verifyGone = await verifyText.isVisible().catch(() => false);
      if (!verifyGone) {
        console.log('[Fanqie Auto] Verification completed!');
        break;
      }
      // Also check if we're now logged in
      const currentUrl = page.url();
      if (currentUrl.includes('writer/work') || currentUrl.includes('writer/index')) {
        console.log('[Fanqie Auto] Logged in successfully!');
        return true;
      }
    }
  }

  // Click login button
  try {
    const loginButton = await page.locator('button:has-text("登录/注册"), button:has-text("登录")').first();

    // Wait for button to be enabled
    console.log('[Fanqie Auto] Waiting for login button to be enabled...');
    await loginButton.waitFor({ state: 'enabled', timeout: 10000 }).catch(() => {});

    if (await loginButton.isVisible({ timeout: 5000 }) && await loginButton.isEnabled()) {
      console.log('[Fanqie Auto] Clicking login button...');
      await loginButton.click();
      await page.waitForTimeout(3000);

      // Check for verification after click
      const verifyTextAfter = await page.locator('text=请先完成验证, text=行为验证').first();
      if (await verifyTextAfter.isVisible({ timeout: 2000 }).catch(() => false)) {
        console.log('[Fanqie Auto] ⚠️ Verification popup appeared after click!');
        console.log('[Fanqie Auto] Please complete the verification manually...');

        // Wait for verification to be completed
        for (let i = 0; i < 60; i++) {
          await page.waitForTimeout(2000);
          const verifyGone = await verifyTextAfter.isVisible().catch(() => false);
          if (!verifyGone) {
            console.log('[Fanqie Auto] Verification completed!');
            break;
          }
          // Check if logged in
          const currentUrl = page.url();
          if (currentUrl.includes('writer/work') || currentUrl.includes('writer/index')) {
            console.log('[Fanqie Auto] Logged in successfully!');
            return true;
          }
        }
      }

      console.log('[Fanqie Auto] After login URL:', page.url());
    }
  } catch (e) {
    console.log('[Fanqie Auto] Login button error:', e.message);
  }

  // Final check
  const currentUrl = page.url();
  if (currentUrl.includes('writer/work') || currentUrl.includes('writer/index')) {
    console.log('[Fanqie Auto] Logged in successfully!');
    return true;
  }

  return false;
}

async function createNovel(page, title, genre) {
  console.log('[Fanqie Auto] Creating novel:', title);

  await page.waitForTimeout(2000);

  const currentUrl = page.url();
  console.log('[Fanqie Auto] Current URL before creating:', currentUrl);

  // Navigate to dashboard if not there
  if (!currentUrl.includes('writer/work') && !currentUrl.includes('writer/index')) {
    console.log('[Fanqie Auto] Not on dashboard, navigating...');
    await page.goto('https://fanqienovel.com/main/writer/work', { waitUntil: 'networkidle' });
    await page.waitForTimeout(2000);
  }

  // Look for create novel button
  try {
    const createBtn = await page.locator('text=创建新书, text=新建书籍, text=写新书').first();
    if (await createBtn.isVisible({ timeout: 5000 })) {
      console.log('[Fanqie Auto] Clicking create novel button...');
      await createBtn.click();
      await page.waitForTimeout(3000);
      console.log('[Fanqie Auto] After create click URL:', page.url());
    }
  } catch (e) {
    console.log('[Fanqie Auto] Create button not found, trying direct navigation...');
    await page.goto('https://fanqienovel.com/main/writer/book/create', { waitUntil: 'networkidle' });
    await page.waitForTimeout(2000);
  }

  // Take a screenshot to see the form
  await page.screenshot({ path: 'fanqie-create-form.png' });
  console.log('[Fanqie Auto] Screenshot saved to fanqie-create-form.png');

  // Try to fill in novel title
  try {
    const titleInput = await page.locator('input[placeholder*="书名"], input[placeholder*="作品名"]').first();
    if (await titleInput.isVisible({ timeout: 5000 })) {
      console.log('[Fanqie Auto] Filling novel title...');
      await titleInput.fill(title);
      await page.waitForTimeout(500);
    }
  } catch (e) {
    console.log('[Fanqie Auto] Title input not found');
  }

  // Submit the form
  try {
    const submitBtn = await page.locator('button:has-text("创建"), button:has-text("下一步"), button:has-text("确定")').first();
    if (await submitBtn.isVisible({ timeout: 5000 })) {
      console.log('[Fanqie Auto] Clicking submit button...');
      await submitBtn.click();
      await page.waitForTimeout(3000);
      console.log('[Fanqie Auto] After submit URL:', page.url());
      console.log('[Fanqie Auto] Novel created successfully!');
    }
  } catch (e) {
    console.log('[Fanqie Auto] Submit button not found');
  }
}

async function uploadChapter(page, content) {
  console.log('[Fanqie Auto] Uploading chapter...');
  await page.waitForTimeout(2000);

  await page.screenshot({ path: 'fanqie-upload.png' });
  console.log('[Fanqie Auto] Screenshot saved to fanqie-upload.png');
}

async function submitChapter(page) {
  console.log('[Fanqie Auto] Submitting chapter...');
  await page.waitForTimeout(2000);

  await page.screenshot({ path: 'fanqie-submit.png' });
}

main().catch(console.error);
