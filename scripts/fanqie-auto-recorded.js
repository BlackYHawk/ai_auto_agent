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

  console.log(`[Fanqie Auto] Action: ${action}, Title: ${novelTitle}`);

  const username = process.env.FANQIE_USERNAME || '15858217054';
  const password = process.env.FANQIE_PASSWORD || 'qweR123_';

  let browser;
  try {
    browser = await chromium.launch({
      channel: 'chrome',
      headless: false,
      args: ['--disable-blink-features=AutomationControlled']
    });
  } catch (e) {
    console.error('Failed to launch:', e.message);
    process.exit(1);
  }

  const context = await browser.newContext({
    viewport: { width: 1280, height: 720 },
    userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36'
  });

  // Load cookies if exist
  if (fs.existsSync(COOKIE_FILE)) {
    try {
      const cookies = JSON.parse(fs.readFileSync(COOKIE_FILE, 'utf-8'));
      await context.addCookies(cookies);
      console.log('[Fanqie] Loaded cookies');
    } catch (e) {}
  }

  const page = await context.newPage();

  // Track verification state
  let verificationCompleted = false;

  page.on('console', msg => {
    const text = msg.text();
    // Detect slider verification
    if (text.includes('sliderView show')) {
      console.log('[Fanqie] ⚠️ 滑块验证出现！请手动完成验证...');
    }
    if (text.includes('sliderView show resolve') || text.includes('验证成功')) {
      verificationCompleted = true;
      console.log('[Fanqie] ✓ 验证完成');
    }
    if (!text.includes('x-tt-zhal') && !text.includes('AppLog')) {
      // console.log('[Browser]:', text);
    }
  });

  try {
    // Step 1: Go to login page
    await page.goto('https://fanqienovel.com/main/writer/login');
    await page.waitForTimeout(2000);

    // Check if already logged in
    if (page.url().includes('writer/work')) {
      console.log('[Fanqie] Already logged in');
    } else {
      // Click 密码登录
      await page.getByRole('button', { name: '密码登录' }).click();
      await page.waitForTimeout(1000);

      // Enter phone
      await page.getByRole('textbox', { name: '请输入手机号/邮箱' }).fill(username);
      await page.waitForTimeout(500);

      // Enter password
      await page.getByRole('textbox', { name: '请输入密码' }).fill(password);
      await page.waitForTimeout(500);

      // Check agreement checkbox
      try {
        await page.locator('.arco-checkbox-mask').click();
      } catch (e) {}

      // Click login
      console.log('[Fanqie] Clicking login...');
      await page.getByRole('button', { name: '登录' }).click();
      await page.waitForTimeout(2000);

      // ====== 验证码处理 ======
      console.log('[Fanqie] 检查是否需要验证码...');

      // Wait for verification (up to 2 minutes)
      let checkCount = 0;
      const maxChecks = 60; // 2 minutes

      while (checkCount < maxChecks) {
        const currentUrl = page.url();

        // Check if already logged in
        if (currentUrl.includes('writer/work') || currentUrl.includes('writer/index')) {
          console.log('[Fanqie] ✓ 登录成功！');
          break;
        }

        // Check for slider verification elements
        const hasSlider = await page.locator('.geetest_panel, .tcaptcha, [id*="tcaptcha"], .slider-captcha').count() > 0;
        const hasSliderText = await page.getByText('滑动验证').count() > 0 || await page.getByText('请先完成验证').count() > 0;

        if (hasSlider || hasSliderText) {
          console.log('[Fanqie] ⚠️ 需要滑块验证！');
          console.log('[Fanqie] 请手动拖动滑块完成验证...');
          console.log('[Fanqie] 等待验证完成 (按 Ctrl+C 取消)...');

          // Wait for verification to complete
          while (checkCount < maxChecks) {
            await page.waitForTimeout(2000);
            checkCount++;

            const url = page.url();
            if (url.includes('writer/work') || url.includes('writer/index')) {
              console.log('[Fanqie] ✓ 验证成功，已登录！');
              verificationCompleted = true;
              break;
            }

            // Check if slider is gone
            const sliderStillThere = await page.locator('.geetest_panel, .tcaptcha').count() > 0;
            if (!sliderStillThere) {
              console.log('[Fanqie] ✓ 滑块已消失，验证完成');
              verificationCompleted = true;
              break;
            }
          }

          if (verificationCompleted) break;
        }

        await page.waitForTimeout(1000);
        checkCount++;
      }

      if (!verificationCompleted && !page.url().includes('writer/work')) {
        console.log('[Fanqie] 验证码超时，请手动登录后重试');
        return;
      }
    }

    // Step 2: Navigate to writer dashboard
    await page.goto('https://fanqienovel.com/main/writer/');
    await page.waitForTimeout(2000);

    // Handle popup
    try {
      await page.locator('.byte-modal-close-icon').click();
    } catch (e) {}

    // Step 3: Click 创建书本
    console.log('[Fanqie] 点击创建书本...');
    await page.getByText('创建书本').click();
    await page.waitForTimeout(2000);

    // Step 4: Enter novel title
    console.log('[Fanqie] 输入小说标题:', novelTitle);
    await page.getByRole('textbox', { name: '请输入作品名称' }).fill(novelTitle);
    await page.waitForTimeout(500);

    // Step 5: Select genre/tag
    await page.locator('.arco-icon-hover').first().click();
    await page.waitForTimeout(1000);

    const genreMap = {
      '仙侠': '东方仙侠',
      '都市': '都市生活',
      '历史': '历史神话',
      '游戏': '游戏异界',
      '科幻': '科幻未来',
      '玄幻': '东方玄幻'
    };
    const tagText = genreMap[genre] || '东方仙侠';

    await page.getByText(tagText).click();
    await page.waitForTimeout(500);

    // Click confirm
    await page.getByRole('button', { name: '确认' }).click();
    await page.waitForTimeout(1000);

    // Step 6: Enter protagonist name (optional)
    try {
      const protagonistInput = await page.getByRole('textbox', { name: '请输入主角名1' });
      if (await protagonistInput.isVisible({ timeout: 2000 })) {
        await protagonistInput.fill('主角');
      }
    } catch (e) {}

    // Step 7: Click 立即创建
    console.log('[Fanqie] 点击立即创建...');
    await page.getByRole('button', { name: '立即创建' }).click();
    await page.waitForTimeout(3000);

    console.log('[Fanqie] ✓ 小说创建成功！');
    console.log('[Fanqie] 当前URL:', page.url());

    // Save cookies
    const cookies = await context.cookies();
    fs.writeFileSync(COOKIE_FILE, JSON.stringify(cookies, null, 2));
    console.log('[Fanqie] Cookies已保存');

  } catch (error) {
    console.error('[Fanqie] 错误:', error.message);
    await page.screenshot({ path: 'fanqie-error.png' });
  } finally {
    console.log('[Fanqie] 完成！浏览器将在30秒后关闭...');
    await page.waitForTimeout(30000);
    await browser.close();
  }
}

main().catch(console.error);
