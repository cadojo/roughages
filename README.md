# 🌾 `roughages`

_A social media feed generator through the AT Protocol._

A Rust crate (library), and accompanying CLI tool to create, publish, and otherwise interact with [AT Protocol](https://atproto.com) feeds. The goal here is to create a performant and flexible interface for social media algorithm implementations. There are lots of social media feeds, and right now they're all using Python or JS. Let's save some electricity and build a safe and effective implementation with Rust!
## Motivation

There are lots of feeds on Bluesky right now, and they all seem to be really simple. There is great work already done, and an official (working!) skeleton structure provided by the Bluesky team, but what about more complicated feeds? What about feeds that apply complex strings of logic? Could we create an interface that can __chain__ algorithmic steps together? If a solid and performant implementation is available, actual algorithmic experts might more easily create ethical and fun social media feeds for Bluesky.

## Vision

The AT Protocol is encoded in Rust's type system. Algorithmic _steps_ are also encoded in the type system, and can chain together. Users can share algorithmic steps as trait implementations. Each trait implementation can act on the AT Protocol type.

## Outline

1. Encode the AT Protocol in Rust's type system.
2. Implement send/receive for the AT Protocol.
3. Encode posts in the type system.
4. Encode algorithmic steps.
5. ???