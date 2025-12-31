#!/usr/bin/env python3
"""Test wallet connection UI with Playwright"""

from playwright.sync_api import sync_playwright, Page
import time

def test_wallet_connection():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # Navigate to the app
        page.goto('http://localhost:5174')
        page.wait_for_load_state('networkidle')

        # Take initial screenshot
        page.screenshot(path='/tmp/kchng_initial.png', full_page=True)
        print("✓ Initial page loaded")

        # Find and verify Connect Wallet button
        connect_btn = page.locator('button.btn-connect')
        if connect_btn.count() > 0:
            print("✓ Connect Wallet button found")
        else:
            print("✗ Connect Wallet button NOT found")
            page.screenshot(path='/tmp/kchng_no_button.png')
            browser.close()
            return

        # Click the button
        connect_btn.click()
        time.sleep(0.5)  # Brief wait for state update
        page.wait_for_load_state('networkidle')

        # Check for error message
        error_msg = page.locator('.error-message')
        if error_msg.count() > 0:
            error_text = error_msg.inner_text()
            print(f"✓ Error message displayed: {error_text[:50]}...")

            # Test dismiss button
            dismiss_btn = page.locator('.error-dismiss')
            if dismiss_btn.count() > 0:
                dismiss_btn.click()
                time.sleep(0.3)
                page.wait_for_load_state('networkidle')

                # Verify error is gone
                error_after = page.locator('.error-message')
                if error_after.count() == 0:
                    print("✓ Error dismiss button works")
                else:
                    print("✗ Error still visible after dismiss")
            else:
                print("✗ Dismiss button not found")
        else:
            print("✗ No error message shown after clicking")
            page.screenshot(path='/tmp/kchng_no_error.png')

        # Final screenshot
        page.screenshot(path='/tmp/kchng_final.png', full_page=True)

        browser.close()
        print("\nTest complete!")

if __name__ == '__main__':
    test_wallet_connection()
