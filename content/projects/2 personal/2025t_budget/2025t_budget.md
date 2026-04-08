_Overview_

i had a budget spreadsheet ive been using for my own personal budgeting back in 2020 when i studied abroad in germany. recently, my partner wanted to start budgeting so i shared it with her and customized the spreadsheet to fit her needs: revolve around when she gets paid, customize savings/goals, paying off student loans, etc.

however, it was a difficult interface to constantly check google sheets on the web or on mobile. i was considering making and hosting a custom website, but it would just be over the top to interact with a spreadsheet that is bound to change. people dont do things that require friction, even when important

the obvious fix was to push the info to her rather than having her pull it. a daily email with a summary on her balance, spending patterns, and progress twd her saving goals.

### dev stack

took about 2 days to build (but many many more to optimize, fix bugs, etc.)

_google sheets api_

the budget lives in a google sheet - i connect using `service_account.json` which is provided by google api services: [https://docs.cloud.google.com/iam/docs/keys-create-delete](https://docs.cloud.google.com/iam/docs/keys-create-delete). we all know i love rust but this is not meant to be perfomative or fast.. so i just used `google-auth-*` and `google-api-python-client` python packages for quick interfacing

_email rendering_

used gmail's smtp interface with python's `smtplib` to send. jinja2 for templating so the html structure is separated from data. easy to tweak the layout without touching the code.

```python
# simplified example of how the email gets constructed
import smtplib
from email.mime.text import MIMEText
from jinja2 import Template

# render the budget summary template
template = Template(email_html_template)
budget_summary = template.render(
    total_balance=current_balance,
    savings_goal=savings_target,
    progress_percent=(current_savings / savings_target) * 100,
    horoscope=daily_horoscope,
    # ... etc
)

# send
msg = MIMEText(budget_summary, 'html')
smtp_server.send_message(msg)
```

_deploy to docker_

needed this running on my nas at home which uses `TrueNAS`. this used to be on `FreeBSD` now its something else? regardless, i can't install custom tooling there but docker images are fine, so i containerized it with a slim python image and `cron` job:

```bash
0 7 * * * docker run --rm [args] budget-reminder:latest
```

_fun features_

after a few weeks she said the email was useful, but i could always add fun things like a quote or whatnot. so i added a daily horoscope :) process: get her astrological sign from her birthday and fetch + render the horoscope reading.

the website rendering horoscopes with client-side javascript. simple GET requests gave stale data, so i had to significantly over-engineer the solution with browser emulation (`playwright`)

### results

<iframe
    src="./reminder-example.html"
    class="center"
    height="2650"
    style="display:block; width:100%; max-width:800px; margin:0 auto;"
></iframe>
