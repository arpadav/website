# it's time to talk about agentic "remote control"

>tl;dr: skip the vendor remote-control features. roll your own with wireguard, `tmux`, and ssh. cheaper, more flexible, and you stop handing third parties a live shell into your dev box.

i've been seeing posts floating around for the past couple months, particularly on the Anthropic side, advertising their ability to remotely control your Claude Code session from your phone! i even saw on linkedin a couple days back that [amp added their own remote control](https://ampcode.com/news/neo)

what a wonderful piece of technology, right?

as someone who has been doing "manual" remote control for about two years, i can tell you that these AI providers implementing them is simply not worth it.

to cut to the chase: use your own VPN / Wireguard, terminal sessions, and terminal access control point, and stop graciously providing third parties yet another attack surface.

## diy remote control

boils down to three fundamentals:

```text
phone (wireguard) -- direct e2e --> dev machine --> terminal session with claude/amp/etc
```

rather than

```text
phone --> provider app --> provider cloud --> provider relay --> agent process on your machine
```

### VPN

i personally use [tailscale](https://tailscale.com/), where i run multiple end-points and authenticate myself from various devices.

however, i have been experimenting with [headscale](https://headscale.net/stable/) - a self-hosted and open-source implementation of tailscale - i have the ability to run it on my NAS and forward it using [cloudflared tunnels](https://github.com/cloudflare/cloudflared) to a readily-accessible domain name/space. the switch would enable maximum control and would not use tailscale servers as a middle-man.

im sure other alternatives exist, but this is the one i have landed on for their mobile compatibility

### terminal sessions

`tmux` or `zellij` to keep sessions alive

### terminal access

on Apple (not an ad i swear) i have been in love with [termius](https://termius.com/index.html), and it looks like they have many other available platforms (and they even support `mosh`!)

however, ive also experimented with [`ttyd`](https://github.com/tsl0922/ttyd) or something similar, where simply running 

```bash
ttyd -W -p <PORT> bash
```

exposes your terminal to `0.0.0.0` to a specific port

## workflow

1. use the tailscale app, connect to tailnet, authenticate
2. use termius, `ssh` to domain of choice, connect to `zellij` or `tmux` sessions
3. ??? profit

the amount of times i am using Apple's speech-to-text to talk to my agent through the terminal, or kicking off sessions or scripts while in bed, getting out of the car, on the go, and more, is very satisfying. this is why "remote control" is desireable in the first place. but i can assure you - after trying the provider options like codex and claude code through the app, they just feel far more restrictive than helpful

## PROS: third party remote control

### barrier of entry

for the non-technical, way less set up

### push notifications

push notifications for requesting input or concluding an agentic session is probably the largest pro to these apps, but im sure you can experiment with adding hooks and integrating with some messaging system if you really care about this

## CONS: third party remote control

### flexibility

theres always more i want to do OUTSIDE of agentic development... most of the times if i am just doing text editing, managing my home-lab, etc.

this is particularly the case when i just want to kick off a couple of scripts or make some todo lists, which i can access on my main machine later. hell, setting up SMB / NFS shares is also stupidly easy on mobile devices too (i digress)

### the middle man

and why use tokens and have some LLM do these for you always? being super dependent on a particular provider is very strange to me. i have used claude code, codex, opencode, pi, and more, and i switch between them for different tasks - and having a raw terminal at your fingertips and having those OPTIONS is always what i have liked - not being forced to go through any particular provider, and more importantly, ANY provider

raw terminal also means you can run claude in one pane, codex in another, a test loop in a third, `htop` in a fourth. most vendor remote UIs give you one agent at a time.

### waiting on vendors 

you can sit around twiddling your thumbs waiting for your favorite coding agent to release their version of remote control, or [flood projects with issues begging for implementation](https://github.com/openai/codex/issues?q=remote%20control%20state%3Aclosed%20label%3Aenhancement), the choice is yours.

plus, every provider is going to implement their remote control differently, with different abilities, etc. i am not going to context switch and have multiple apps or urls when i could simply `Ctrl + b + n`

### security

this is the biggest one of all.

*trust boundary*: vendor remote control extends the trust boundary of YOUR machine to include the vendor's auth system, their relay infrastructure, mobile app supply chain, employees with production access, and their incident response when (not if) they get breached. wireguard tunnels between your own devices use keys generated and held on each endpoint, where even a self-hosted control server can't decrypt the traffic.

*blast radius of an account compromise*: SIM swap, OAuth phish, session hijack on your vendor account -> attacker gets the same remote shell capability you do. your repos, your `.env`, push access as you. with device-keyed wireguard + SSH keys in the phone's secure enclave, owning a vendor account does not put an attacker on the mesh.

*credentials never leave your devices*: anything that scrolls past in the terminal. `env`, `cat .env`, accidental `aws sts get-caller-identity` will always transit vendor infrastructure. TLS-to-the-relay is not the same as never-having-existed-there.

*logging you don't control*: vendors retain telemetry. privacy policies change. your in-progress security fixes and customer data fixtures live somewhere you can't audit.

for reassurance, the past couple days we've had:

* [https://www.redcaller.com/docs/references/mcp-client-oauth-refresh-token-support](https://www.redcaller.com/docs/references/mcp-client-oauth-refresh-token-support)
* [https://github.com/advisories/GHSA-vp62-r36r-9xqp](https://github.com/advisories/GHSA-vp62-r36r-9xqp)

and im sure there is much more i am missing, since i am no security expert.
