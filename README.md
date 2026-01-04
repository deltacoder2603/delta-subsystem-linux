# DSL - Delta Subsystem for Linux

**Minimal. Isolated. Fast.**

DSL is a lightweight containerization tool that provides isolated Linux environments using Linux namespaces, cgroups, and chroot. It allows you to run Linux distributions in isolated containers with configurable resource limits.

## Features

- 🚀 **Lightweight**: Minimal overhead, fast startup
- 🔒 **Isolated**: Uses Linux namespaces for process, mount, and UTS isolation
- 📊 **Resource Limits**: Configure memory, CPU, and process limits via cgroups
- 🐧 **Multi-Distro**: Run different Linux distributions in isolated environments
- ⚙️ **Configurable**: TOML-based configuration file support

## Prerequisites

DSL requires Linux kernel features (namespaces, cgroups) and cannot run natively on Windows or macOS. See platform-specific instructions below.

### Linux Requirements

- Linux kernel 4.0+ (for cgroups v2 support)
- Rust toolchain (1.70+)
- Root/sudo privileges (for namespace and cgroup operations)
- A Linux distribution rootfs in the `distros/` directory

## Installation

**Note**: The `dsl` executable binary is already included in the project folder. You can use it directly without building if you're on a compatible Linux system. However, if you need to rebuild or modify the code, follow the build instructions below.

### Linux

#### Quick Start (Using Pre-built Binary)

If the `dsl` executable binary is already in the project folder, you can use it directly:

```bash
# Make sure it's executable
chmod +x dsl

# Run it directly
sudo ./dsl run ubuntu
```

#### Building from Source

#### 1. Install Rust

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 2. Clone and Build

```bash
git clone <repository-url>
cd dsl
cargo build --release
```

#### 3. Using the Executable Binary

The `dsl` executable binary is already available in the project root directory. You can use it in several ways:

**Option A: Run directly from the project directory (Recommended if binary exists)**

```bash
# Ensure the binary is executable
chmod +x dsl

# Run it
sudo ./dsl run ubuntu
```

**Option B: Install system-wide**

```bash
sudo cp dsl /usr/local/bin/
sudo chmod +x /usr/local/bin/dsl
```

Then you can run `dsl` from anywhere:

```bash
sudo dsl run ubuntu
```

**Option C: Add project directory to PATH**

```bash
export PATH=$PATH:$(pwd)
```

Then run:

```bash
sudo dsl run ubuntu
```

**Note**: If you build from source, the binary will also be available at `target/release/dsl`. You can use either location.

### macOS

DSL cannot run natively on macOS. Use one of these options:

#### Option 1: WSL2 (Recommended for Development)

