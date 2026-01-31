import argparse
import os
from pathlib import Path
import subprocess
from typing import Optional

parser = argparse.ArgumentParser(description='A cli for processing tauri')
# mode dev / build
parser.add_argument('mode', choices=['dev', 'build'], help='dev or build')
parser.add_argument('--release', action='store_true', help='build in release mode')
WINDOWS = os.name == 'nt'
args = parser.parse_args()
release = args.release
target = 'i686-pc-windows-' + ('msvc' if WINDOWS else 'gnu')
#windows7_target = 'i686-win7-windows-msvc'

def build_vcmp_core():
    # first add i686-pc-windows-msvc / i686-pc-windows-gnu 
    subprocess.run(['cargo', '+stable', 'build', '--target', target, '--release', '--package', 'vcmp_core'])
    # win7
    # subprocess.run(['cargo', '+stable', 'build', '--target', windows7_target, '--release', '--package', 'vcmp_core'])

    # cp target/i686-pc-windows-msvc/release/vcmp_core.exe to build/vcmp_core.exe

    cp_build(Path(target) / 'release' / 'vcmp_core.exe')
    # cp_build(Path(windows7_target) / 'release' / 'vcmp_core.exe', 'vcmp_core_win7.exe')

def cp_build(src: Path, rename: Optional[str] = None):
    src = Path('target') / src
    dst = Path('build') / (src.name if rename is None else rename)
    dst.parent.mkdir(parents=True, exist_ok=True)
    with open(src, 'rb') as w, open(dst, 'wb') as r:
        r.write(w.read())


def dev_tauri():
    subprocess.run(['yarn', 'tauri', 'dev'], shell=True)

    
if __name__ == '__main__':
    subprocess.run(['rustup', '+stable', 'target', 'add', target])
    # subprocess.run(['rustup', '+stable', 'target', 'add', windows7_target])
    build_vcmp_core()
    if args.mode == 'dev':
        dev_tauri()
    elif args.mode == 'build':
        tauri_cmd = ['yarn', 'tauri', 'build']
        if release:
            tauri_cmd.append('--release')
        subprocess.run(tauri_cmd, shell=True)
    
