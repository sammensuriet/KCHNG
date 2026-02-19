#!/usr/bin/env node
/**
 * Quick smoke test to verify build output exists and is valid
 * Catches basic issues like missing build output or corrupted files
 */
const fs = require('fs');
const path = require('path');

const BUILD_DIR = path.join(__dirname, '../build');
const INDEX_HTML = path.join(BUILD_DIR, 'index.html');
const MANIFEST_JSON = path.join(BUILD_DIR, 'manifest.json');
const SERVICE_WORKER = path.join(BUILD_DIR, 'service-worker.js');

console.log('🔍 Running smoke test...');

let errors = 0;

// Check if build directory exists
if (!fs.existsSync(BUILD_DIR)) {
  console.error('❌ Build directory missing:', BUILD_DIR);
  process.exit(1);
}

// Check index.html exists and is valid
if (!fs.existsSync(INDEX_HTML)) {
  console.error('❌ index.html missing');
  errors++;
} else {
  const content = fs.readFileSync(INDEX_HTML, 'utf-8');
  if (!content.includes('<html') || !content.includes('<body')) {
    console.error('❌ index.html appears invalid');
    errors++;
  } else {
    const sizeKB = fs.statSync(INDEX_HTML).size / 1024;
    console.log(`✅ index.html valid (${sizeKB.toFixed(2)}KB)`);
  }
}

// Check manifest.json exists and has required fields
if (!fs.existsSync(MANIFEST_JSON)) {
  console.error('❌ manifest.json missing');
  errors++;
} else {
  try {
    const manifest = JSON.parse(fs.readFileSync(MANIFEST_JSON, 'utf-8'));
    const required = ['name', 'short_name', 'start_url', 'display', 'icons'];
    const missing = required.filter(field => !manifest[field]);
    if (missing.length > 0) {
      console.error('❌ manifest.json missing required fields:', missing.join(', '));
      errors++;
    } else {
      console.log('✅ manifest.json valid');
    }
  } catch (e) {
    console.error('❌ manifest.json is not valid JSON');
    errors++;
  }
}

// Check service worker exists
if (!fs.existsSync(SERVICE_WORKER)) {
  console.error('❌ service-worker.js missing');
  errors++;
} else {
  console.log('✅ service-worker.js present');
}

// Check robots.txt
const robotsPath = path.join(BUILD_DIR, 'robots.txt');
if (!fs.existsSync(robotsPath)) {
  console.error('❌ robots.txt missing');
  errors++;
} else {
  console.log('✅ robots.txt present');
}

// Check sitemap.xml
const sitemapPath = path.join(BUILD_DIR, 'sitemap.xml');
if (!fs.existsSync(sitemapPath)) {
  console.error('❌ sitemap.xml missing');
  errors++;
} else {
  console.log('✅ sitemap.xml present');
}

// Check required icons
const iconsDir = path.join(BUILD_DIR, 'icons');
const requiredIcons = ['icon-192.png', 'icon-512.png'];
for (const icon of requiredIcons) {
  const iconPath = path.join(iconsDir, icon);
  if (!fs.existsSync(iconPath)) {
    console.error(`❌ ${icon} missing`);
    errors++;
  } else {
    console.log(`✅ ${icon} present`);
  }
}

if (errors > 0) {
  console.error(`\n❌ Smoke test failed with ${errors} error(s)`);
  process.exit(1);
}

console.log('\n✅ Smoke test passed!');
