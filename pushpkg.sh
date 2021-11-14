#!/bin/bash
msg=$(cargo run -- --version)
cp PKGBUILD ../packages/yearsfx/
cd ../packages/yearsfx
makepkg --printsrcinfo > .SRCINFO
makepkg -g >> PKGBUILD
git add PKGBUILD .SRCINFO
git commit -m "$msg"
git push
