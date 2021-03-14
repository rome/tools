@echo off

set ROME_DEV_VENDOR_BUNDLING=1
node scripts/vendor/rome.cjs run scripts/%*
