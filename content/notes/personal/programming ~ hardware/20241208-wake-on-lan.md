# Wake-on-LAN Instructions (Linux)

## On device that needs to be remotely woken

### Requirements

```bash
sudo apt install ethtool
```

### Setup

1. See network interfaces (NIC's)

    ```bash
    ip link show
    ```

    Select the default one (state up, not loop back, usually `eth0`, `eth1`, etc.)

2. See WoL status

    ```bash
    sudo ethtool <NETWORK-DEVICE-NAME>
    ```

    * If `Supports Wake-on` has a `g` character, then it is supported.
    * If it does not, then you can swiftly **exit this tutorial, as you can not wake-on-LAN the device you want to :(**

    See if `Wake-on` is `g`. It can be any of the following:

    _Source: [https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/6/html/deployment_guide/s1-ethtool](https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/6/html/deployment_guide/s1-ethtool#idm140653496737808)_

    * `p` — Wake on PHY activity.
    * `u` — Wake on unicast messages.
    * `m` — Wake on multicast messages.
    * `b` — Wake on broadcast messages.
    * `g` — Wake-on-Lan; wake on receipt of a "magic packet".
    * `s` — Enable security function using password for Wake-on-Lan.
    * `d` — Disable Wake-on-Lan and clear all settings.

    If not `g`, then run the following to set it:

    ```bash
    sudo ethtool -s <NETWORK-DEVICE-NAME> wol g
    ```

3. Set WoL status every-time on boot to `g`

    `/etc/systemd/system/wol.service`:

    ```bash
    [Unit]
    Description=Enable Wake-on-LAN
    Wants=network-online.target
    After=network.target network-online.target # <-- `network.target` is optional, backwards compatibility

    [Service]
    Type=oneshot
    ExecStart=/usr/bin/sudo /usr/local/bin/set-wol.sh
    User=root

    [Install]
    WantedBy=multi-user.target
    ```

    `/usr/local/bin/set-wol.sh`:

    ```bash
    #!/bin/bash

    DEV=<NETWORK-DEVICE-NAME>
    SLEEP_S=10
    MAX_RETRIES=50
    RETRY_COUNT=0

    while ! ip link show $DEV | grep -q "state UP" && [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
        sleep $SLEEP_S
        ((RETRY_COUNT++))
    done

    if ip link show $DEV | grep -q "state UP"; then
        /usr/bin/sudo /usr/sbin/ethtool -s $DEV wol g
    else
        echo "Failed to bring up $DEV after $MAX_RETRIES attempts."
        exit 1
    fi
    ```

    Note: I tried to simply make the `ExecStart` in the systemd service: `/usr/bin/sudo /usr/sbin/ethtool -s <NETWORK-DEVICE-NAME> wol g`, but it didn't seem to work.

    It might come down to the NIC not being up yet, because of either 1. standard boot procedures or 2. disk decryption (which I have).

    Then, run the following to start the service:

    ```bash
    sudo chmod +x /usr/local/bin/set-wol.sh
    sudo systemctl daemon-reload
    sudo systemctl enable wol.service
    sudo systemctl start wol.service
    ```

    Check `sudo systemctl status wol.service` and `sudo ethtool <NETWORK-DEVICE-NAME>` to verify.

4. Get the MAC address of the device

    ```bash
    ip addr show <NETWORK-DEVICE-NAME> | grep ether | awk '{print $2}'
    ```

    Save this, since you will need this on all other LAN devices that need to magically boot this device.

## On device that wakes remote device

### Requirements (Linux)

```bash
sudo apt install wakeonlan
```

#### Requirements (Windows)

```powershell
winget install --id=DarkfullDante.wol  -e
```

### Setup (Linux)

```bash
wakeonlan <MAC-ADDRESS>
```

### Setup (Windows)

```powershell
wol -m "<MAC-ADDRESS>"
```

## Conclusion

Done! Can further automate this, e.g. making a powershell/bash script to automatically WoL + decrypt disk upon trying to SSH to the device, by mapping domain names to MAC addresses. But that is niche enough and fairly easy which is outside the scope of this note.
