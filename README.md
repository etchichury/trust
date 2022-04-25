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

Here, I'll describe my journey trhought this project, my thoughts and findings.

### 04/24/22 - Sunday

I first downloaded Rust, and already noted something interesing: it has specific command to install when you're using WSL:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Creating a new project using Cargo (Rust's package manager) is really starightfoward. And I can say the samething for setting up the manifest file (adding metadata and managing dependencies).

Since we want to deal inside Linux' user space we have to use [TUN/TAP](https://www.kernel.org/doc/Documentation/networking/tuntap.txt) and Rust already provides an [interface](https://docs.rs/tun-tap/latest/tun_tap/) for that!

The first iteration of the program is really simple. We create a new tun interface and print what we receive. What is a little more complex is how to setup the device create by this program. First we need to setup a capability to our program using the `setcap` command.

According to `man capabilities`:

> Linux divides the privileges traditionally associated with superuser into distinct units, known as **capabilities**, which can be independently enabled and disabled.

The capabilty we want to set is `cap_net_admin` which according to `man capabilities`:

> Perform various network-related operations:
>
> - interface configuration;
> - [...]
> - modify routing tables;

After that, we add a new protocol address (`ip addr add` command) specifing the range of IPs and the device to add the address to (`man ip-address` for more information). We are also going to need to bring up the device interface using the `ip link set` command (`man ip-link` for more information). To avoid going trhough these steps every time, we can create a bash script to do it for us.

I learned a new bash syntax (`$!`) which gets the process ID (PID) of the last job run in the background.

Using `ping`, we start sending packet to our interface (`ping -I tun0 <ip_addr>`). Once we see that we're receiveing packets, we can start deconstructing these packets following the TUN/TAP docs:

- Flags [2 bytes]
- Proto [2 bytes]
- Raw protcol(IP, IPv6, etc) frame.

To track these flow of pacckets we can use `tshark`, which is the TUI version of the [Wireshark](https://www.wireshark.org/) program. I didn't know that Wireshark had a TUI version, but when I stop to think about it, it makes total sense.

It turns out that our program is running in the background even after terminating it (`CRTL+C`). So I learned two new commands:

- `pgrep`: look up for process based on name and other attributes. Example: `pgrep -af <name>`.
- `term`: bash command that catches signals (like EXIT) and can execute code when they occur. Using this, will solve the issue where the process was still running (in background), even after exiting it.

Regarding Rust, what I saw so far is actually really simple. There are some diferent syntaxes, like `::` and `..`, but they all tranlate to something I've already seen in other languages. One thing that I have to point out, is the Rust documentation. It's so _f_...ing nice! You get the Rust by Example book that explains and shows how things work in practice, and what I REALLY enjoyed: the standard library documentation, and **much** more. It got great keyboard intagration, the search algorithm is fast and works great, the explanations and examples are easy to follow and understand.

It has been about 3 hours since I started this project. I spent this time: installing and setting up Rust development environment, reading Rust documentation, going back and forth `man` pages, writing this README and, of course, following Jon's video. So far, I've watched 38 minutes of the video out of 5 hours. I guess this will take a bit longer than what I initially expected. :)
