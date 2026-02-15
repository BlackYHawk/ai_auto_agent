import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('https://fanqienovel.com/main/writer/login');
  await page.locator('video').click();
  await page.getByRole('button', { name: '密码登录' }).click();
  await page.getByRole('textbox', { name: '请输入手机号/邮箱' }).click();
  await page.locator('video').click();
  await page.getByRole('textbox', { name: '请输入手机号/邮箱' }).click();
  await page.getByRole('textbox', { name: '请输入手机号/邮箱' }).fill('15858217054');
  await page.getByRole('textbox', { name: '请输入密码' }).click();
  await page.getByRole('textbox', { name: '请输入密码' }).fill('qweR123_');
  await page.locator('.arco-checkbox-mask').click();
  await page.getByRole('button', { name: '登录' }).click();
  await page.goto('https://fanqienovel.com/main/writer/');
  await page.locator('.byte-modal-close-icon').click();
  await page.getByText('创建书本').click();
  await page.getByRole('textbox', { name: '请输入作品名称' }).click();
  await page.getByRole('textbox', { name: '请输入作品名称' }).fill('tilt le');
  await page.getByRole('textbox', { name: '请输入作品名称' }).press('Enter');
  await page.getByRole('textbox', { name: '请输入作品名称' }).fill('tiltle');
  await page.locator('.arco-icon-hover').first().click();
  await page.locator('div').filter({ hasText: /^请选择作品标签$/ }).nth(5).click();
  await page.getByText('东方仙侠').click();
  await page.getByRole('button', { name: '确认' }).click();
  await page.getByRole('textbox', { name: '请输入主角名1' }).click();
  await page.getByRole('textbox', { name: '请输入50-500' }).click();
  await page.getByRole('button', { name: '立即创建' }).click();
});