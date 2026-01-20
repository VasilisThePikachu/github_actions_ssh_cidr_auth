A terrible script in Rust that fills in an `authorized_keys` `from="awawa"` field with all the IP's of Github action runners CIDR's.

## Why?
I reworked my sites action script to instead upload my files via rsync to my own server instead of Github pages. While `rrsync` and the `restrict` options in `authorized_keys` are probably good enough here I wanted to go FURTHER and make sure that key could ONLY ever be used by a github action runner in the case of compromise.

Now the attacker either has to do their entire attack over GH runners or DDOS github (Probably easier then the former.)

Also an excuse to make my first program in rust, because rust is SECURE.

## How to use (or at least how I did it)
1. Compile it (`cargo build --release`)
2. Copy the executable to /usr/bin, mark it as executable
3. Edit /etc/ssh/sshd_config and add the following at the end
```
Match User YOUR_USER_HERE
        AuthorizedKeysCommand /usr/bin/github_actions_ssh_cidr_auth /authorized_keys
        AuthorizedKeysCommandUser nobody
```

"Why is authorized_keys on the root of the file system here" I did not wanna bother setting proper perms for nobody to be able to see the folder. You do it properly I did not care enough.

NOTE: Only the first entry of the authorized_keys will get the IP added to. This essentials just prepends `from="Action runner CIDRs here"` infront of whatever you feed into the argument (in this case `/authorized_keys`) and just spits out a `authorized_keys` in sdout.

4. Fill in the authorized_keys file that you passed into the program, here is my example for my use case, you could of course not have any special options and just have the key file. But if you are letting a GH runner have access to your SSH you probably wanna limit what it can do:
```
command="/usr/bin/rrsync -wo /opt/website/",restrict ssh-ed25519 AAAAC.....
```

In this case, this forces the rrsync command to run which is there to setup restricted rsync, in this case allowing writing only and only allowing access to `/opt/website/`, `restrict` is just a shortcode for `no-agent-forwarding,no-port-forwarding,no-pty,no-user-rc,no-X11-forwarding` which I do not use in my runner script so they can go and further down restricts damage this key can do if compromised.

5. Reload sshd, and try it out.

This will to my knowledge run on the each connection attempt to the user via SSH. So don't put it under a popular username because it will probably spam github's API endpoint and I did not wanna bother writing a cache or something.

"Can i just put the output into `authorized_keys` instead of running this on every connection?"

Well yes, but github themselves says these ip's come and go. So they may be out of date. If you want, another way of running this is just putting it on a timer and just piping the output straight into the `authorized_keys` files of the prefered user.

There is no real error handling here so I guess you can add it on yourself and contribute it? I am not responsible if this breaks. sshd will use the usual configured auth key file if the `AuthorizedKeysCommand` fails.