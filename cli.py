import argparse
import os
from pathlib import Path
import subprocess

parser = argparse.ArgumentParser(description='A cli for processing tauri')
# mode dev / build
parser.add_argument('mode', choices=['dev', 'build'], help='dev or build')
parser.add_argument('--release', action='store_true', help='build in release mode')
WINDOWS = os.name == 'nt'
args = parser.parse_args()
release = args.release
target = 'i686-pc-windows-' + ('msvc' if WINDOWS else 'gnu')

def build_vcmp_core():
    # first add i686-pc-windows-msvc / i686-pc-windows-gnu 
    subprocess.run(['cargo', 'build', '--target', target, '--release', '--package', 'vcmp_core'])

    # cp target/i686-pc-windows-msvc/release/vcmp_core.exe to build/vcmp_core.exe

    cp_build(Path(target) / 'release' / 'vcmp_core.exe')

def cp_build(src: Path):
    src = Path('target') / src
    dst = Path('build') / src.name
    dst.parent.mkdir(parents=True, exist_ok=True)
    with open(src, 'rb') as w, open(dst, 'wb') as r:
        r.write(w.read())


def build_library_redirector():
    subprocess.run(['cargo', 'build', '--target', target, '--release', '--package', 'library_redirector'])
    # dll
    cp_build(Path(target) / 'release' / 'library_redirector.dll')

def dev_tauri():
    subprocess.run(['yarn', 'tauri', 'dev'], shell=True)

    
if __name__ == '__main__':
    subprocess.run(['rustup', 'target', 'add', target])
    build_vcmp_core()
    build_library_redirector()
    if args.mode == 'dev':
        dev_tauri()
    # elif args.mode == 'build':
    #     build_vcmp_core()
    #     subprocess.run(['yarn', 'tauri', 'build', '--distDir', 'build'])
    
