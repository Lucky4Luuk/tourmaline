#! /bin/sh
#
# This script will be executed by `cargo run`.
# If this script is not saved with LF line-endings, it will break!

################################################################################
# Config
################################################################################
RUN_AS_CMD=true
LIMINE_GIT_URL="https://github.com/limine-bootloader/limine.git"

################################################################################
# The script itself
################################################################################

cd ..

# Cargo passes the path to the built executable as the first argument.
KERNEL=$1
echo "Kernel path: $KERNEL"

# Clone the `limine` repository if we don't have it yet.
if [ ! -d target/limine ]; then
    git clone $LIMINE_GIT_URL --depth=1 --branch v3.0-branch-binary target/limine
fi

# Make sure we have an up-to-date version of the bootloader.
cd target/limine
git fetch
make
cd ..
cd ..

echo "$PWD"

# Copy the needed files into an ISO image.
mkdir -p target/iso_root
cp $KERNEL kernel/conf/limine.cfg target/limine/limine.sys target/limine/limine-cd.bin target/limine/limine-cd-efi.bin target/iso_root

xorriso -as mkisofs                                             \
    -b limine-cd.bin                                            \
    -no-emul-boot -boot-load-size 4 -boot-info-table            \
    --efi-boot limine-cd-efi.bin                                \
    -efi-boot-part --efi-boot-image --protective-msdos-label    \
    target/iso_root -o $KERNEL.iso

# For the image to be bootable on BIOS systems, we must run `limine-deploy` on it.
target/limine/limine-deploy $KERNEL.iso

echo "Kernel built! Attempting to start with qemu..."
echo "If starting qemu fails on WSL, please set the RUN_AS_CMD flag at the top of this script."

# Run the created image with QEMU.
if [ "$RUN_AS_CMD" = false ]; then
    qemu-system-x86_64 \
        -machine q35 -cpu qemu64 -M smm=off -smp 4 \
        -D target/log.txt -d int,guest_errors -no-reboot -no-shutdown \
        -serial stdio \
        $KERNEL.iso
fi
if [ "$RUN_AS_CMD" = true ]; then
    WIN_PWD=`wslpath -w "$(pwd)"`
    WIN_KERNEL=`wslpath -w "$KERNEL.iso"`
    cmd.exe /c "pushd ${WIN_PWD} && qemu-system-x86_64 \
        -machine q35 -cpu qemu64 -M smm=off -smp 4 \
        -D target/log.txt -d int,guest_errors -no-reboot -no-shutdown \
        -serial stdio \
        ${WIN_KERNEL}"
fi
