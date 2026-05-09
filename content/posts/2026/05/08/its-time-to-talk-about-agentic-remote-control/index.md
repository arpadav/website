# it's time to talk about agentic "remote control"

>tldr: skip the vendor remote-control features. roll your own with wireguard, `tmux`, and ssh. cheaper, more flexible, and you stop handing third parties a live shell into your dev box.

i've been seeing posts floating around for the past couple months, particularly on the Anthropic side, advertising their ability to remotely control your Claude Code session from your phone! i even saw on linkedin a couple days back that [amp added their own remote control](https://ampcode.com/news/neo)

what a wonderful piece of technology, right?

as someone who has been doing "manual" remote control for about two years, i can tell you that these AI providers implementing them is simply not worth it.

to cut to the chase: use your own VPN / Wireguard, terminal sessions, and terminal access control point, and stop graciously providing third parties yet another attack surface.

## DIY remote control

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

*blast radius of an account compromise*: SIM swap, OAuth phish, session hijack on your vendor account -> attacker gets the same remote shell capability you do. with device-keyed wireguard + SSH keys in the phone's secure enclave, owning a vendor account does not put an attacker on the mesh.

*credentials never leave your devices*: anything that scrolls past in the terminal. `env`, `cat .env`, accidental `aws sts get-caller-identity` will always transit vendor infrastructure. TLS-to-the-relay is not the same as never-having-existed-there.

*logging you don't control*: vendors retain telemetry. privacy policies change. your in-progress security fixes and customer data fixtures live somewhere you can't audit.

for reassurance, the past couple days we've had:

* [https://www.redcaller.com/docs/references/mcp-client-oauth-refresh-token-support](https://www.redcaller.com/docs/references/mcp-client-oauth-refresh-token-support)
* [https://github.com/advisories/GHSA-vp62-r36r-9xqp](https://github.com/advisories/GHSA-vp62-r36r-9xqp)

and im sure there is much more i am missing, since i am no security expert.

## \* *addendum*: investigation of claude code's rc

>tldr: regardless of remote-control use, a compromised provider still has active connections to your machine and could *potentially* execute malicious code

i wanted to actually see what claude code does on the wire when remote control is on, so i had claude itself write me a tracing harness, consisting of a single bash script with the following:

1. `strace -f -tt -T -y` for every top-level pid in the tree (one log per pid, `-f` follows children)
2. `tcpdump -i any -s 0 'tcp or udp'` for the full pcap
3. `execsnoop-bpfcc -T` system-wide for every `exec()`
4. `opensnoop-bpfcc -T` system-wide for every `open()`
5. `tcpconnect-bpfcc -t` for outbound TCP `connect()`
6. `tcpaccept-bpfcc -t` for inbound TCP `accept()`
7. `fatrace -t` for filesystem access
8. `ss -tnpeO` snapshotted every 1s
9. `pstree` + `lsof` snapshotted every 5s
10. an initial `/proc` + `ss` + `lsof` snapshot at start, and a matching one at end

attached to a long-running claude code session (cwd: a normal project repo), let it idle for 10 seconds or so, turned on remote control, sent it one prompt mid-trace, turned off remote control, then stopped tracing after ~50 seconds. then handed the whole log directory back to claude and asked it to build the timeline

### what the trace shows

well.. no new TCP SYN or DNS. the prompt arrived, and the message rode in over a socket that had been open the entire time

specifically, before i ever touched my phone, claude already had **8 keep-alive TLS sockets to `160.79.104.10:443`** (anthropic edge, no PTR record) plus one to `35.190.46.17:443` (statsig on google cloud). the MCP children had their own sockets to cloudflare. when the remote prompt landed, it just appeared as bytes on one of the existing 8 sockets. local hooks fired (`userpromptsubmit.mjs`), `git ls-files` ran, `~/.claude.json` got rewritten (all the normal prompt-arrival machinery and nothing out of the ordinary). nothing on the network looked like "a remote control connection just opened"

### why this matters

those 8 sockets are open whenever claude code is running. they exist regardless of whether you have remote control toggled on. the client is, by construction, sitting on a persistent bidirectional channel to anthropic with the *capability* to receive instructions at any moment

the remote control toggle is presumably gating *something*.

*best case*: the client refuses to dispatch inbound prompt messages unless the local flag is set, AND anthropic refuses to forward them server-side unless your account opted in. defense in depth on both ends.

*worst case*: the toggle is purely a server-side ACL, where anthropic decides whether to push, the client always accepts. in this case a compromised anthropic (or anyone with the right internal access) could inject a user prompt into any open claude code session anywhere, with no client-side gate

from a packet trace alone, i couldnt tell which, since they both look identical :)

and remember the existing trust model already lets the model emit tool calls. remote control doesn't expand the *blast radius* of a compromise. it expands *who can pull the trigger*. without it, an attacker needs you to actually be typing. with it, an idle session at 3am is reachable.

so: same conclusion as the rest of this post - just roll your own rc for flexibility and reduced risk
