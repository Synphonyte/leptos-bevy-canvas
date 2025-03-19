import re
import sys
from datetime import datetime


def main():
    with open("Cargo.toml", "r") as f:
        cargo_text = f.read()

        m = re.search(r'leptos = "(\d+)\.(\d+)', cargo_text)
        leptos_version = f"{m.group(1)}.{m.group(2)}"

        m = re.search(r'bevy = \{ version = "(\d+)\.(\d+)', cargo_text)
        bevy_version = f"{m.group(1)}.{m.group(2)}"

        m = re.search(r'version = "(\d+)\.(\d+)\.(\d+)', cargo_text)
        crate_version_short = f"{m.group(1)}.{m.group(2)}"
        crate_version_long = f"{m.group(1)}.{m.group(2)}.{m.group(3)}"

    print("Found crate version", crate_version_short, ", leptos version", leptos_version, "and bevy version",
          bevy_version)

    with open("README.md", "r") as f:
        original_text = f.read()

        text = check_compat_table(leptos_version, bevy_version, crate_version_short, original_text)

        if check_compat_table(leptos_version, bevy_version, crate_version_short, original_text):
            print("[OK] README.md does contain the current crate version in the compatibility table")
        else:
            print("[Failed] README.md doesn't contain the current crate version in the compatibility table",
                  file=sys.stderr)
            quit(1)

    with open("CHANGELOG.md", "r") as f:
        original_text = f.read()

        if check_in_changelog(original_text):
            print("[Failed] CHANGELOG.md still contains an [Unreleased] header",
                  file=sys.stderr)
            quit(1)
        else:
            print("[OK] CHANGELOG.md doesn't contain an [Unreleased] header")


def check_compat_table(leptos_version: str, bevy_version: str, crate_version: str, original_text: str):
    lines = original_text.splitlines()

    for line in lines:
        if re.search(rf"^\| (.* )?{crate_version}\s*\| {leptos_version}\s*\| {bevy_version}", line) is not None:
            return True

    return False


def check_in_changelog(original_text: str):
    return "## [Unreleased]" in original_text


if __name__ == '__main__':
    main()
