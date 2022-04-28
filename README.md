# Trust (TCP RUST)

This is a simple implementation of the TCP protcol in [Rust](https://www.rust-lang.org/).

## Description

This is an implementation following the [YouTube videos from Jon Gjenset](https://www.youtube.com/watch?v=bzja9fQWzdA&).
Jon is followign the [RFC 793](https://datatracker.ietf.org/doc/html/rfc793) - which describes the original protocol. For this project, IPv6 implementation will be set aside.

I always found low level stuff interesting, but never really tried to learn something, besides college level stuff (basic). This weekend I saw this series and decided to give it a try.

I have a base understanding of the TCP protocol and never wrote one line of Rust. So, this is a good project for a Rust beginner, right? What makes me think I can, at least, follow up Jon's videos is that:

- I'm not a total beginner on comp science/software engineering. I have a degree in Software Enginering
- I've beeing working as a software engineer at HP for almost 2 years
- Always used Linux (currently using [WSL](https://docs.microsoft.com/en-us/windows/wsl/))
- I like to read documentation (`man` is my savior!)
- I really like to learn new languages and their eccentricities

If I stop and think, it's not just Rust that I'll learn along the way. What I can learn from this is quite exciting:

- Rust
- Networking
- Linux
- Typing and keyboard shortcuts

## Running

Build the project:

```sh
cargo build --release
```

Run the project:

```sh
./run.sh
```

**Note:** you'll probably need to give execution permission to the `.sh` file:

```sh
chmod +x run.sh
```

## Captain's Log (yes, it's a Star Trek: The Original Series reference)

Here, I'll describe my journey throught this project, my thoughts and findings.

### 04/24/22 - Sunday

I first downloaded Rust, and already noted something interesting: it has a specific command to install when you're using WSL:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Creating a new project using Cargo (Rust's package manager) is really starightfoward. And I can say the same thing for setting up the manifest file (adding metadata and managing dependencies).

Since we want to deal inside Linux' user space we have to use [TUN/TAP](https://www.kernel.org/doc/Documentation/networking/tuntap.txt) and Rust already provides an [interface](https://docs.rs/tun-tap/latest/tun_tap/) for that!

The first iteration of the program is really simple. We create a new tun interface and print what we receive. What is a little more complex is how to setup the device created by this program. First we need to setup a capability to our program using the `setcap` command.

According to `man capabilities`:

> Linux divides the privileges traditionally associated with superuser into distinct units, known as **capabilities**, which can be independently enabled and disabled.

The capabilty we want to set is `cap_net_admin` which according to `man capabilities`:

> Perform various network-related operations:
>
> - interface configuration;
> - [...]
> - modify routing tables;

After that, we add a new protocol address (`ip addr add` command) specifing the range of IPs and the device to add the address to (`man ip-address` for more information). We are also going to need to bring up the device interface using the `ip link set` command (`man ip-link` for more information). To avoid going through these steps every time, we can create a bash script to do it for us.

I learned a new bash syntax (`$!`) which gets the process ID (PID) of the last job run in the background.

Using `ping`, we start sending packet to our interface (`ping -I tun0 <ip_addr>`). Once we see that we're receiveing packets, we can start deconstructing these packets following the TUN/TAP docs:

- Flags [2 bytes]
- Proto [2 bytes]
- Raw protcol(IP, IPv6, etc) frame.

To track these flow of packets we can use `tshark`, which is the TUI version of the [Wireshark](https://www.wireshark.org/) software. I didn't know that Wireshark had a TUI version, but when I stop to think about it, it makes total sense.

It turns out that our program is running in the background even after terminating it (`CRTL+C`). So I learned two new commands:

- `pgrep`: look up for process based on name and other attributes. Example: `pgrep -af <name>`.
- `term`: bash command that catches signals (like EXIT) and can execute code when they occur. Using this, will solve the issue where the process was still running (in background), even after exiting it.

Regarding Rust, what I saw so far is actually really simple. There are some diferent syntaxes, like `::` and `..`, but they all tranlate to something I've already seen in other languages. One thing that I have to point out, is the Rust documentation. It's so _f_...ing nice! You get the Rust by Example book that explains and shows how a lot things work in practice. And what I REALLY enjoyed: the standard library documentation. It got great keyboard intagration, the search algorithm is fast and works great, the explanations and examples are easy to follow and understand. There's so much content that it's almost overwelming, but in fact it's not because it's so well structured and easy to navigate.

It has been about 3 hours since I started this project. I spent this time: installing and setting up Rust development environment, reading Rust documentation, going back and forth `man` pages, writing this README and, of course, following Jon's video. So far, I've watched 38 minutes out of 5 hours. I guess this will take a bit longer than what I initially expected. :)

### 04/27/22 - Wednesday

A nice thing I realized after staying away from this project for a couple of day is that this series of videos are not a tutorial. Jon doesn't explain every command he types or go in depth of what he is doing as a tutorial would do. This is great, and for me, this is way better than standard tutorials. With this videos, **I** have to go after the knowledge gap, if I don't know what a command does or what he is talking about, I need to look through documentation, or else I'll be lost. I found a really nice way to learn.

There are two packets that we're receiveing, one from ping and the other one I don't knowunderstand where it's coming from (neihter does Jon). The packets have two different hex values (proto):

```
// unkown
read 52 bytes: [0, 0, 86, dd, 60, 0, 0, 0, 0, 8, 3a, ff, fe, 80, 0, 0, 0, 0, 0, 0, 1f, 6f, ba, b3, 75, af, 67, e1, ff, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 85, 0, c5, 83, 0, 0, 0, 0]
// ping
read 88 bytes: [0, 0, 8, 0, 45, 0, 0, 54, ea, 8f, 40, 0, 40, 1, ce, c5, c0, a8, 0, 1, c0, a8, 0, 2, 8, 0, 47, 17, 45, 58, 0, 7, b0, 0, 6a, 62, 0, 0, 0, 0, 86, 53, c, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37]
```

- `0x0800`: Internet Protocol Version 4 (IPv4) | Source: ping
- `0x86DD`: Internet Protocol Version 6 (IPv6) | Source: unknown

To parse the packet, Jon searches for a crate that parses it for us in a [website that hosts documentation host for Rust crates](https://docs.rs/about).

Something really cool that now I confirmed is that just adding the dependecy to the manifest file is enough for the autocomplete. It doesn't require downloading or build it, it just works. I'm curious to know how they do it. this might be a future search.

Now that we are parsing flags, protocol and raw protocol we have this:

```
read 84 bytes: (flags: 0, proto: 800) Ipv4HeaderSlice { slice: [45, 0, 0, 54, 41, d6, 40, 0, 40, 1, 77, 7f, c0, a8, 0, 1, c0, a8, 0, 2] }
```

Once I started adding the parsing, I saw that we are using `.` to call a method. I didn't understand why this was, shouldn't be `::`? No, and the explanation is pretty interesting, it turns out this helps identify and differentiate static methods (using `::`) and dynamic methods (using `.`). This [Reddit comment](https://www.reddit.com/r/rust/comments/3fimgp/comment/ctqfg33) explanation really helped me better uderstand it. I also found a really good source that will help me understand things like this and future reference it: [Appendix B: Operators and Symbols](https://doc.rust-lang.org/book/appendix-02-operators.html)

Filtering a bit more (source, destination, size and protocol), we have this:

```
192.168.0.1 →  192.168.0.2 | 64 bytes of protocol 1
```

We have two protocols because they are from different levels.

- Ethernet frame protocol: IPv4 (`0x0800`)
- IP protcol: TCP (6) that we want or ICMP (1) when we `ping`.

Didn't do much today, but learned a lot. Ending this session at 48 minutes of the video.
