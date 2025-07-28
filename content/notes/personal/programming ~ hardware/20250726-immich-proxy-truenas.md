# setting up Immich-Public-Proxy on TrueNAS Scale

TLDR: Use Cloudflare!!!!

There are plenty of tutorials on how to transfer your domain to Cloudflare from any other registrar (e.g., I was originally on GoDaddy), so this step is purposefully not included. A basic overview is to change your nameservers to Cloudflare on the original registrar, then turn off all "locks" on the domain to initiate transfer, get an authorization code, import into Cloudflare, wait some time for DNS to update, then import into Cloudflare, then done.

See my [struggle](#struggle) at the end to understand how I got here.

[result :) click me to see a picture of my beautiful childhood pet cat](https://images.arpadvoros.com/share/zHZ_RlibRuu-D-mDv8lnpYRr3sv7ySpxGBSyzK3dN-K8qAXWoyr_MZMIm6zmr8Gai48)

## goals

1. Share read-only images/albums with friends on my website @ `images.arpadvoros.com`
2. Host images on my TrueNAS Scale NAS using Immich
3. Click `Share` and fetch a link (with/without password)
4. Secure with HTTPS

## assumptions

1. User is using TrueNAS Scale
2. User has Immich installed as a TrueNAS app (just Docker)
3. User has a domain on Cloudflare

## steps

### Install Immich Public Proxy as Custom TrueNAS App

[Immich Public Proxy](https://github.com/alangrainger/immich-public-proxy) is a tool to only expose read-only endpoints of Immich, preventing things like the `/api` endpoint or writable endpoints. This is at least my understanding.

On TrueNAS, this app does not exist. It must be installed manually.

I would highly recommend [this video by Techno Tim](https://www.youtube.com/watch?v=gPL7_tzsJO8) on the best approach to install apps manually on TrueNAS. It makes things easy, and I version control all of mine through a private git repo.

A TLDR of the video is:

1. Log into TrueNAS
2. Go to `Datasets -> Add Dataset`
3. Create one for your custom app. In this case, I called mine `immich-public-proxy`
4. This creates a folder called `<path-to-pool>/immich-public-proxy`. Set permissions if need be (explained in video)
5. Add any `docker-compose.yml` file inside of `<path-to-pool>/immich-public-proxy`
6. Navigate to `Apps -> Discover Apps -> ... Install via YAML`
7. Add the following:

    ```yaml
    include:
      - "<path-to-pool>/immich-public-proxy/compose.yml"
    ```

8. Click `Save` and you can now see your custom app!

### Configure Immich Public Proxy

The instructions above were for any Docker Compose YAML file -> custom TrueNAS app. These instructions are explicitly for Immich Public Proxy.

1. Copy-paste [this `docker-compose.yml`](https://github.com/alangrainger/immich-public-proxy/blob/main/docker-compose.yml) into `<path-to-pool>/immich-public-proxy/compose.yml`
2. Change `PUBLIC_BASE_URL` to your domain, e.g., `images.arpadvoros.com` for me. Even if this domain is not yet configured, don't worry.
3. Change `IMMICH_URL` to the URL of your Immich instance. This can **not** be `localhost` since this URL is defined within the Docker container. It must be a static IP or some domain name of your NAS. For example, `192.168.0.100:30041`.
4. I would recommend adding another environment variable called `IPP_PORT` to customize your port value. Ensure to modify the `ports` and `healthcheck` sections with that hard-coded port. If this is not done, it will **always** default to 3000, which prevents the healthcheck from working and looks like the app is not running. This environment variable is not documented in the repo, but you can search for it in issues and [also here](https://github.com/alangrainger/immich-public-proxy/blob/93b6e6ef5171ec0cd6600c6c4af2659527560b34/app/src/index.ts#L186).

To verify all this, I would navigate over to Immich and try to get a share link. This share link will give you the following format:

`http[s]://<NAS-IP>:<IMMICH-PORT>/share/<UID>`

Go ahead and _**just**_ change the port from the Immich port to the Immich Public Proxy port, and if you can see the image then the proxy is running properly. Ensure to also check the logs via Docker or via the TrueNAS apps logs.

### Using Cloudflared Tunnel

Documentation:

- [https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/)

My instructions:

1. On TrueNAS, look for the [`Cloudflared`](https://github.com/cloudflare/cloudflared) app
2. During installation, it will ask for a Tunnel API token
3. Navigate to [Zero Trust - Cloudflare Dashboard](https://one.dash.cloudflare.com/)
4. Go to `Networks -> Tunnels -> Create a tunnel`
5. Select `Cloudflared`
6. Enter a descriptive name for the tunnel. This is **not** the service, so name it something like `mynas`
7. Copy the Docker command. Don't run it, just copy the token out of it
8. Return to TrueNAS (step 2) and enter the token there. Create the `Cloudflared` TrueNAS app
9. Return to Cloudflare, click `Public hostnames` to add one
10. Add:

    ```markdown
    **Subdomain**: images
    **Domain**: arpadvoros.com
    **Service Type**: HTTP      _# this is internal / local. It is HTTP locally, but HTTPS via the Cloudflare tunnel. You can set this to HTTPS if you have Immich configured as such._
    **URL**: NAS_IP:IPP_PORT    _# this is the Immich Public Proxy env variable, see above._
    ```

11. Click `Save`

And it should be magically done! Now you can modify in your Immich settings the endpoint of your share links. And now you will automatically have HTTPS thanks to Cloudflare, the URL will automatically be forwarding to the Cloudflared tunnel to your NAS:IPP_PORT, and it should just work. This was far easier than the struggle I initially had with a bunch more moving parts. Read below.

## notes

Previously I was using DuckDNS for DDNS resolution, I wanted HTTPS using Caddy, custom certificate generation, port forwarding on my router, etc.

And it did not help that I was using GoDaddy, which barely had any options for advanced configuration, and my router was also awful. I spent 4+ hours the other day running around in circles to get this to work with no avail. I definitely learned a lot, however, the fact that at the end of the day it **still** did not work and it was completely out of my hands (due to GoDaddy and my router not allowing advanced configuration) was extremely frustrating. I have been using DuckDNS in the past for DDNS resolution but after searching online, it looked like Cloudflare solved all my issues:

- Domain registration + purchase
- Allowed for advanced configuration of DNS
- Using `Cloudflared`, set up a tunnel rather than a traditional DDNS solution like DuckDNS
- No port-forwarding required, since `Cloudflared` also solved this
- All free, similar to previous solutions, but under one provider

The only thing I had to pay for at the end of the day was the registrar transfer, moving my domain from GoDaddy -> Cloudflare. Which was like 10 bucks and so worth it.

## struggle

Rather than going into detail, I will just relay my ChatGPT conversation so you can see the insanity I was attempting to tie together and failing to do so. It did not help that my [router (TP-Link Festa FR205)](https://www.tp-link.com/us/business-networking/soho-festa-gateway/festa-fr205/) absolutely sucks in terms of configurability... In order to port-forward anything or manage your router you can NOT just log into the default web interface, but go through some TP-Link cloud?? And then you lose DHCP functionality/access/control, and once you have the cloud configured you can no longer log in locally?? It is awful.....

Anyways, here is the gist of the initial approach prior to moving to Cloudflare:

```markdown
a lot of moving parts. need your help.

context: networking help to expose a read-only API on my NAS to the world wide web to share images/albums with friends

1. i have a NAS which runs truenas scale
2. on said NAS, i have immich which stores my images locally
3. i see `immich public proxy` (ipp) project, which prevents exposing the /api endpoint and ensures any external user stays as read-only
4. on the ipp repo, it is recommended to use Caddy to forward expose this port on https. it also gives instructions on how to make a certificate
5. i have a domain on Godaddy under myname.com. id want to share images to images.myname.com
6. i dont have a static IP with my ISP, so i am using duckdns at myname.duckdns.org
7. my NAS has a cron job which executed (6) to keep the ip's up to date, however, due to some misconfiguration(?) it exposes my routers IP address
8. my router is a tp link festa and it barely has any options for port forwarding...

given ALL this, here is what went down:
1. if i add CNAME images myname.duckdns.org on GoDaddy, it literally exposes my router's web portal to the WWW... **very bad**
2. my immich server works without issue. the IPP also works, since it runs locally on port 3000. so i am able to check health and see my shared pictures there
3. my caddy configuration should be forwarding port 3000 to images.myname.com, but it gives an error with the certificate signing about how there is no A or AAAA record in for images.myname.com

however, on GoDaddy i cant add an A or AAAA record without it being a static ip address

so my question is:

1. any way to accomplish my goals without tls/https and not have a need for caddy?
2. how to ensure caddy works correctly and properly setting up https?

please search the web and be thorough
```
