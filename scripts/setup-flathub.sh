#!/usr/bin/env bash

. "$(dirname "$0")/bash-guard.sh"

set -euo pipefail

sudo apt-get update
sudo apt-get install -y flatpak flatpak-builder
flatpak remote-add --if-not-exists --user flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install --user --noninteractive flathub org.gnome.Platform//47 org.gnome.Sdk//47
flatpak install --user --noninteractive flathub org.freedesktop.Sdk.Extension.rust-stable//24.08
flatpak install --user --noninteractive flathub org.freedesktop.Sdk.Extension.node22//24.08
