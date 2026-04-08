_Overview_

## the problem

i had a budget spreadsheet ive been using for my own personal budgeting back in 2020 when i studied abroad in germany. recently, my partner wanted to start budgeting so i shared it with her and customized the spreadsheet to fit her needs: revolve around when she gets paid, customize savings/goals, paying off student loans, etc.

however, it was a difficult interface to constantly check google sheets on the web or on mobile. i was considering making and hosting a custom website, but it would just be over the top to interact with a spreadsheet that is bound to change. people dont do things that require friction, even when important

the obvious fix was to push the info to her rather than having her pull it. a daily email with a summary on her balance, spending patterns, and progress twd her saving goals.

## the build

took about 2 days to build (but many many more to optimize, fix bugs, etc.). the stack:

### google sheets api

the budget lives in a google sheet - i connect using `service_account.json` which is provided by google api services: [https://docs.cloud.google.com/iam/docs/keys-create-delete]. we all know i love rust but this is not meant to be perfomative or fast.. so i just used `google-auth-*` and `google-api-python-client` python packages for quick interfacing

### email rendering

used gmail's smtp interface with python's `smtplib` to send. jinja2 for templating so the html structure is separated from data. easy to tweak the layout without touching the code.

```python
# Simplified example of how the email gets constructed
import smtplib
from email.mime.text import MIMEText
from jinja2 import Template

# Render the budget summary template
template = Template(email_html_template)
budget_summary = template.render(
    total_balance=current_balance,
    savings_goal=savings_target,
    progress_percent=(current_savings / savings_target) * 100,
    horoscope=daily_horoscope
)

# Send it
msg = MIMEText(budget_summary, 'html')
smtp_server.send_message(msg)
```

I used **Jinja2 templating** to make the email clean, structured, and easy to update. Want to change the layout? Just modify the template.

### docker for deployment

needed this running on my nas at home. can't just pip install custom deps on a nas, so i containerized it with a slim python image and cron job:

```bash
0 7 * * * docker run --rm [args] my-budget-reminder:latest
```

portable, reproducible, easy.

## the horoscope thing

after a few weeks she said the email was useful but boring. fair. added daily horoscopes as a small thing to make it feel less purely utilitarian.

process: get her astrological sign from her birthday, fetch the daily horoscope, include it in the email.

### the horoscope rabbit hole

the website rendering horoscopes with client-side javascript. simple GET requests gave stale data. had to use playwright with browser emulation to wait for the page to fully render:

```python
from playwright.sync_api import sync_playwright

async def fetch_horoscope_with_rendering():
    async with sync_playwright() as p:
        browser = await p.chromium.launch()
        page = await browser.new_page()
        await page.goto(horoscope_url)
        await page.wait_for_selector('.horoscope-text')
        horoscope = await page.text_content('.horoscope-text')
        await browser.close()
        return horoscope
```

overkill? probably. does it work? yes.

## results

it works. she's actually saving more than before. pays attention to her budget instead of ignoring it. the daily nudge changes behavior.

## what i learned

**daily emails are better than making people check dashboards.** no friction. hits the inbox at a time she'll actually read it.

**docker is worth learning for personal projects.** develop locally, run anywhere.

**small additions like horoscopes make utilities feel less like chores.** doesn't change financial outcomes, but people actually want to get the email instead of dreading it.

**2-day projects that are good enough are better than perfect projects that never ship.** this has been running for months with periodic tweaks.

**build for the specific person using it.** customized for her paycheck dates, her savings goals, her sign. that specificity is what made it useful.

## next

keep tinkering. most recent addition was the horoscope. there's always something to refine.

the repo is open source. it's small enough to understand in an afternoon. if spreadsheet-based budgeting feels intimidating to someone, sometimes they just need it delivered to their inbox instead.
