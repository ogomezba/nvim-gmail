# nvim-gmail

Neovim plugin that allows for basic reading of last emails from the main Gmail
inbox and to send text emails from Neovim. Rather than a fully fledged plugin,
it is a minimum functional plugin used as an exploration of the capabilities
of the Neovim messagepack-RPC API.

## Installation

As the plugin uses Rust to perform the Gmail actions, it is required to build
the Rust binary for your platfrom which can be done using cargo. Cargo can be
installed with [rustup](https://rustup.rs/).

After cloning the repository, a simple build can be done by executing `$ ./build.sh`.
This generates the binary in the `/bin` folder, which can be later used when
configuring the plugin. It is okay to move the compiled file to somewhere else
and specify it's location when configuring the plugin (see below).

After that, just install it from local using your preferred package manager.
For example, using packer:

```lua
use({ "~/nvim-gmail/" })
```

In order to get access to the `:GmailOpen` and `:GmailSendEmail` commands, require
the package in your config:

```lua
require("nvim-gmail").setup({
	username = "yourgmail@gmail.com",
	pass = "your_password",
	bin_path = "path/to/rust/compiled/binary",
})
```

**NOTE**: In case 2-Step Verification is enabled for Gmail, you need to generate an
[application password](https://support.google.com/mail/answer/185833?hl) and use
that as your password when using `GmailInbox::new()`.

## Usage

Requiring the package creates two commmands: `:GmailOpen` and `:GmailSendEmail`.

### :GmailOpen

Opens a non-modifiable buffer with the last 20 messages from the main Gmail inbox.
Pressing `Enter` on an Email opens the raw Email content in a new buffer.

### :GmailSendEmail

Opens an editable buffer that allows for writing the body of an email. In order
to send the email, just press `cc` in Normal mode. After that, the plugin will
ask for a "subject" and a "to" address. After a successfully sent email, the
buffer will be closed automatically.

## Final Comments

As indicated at the beginning, the plugin mainly served as an exploration of
the Neovim API and its interfacing with Rust. However, it can serve as a quick
tool for sending emails from Neovim with all the benefits of not having to leave
the editor.
