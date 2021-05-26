# Setup

## Step 1: Install Rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For those of you, who are skeptical, please read the script first! And can customize your specific environment based on the script directly.

> Accordingly, it is customary for Rust developers to include this directory in their PATH environment variable. During installation rustup will attempt to configure the PATH. Because of differences between platforms, command shells, and bugs in rustup, the modifications to PATH may not take effect until the console is restarted, or the user is logged out, or it may not succeed at all.

### Step 2

Verify the `CARGO_HOME`.

```bash
echo $CARGO_HOME
```

Your `CARGO_HOME` is where you would be storing and managing any third-party source code.

#### Step 2.1

In case, you don't the `CARGO_HOME` set, then set the variable to the full-path to somewhere in your file-system.

### For *Nix

Add to your shell initialization scripts (`~/.bashrc` or `~/.bash_profile` or `~/.zshrc`) the following...

```bash
# set CARGO_HOME & PATH
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"
```

P.S. You can check which shell you are using by running `echo $SHELL`.

### For Windows

You would have to Edit your "Environment Variables". Refer: [How to set environment variables](https://support.shotgunsoftware.com/hc/en-us/articles/114094235653-Setting-global-environment-variables-on-Windows#:~:text=Windows%207,to%20edit%2C%20and%20click%20Edit.)

Ensure `%CARGO_HOME%` is set and `%CARGO_HOME%\bin` is added to the PATH.

> Restart your `git bash` and verify the steps 1 and 2 again.

## Machine Requirements

- Minimum Specs
  - 8 GB RAM
  - 4-core i5 or higher
- Any OS (MacOS, Windows or Linux)
