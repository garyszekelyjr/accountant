import os
import time

from datetime import datetime
from pathlib import Path

from dotenv import load_dotenv
from playwright.sync_api import Download, Page, sync_playwright


load_dotenv()

USERNAME = os.getenv("CAPITAL_ONE_USERNAME", "")
PASSWORD = os.getenv("CAPITAL_ONE_PASSWORD", "")

TRANSACTIONS = Path("transactions")


def login(page: Page):
    page.goto("https://www.capitalone.com/")
    page.locator('input[aria-label="Username"]').fill(USERNAME)
    page.locator('input[aria-label="Password"]').fill(PASSWORD)
    time.sleep(3)
    page.get_by_role("button", name="Sign in").click()
    page.wait_for_url("https://myaccounts.capitalone.com/**")


def download(page: Page):
    page.goto("https://myaccounts.capitalone.com/")
    page.get_by_text("View Account").click()
    page.locator('a[id="moreAccountServicesLink"]').click()
    page.locator('a[id="downloadTransactionsLink"]').click()
    page.locator('[formcontrolname="fileTypeSelector"]').click()
    page.get_by_text("CSV").click()
    page.locator('[formcontrolname="timePeriodSelector"]').click()
    page.get_by_text("By Statement").click()

    # page.locator('[formcontrolname="byStatementSelector"]').click()
    # page.get_by_text("Statment Ending October 18, 2024").click()

    statement = page.locator('[formcontrolname="byStatementSelector"]').inner_text().split("Ending ")[1]

    statement = datetime.strptime(statement, "%B %d, %Y")

    page.get_by_text("Export").click()
    download: Download = page.wait_for_event("download")
    download.save_as(TRANSACTIONS / f"{statement.strftime('%Y-%m-%d')}.csv")


with sync_playwright() as playwright:
    with playwright.chromium.launch(headless=False) as browser:
        with browser.new_page() as page:
            login(page)
            download(page)