1. Install [Docker Desktop](https://www.docker.com/products/docker-desktop) or use a Linux VM
2. Build inside a Linux container:

```bash
docker run -it --rm -v $(pwd):/workspace -w /workspace rust:latest bash
cd /workspace
cargo build --release
```

#### Option 2: Linux VM

1. Install [VirtualBox](https://www.virtualbox.org/) or [VMware Fusion](https://www.vmware.com/products/fusion.html)
2. Install a Linux distribution (Ubuntu recommended)
3. Follow the Linux installation instructions inside the VM

#### Option 3: GitHub Codespaces / Remote Linux Server

Use a remote Linux environment for development and testing.

### Windows

DSL cannot run natively on Windows. Use one of these options:

#### Option 1: WSL2 (Recommended)

1. **Install WSL2**:

```powershell
wsl --install
```

Or manually:

```powershell
# Enable WSL and Virtual Machine Platform
dism.exe /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux /all /norestart
dism.exe /online /enable-feature /featurename:VirtualMachinePlatform /all /norestart

# Restart your computer, then set WSL2 as default
wsl --set-default-version 2

# Install a Linux distribution from Microsoft Store (Ubuntu recommended)
```

2. **Open WSL2 terminal** and follow Linux installation instructions:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Navigate to your project (mounted at /mnt/c/Users/YourUsername/...)
cd /mnt/c/path/to/dsl

# Build
cargo build --release
```

**Note**: WSL2 uses cgroups v1 by default. You may need to enable cgroups v2:

```bash
# Edit /etc/default/grub (if using GRUB)
sudo nano /etc/default/grub
# Add: GRUB_CMDLINE_LINUX="systemd.unified_cgroup_hierarchy=1"
sudo update-grub
sudo reboot
```

#### Option 2: Docker Desktop

1. Install [Docker Desktop for Windows](https://www.docker.com/products/docker-desktop)
2. Build inside a Linux container:

```powershell
docker run -it --rm -v ${PWD}:/workspace -w /workspace rust:latest bash
cd /workspace
cargo build --release
```

#### Option 3: Linux VM

1. Install [VirtualBox](https://www.virtualbox.org/) or [Hyper-V](https://docs.microsoft.com/en-us/virtualization/hyper-v-on-windows/)
2. Install a Linux distribution (Ubuntu recommended)
3. Follow the Linux installation instructions inside the VM

## Usage

The `dsl` executable binary is the main entry point for the DSL tool. The binary is already included in the project root directory, so you can run it directly with `./dsl` or install it system-wide for easier access.

### Basic Usage

The `dsl` executable binary is located in the project root directory. You can use it in the following ways:

**If installed system-wide:**

```bash
dsl run <distro> [options]
```

**If running from the project directory:**

```bash
sudo ./dsl run <distro> [options]
```

**If using the built binary from target/release:**

```bash
sudo ./target/release/dsl run <distro> [options]
```

### Options

- `--memory <size>`: Memory limit (e.g., `256M`, `1G`, `512M`)
- `--cpu <n>`: CPU quota in cores (e.g., `1`, `2`, `0.5`)
- `--pids <n>`: Maximum number of processes

### Examples

#### Run Ubuntu with default settings

Using the binary from the project directory:

```bash
sudo ./dsl run ubuntu
```

Or if installed system-wide:

```bash
sudo dsl run ubuntu
```

#### Run with resource limits

```bash
sudo ./dsl run ubuntu --memory 512M --cpu 1 --pids 64
```

#### Run Alpine with strict limits

```bash
sudo ./dsl run alpine --memory 256M --cpu 0.5 --pids 32
```

### Configuration File

You can also configure DSL using a `dsl.toml` file in your project root:

```toml
[dsl]
distro = "ubuntu"

[limits]
memory = "1G"
cpu = 2
pids = 128
```

Command-line options override configuration file settings.

## Setting Up Distribution Rootfs

DSL requires a rootfs for each distribution you want to run. The rootfs should be located at:

```
distros/<distro-name>/rootfs/
```

### Example: Setting up Ubuntu rootfs

```bash
# Create directory structure
mkdir -p distros/ubuntu/rootfs

# Use debootstrap to create a minimal Ubuntu rootfs
sudo debootstrap focal distros/ubuntu/rootfs

# Or use a pre-built rootfs
# Download and extract a minimal rootfs tarball
```

### Example: Setting up Alpine rootfs

```bash
mkdir -p distros/alpine/rootfs
cd distros/alpine/rootfs

# Download and extract Alpine minirootfs
wget https://dl-cdn.alpinelinux.org/alpine/v3.18/releases/x86_64/alpine-minirootfs-3.18.0-x86_64.tar.gz
sudo tar xzf alpine-minirootfs-3.18.0-x86_64.tar.gz
sudo rm alpine-minirootfs-3.18.0-x86_64.tar.gz
```

## Project Structure

```
dsl/
├── Cargo.toml          # Rust project configuration
├── dsl.toml            # DSL configuration file
├── dsl                 # Executable binary (compiled output)
├── src/
│   ├── main.rs         # Entry point
│   ├── cli.rs          # Command-line interface
│   ├── runtime.rs      # Runtime orchestration
│   ├── namespace.rs    # Linux namespace setup
│   ├── cgroup.rs       # cgroup resource limits
│   ├── filesystem.rs   # Filesystem/chroot setup
│   └── init.rs         # Container initialization
├── target/             # Build artifacts
│   └── release/
│       └── dsl         # Release executable binary
└── distros/            # Distribution rootfs files
    └── <distro-name>/
        └── rootfs/
```

**Note**: The `dsl` executable binary is the compiled Rust program. After building with `cargo build --release`, the binary will be available at `target/release/dsl`. You can copy it to the project root or install it system-wide for easier access.

## Requirements

- **Linux Kernel**: 4.0+ (cgroups v2 support recommended)
- **Rust**: 1.70 or later
- **Privileges**: Root or sudo access required
- **cgroups**: Must be mounted at `/sys/fs/cgroup`

## Troubleshooting

### "namespace setup failed"

- Ensure you're running with root/sudo privileges
- Check that your kernel supports namespaces: `cat /proc/self/ns/pid`

### "cgroup setup failed"

- Verify cgroups are mounted: `mount | grep cgroup`
- For cgroups v2, ensure unified hierarchy is enabled
- Check permissions on `/sys/fs/cgroup`

### "rootfs not found"

- Ensure the distribution rootfs exists at `distros/<distro>/rootfs/`
- Verify the rootfs directory contains a valid Linux filesystem
- Check that you're running DSL from the project root directory

### "chroot failed"

- Ensure you have root/sudo privileges
- Verify the rootfs path is correct and accessible
- Check that the rootfs contains necessary directories (`/bin`, `/lib`, etc.)

### WSL2 Issues

- WSL2 may require additional configuration for cgroups
- Some namespace features may be limited in WSL2
- Consider using a full Linux VM for production use

## Development

### Building

```bash
cargo build
```

### Building Release

```bash
cargo build --release
```

This will create the `dsl` executable binary at `target/release/dsl`. You can then:
- Run it directly: `sudo ./target/release/dsl run ubuntu`
- Copy it to project root: `cp target/release/dsl ./dsl`
- Install it system-wide: `sudo cp target/release/dsl /usr/local/bin/`

### Running Tests

```bash
cargo test
```

## Security Considerations

- DSL requires root privileges to create namespaces and cgroups
- Only run trusted distributions and rootfs images
- Resource limits help prevent resource exhaustion attacks
- Isolated namespaces provide process isolation but are not a complete security boundary

## License

[Add your license information here]

## Contributing

[Add contribution guidelines here]

## Acknowledgments

DSL uses the following Rust crates:
- `nix` - Unix system call interface
- `toml` - TOML configuration parsing
- `serde` - Serialization framework

